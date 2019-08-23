// wengwengweng

use std::io::Write;
use std::io::Read;
use std::net::TcpStream;

use url::Url;
#[cfg(not(target_arch = "wasm32"))]
use native_tls::TlsConnector;

use crate::Error;
use crate::Result;

use super::*;

const RESPONSE_BUF_SIZE: usize = 1024;

#[derive(Clone)]
pub struct Request {
	method: Method,
	scheme: Scheme,
	version: Version,
	host: String,
	path: String,
	port: u16,
	headers: HeaderMap,
	body: Vec<u8>,
}

impl Request {

	pub fn from_raw(buf: &[u8]) -> Result<Self> {

		let mut headers = [httparse::EMPTY_HEADER; 128];
		let mut req = httparse::Request::new(&mut headers);

		let body_pos = match req.parse(&buf)? {
			httparse::Status::Complete(len) => len,
			httparse::Status::Partial => return Err(Error::Net),
		};

		let method = req.method.ok_or(Error::Net)?.parse::<Method>()?;
		let path = req.path.ok_or(Error::Net)?;
		let version = req.version.ok_or(Error::Net)?;
		let body = &buf[body_pos..];

		return Ok(Self {
			method: method,
			version: version.into(),
			scheme: Scheme::HTTP,
			host: String::new(),
			path: path.to_owned(),
			port: 80,
			headers: HeaderMap::new(),
			body: body.to_owned(),
		});

	}

	pub fn from_url(method: Method, url: &str) -> Result<Self> {

		let url = Url::parse(url)?;
		let scheme = url.scheme().parse::<Scheme>()?;
		let host = url.host_str().ok_or(Error::Net)?;
		let path = url.path();
		let mut headers = HeaderMap::new();

		headers.set(Header::Host, host);

		return Ok(Self {
			method: method,
			version: Version::V10,
			scheme: scheme,
			host: host.to_owned(),
			path: path.to_owned(),
			port: scheme.port(),
			headers: headers,
			body: Vec::new(),
		});

	}

	pub fn get(url: &str) -> Result<Self> {
		return Self::from_url(Method::GET, url);
	}

	pub fn post(url: &str) -> Result<Self> {
		return Self::from_url(Method::POST, url);
	}

	pub fn port(&self) -> u16 {
		return self.port;
	}

	pub fn host(&self) -> &str {
		return &self.host;
	}

	pub fn path(&self) -> &str {
		return &self.path;
	}

	pub fn scheme(&self) -> Scheme {
		return self.scheme;
	}

	pub fn method(&self) -> Method {
		return self.method;
	}

	pub fn version(&self) -> Version {
		return self.version;
	}

	pub fn headers(&self) -> &HeaderMap {
		return &self.headers;
	}

	pub fn set_header(&mut self, key: Header, value: &str) {
		self.headers.set(key, value);
	}

	pub fn body(&mut self, data: impl AsRef<[u8]>) {
		self.body = data.as_ref().to_owned();
	}

	pub fn message(&self) -> Vec<u8> {

		let mut m = Vec::new();

		m.extend_from_slice(&format!("{} {} {}", self.method().to_string(), self.path(), self.version().to_string()).as_bytes());
		m.extend_from_slice("\r\n".as_bytes());
		m.extend_from_slice(&self.headers.to_string().as_bytes());
		m.extend_from_slice("\r\n".as_bytes());
		m.extend_from_slice(&self.body);

		return m;

	}

	pub fn send(&mut self, data: Option<&[u8]>) -> Result<Response> {

		if let Some(data) = data {
			self.body(data);
		}

		let mut stream = TcpStream::connect((self.host(), self.port()))?;
		let mut buf = Vec::with_capacity(RESPONSE_BUF_SIZE);

		match self.scheme() {

			Scheme::HTTP => {

				stream.write_all(&self.message())?;
				stream.read_to_end(&mut buf)?;

			},

			Scheme::HTTPS => {

				#[cfg(not(target_arch = "wasm32"))] {

					let connector = TlsConnector::new()?;
					let mut stream = connector.connect(self.host(), stream)?;

					stream.write_all(&self.message())?;
					stream.read_to_end(&mut buf)?;

				}

			},

		};

		return Response::from_raw(&buf);

	}

}

