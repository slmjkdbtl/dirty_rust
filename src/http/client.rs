// wengwengweng

use std::io::Write;
use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::Result;
use super::*;

static TLS_CONF: Lazy<Arc<rustls::ClientConfig>> = Lazy::new(|| {

	let mut config = rustls::ClientConfig::new();

	config
		.root_store
		.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

	return Arc::new(config);

});

enum Stream {
	HTTP(TcpStream),
	HTTPS(rustls::StreamOwned<rustls::ClientSession, TcpStream>),
}

fn is_close_notify(e: &std::io::Error) -> bool {

	if e.kind() != std::io::ErrorKind::ConnectionAborted {
		return false;
	}

	if let Some(msg) = e.get_ref() {
		// TODO: mess?
		return msg.description().contains("CloseNotify");
	}

	return false
}

impl Read for Stream {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		match self {
			Stream::HTTP(sock) => sock.read(buf),
			Stream::HTTPS(stream) => {
				return match stream.read(buf) {
					Ok(size) => Ok(size),
					Err(ref e) if is_close_notify(e) => Ok(0),
					Err(e) => Err(e),
				}
			},
		}
	}
}

impl Write for Stream {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		match self {
			Stream::HTTP(sock) => sock.write(buf),
			Stream::HTTPS(stream) => stream.write(buf),
		}
	}
	fn flush(&mut self) -> std::io::Result<()> {
		match self {
			Stream::HTTP(sock) => sock.flush(),
			Stream::HTTPS(stream) => stream.flush(),
		}
	}
}

pub fn send(mut req: Request, data: Option<Body>) -> Result<Response> {

	if let Some(data) = data {
		req.set_body(data);
	}

	let stream = TcpStream::connect((req.host(), req.port()))
		.map_err(|_| format!("failed to connect to tcp stream"))?;

	let mut buf = Vec::with_capacity(1024);
	let msg = req.message();

	let mut stream = match req.scheme() {

		Scheme::HTTP => Stream::HTTP(stream),
		Scheme::HTTPS => {

			let sni = webpki::DNSNameRef::try_from_ascii_str(req.host()).unwrap();
			let sess = rustls::ClientSession::new(&TLS_CONF, sni);

			Stream::HTTPS(rustls::StreamOwned::new(sess, stream))

		}
	};

	stream.write_all(&msg)
		.map_err(|_| format!("failed to write to stream"))?;
	stream.read_to_end(&mut buf)
		.map_err(|_| format!("failed to read from stream"))?;

	return Response::parse(&buf);

}

pub fn get(url: &str) -> Result<Response> {
	return send(Request::from_url(Method::GET, url)?, None);
}

pub fn post(url: &str, body: Body) -> Result<Response> {
	return send(Request::from_url(Method::POST, url)?, Some(body));
}

