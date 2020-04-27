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

impl Read for Stream {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		match self {
			Stream::HTTP(sock) => return sock.read(buf),
			Stream::HTTPS(stream) => {
				return match stream.read(buf) {
					Ok(size) => Ok(size),
					Err(e) => {
						if e.kind() == std::io::ErrorKind::ConnectionAborted {
							return Ok(0);
						}
						return Err(e);
					}
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

pub(super) fn send(req: Request) -> Result<Response> {

	let scheme = req.scheme();
	let host = req.host().to_string();
	let port = req.port();
	let sock = TcpStream::connect((host.as_ref(), port))
		.map_err(|_| format!("failed to connect to tcp stream"))?;

	let mut buf = Vec::with_capacity(1024);
	let msg = req.into_msg();

	let mut stream = match scheme {

		Scheme::HTTP => Stream::HTTP(sock),
		Scheme::HTTPS => {

			let sni = webpki::DNSNameRef::try_from_ascii_str(&host).map_err(|_| format!("webpki failure"))?;
			let sess = rustls::ClientSession::new(&TLS_CONF, sni);

			Stream::HTTPS(rustls::StreamOwned::new(sess, sock))

		}
	};

	stream.write_all(&msg)
		.map_err(|_| format!("failed to write to stream"))?;
	stream.read_to_end(&mut buf)
		.map_err(|_| format!("failed to read from stream"))?;

	return Response::from(&buf);

}

macro_rules! gen_method {
	($fname:ident => $method:expr) => {
		pub fn $fname(url: &str) -> Result<Request> {
			return Request::new($method, url);
		}
	}
}

gen_method!(get => Method::GET);
gen_method!(post => Method::GET);
gen_method!(put => Method::PUT);
gen_method!(delete => Method::DELETE);

