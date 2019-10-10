// wengwengweng

use std::io::Write;
use std::io::Read;
use std::net::TcpStream;

#[cfg(all(not(mobile), not(web)))]
use native_tls::TlsConnector;

use crate::Result;
use super::*;

pub fn send(mut req: Request, data: Option<&[u8]>) -> Result<Response> {

	if let Some(data) = data {
		req.set_body(data);
	}

	let mut stream = TcpStream::connect((req.host(), req.port()))?;
	let mut buf = Vec::with_capacity(1024);
	let msg = req.message();

	match req.scheme() {

		Scheme::HTTP => {

			stream.write_all(&msg)?;
			stream.read_to_end(&mut buf)?;

		},

		Scheme::HTTPS => {

			#[cfg(all(not(mobile), not(web)))] {

				let connector = TlsConnector::new()?;
				let mut stream = connector.connect(req.host(), stream)?;

				stream.write_all(&msg)?;
				stream.read_to_end(&mut buf)?;

			}

		},

	};

	return Response::from_raw(&buf);

}

pub fn get(url: &str) -> Result<Response> {
	return send(Request::from_url(Method::GET, url)?, None);
}

pub fn post(url: &str, data: impl AsRef<[u8]>) -> Result<Response> {
	return send(Request::from_url(Method::POST, url)?, Some(data.as_ref()));
}

