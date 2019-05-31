// wengwengweng

//! HTTP Requests & Servers

use std::io::Write;
use std::io::Read;
use std::net::TcpStream;
use std::str::FromStr;
use std::collections::HashMap;

use url::Url;
use native_tls::TlsConnector;

use crate::Error;
use crate::Result;

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;
const RESPONSE_BUF_SIZE: usize = 1024;

#[derive(Clone, Copy)]
pub enum Scheme {
	HTTP,
	HTTPS,
}

impl Scheme {
	pub fn port(&self) -> u16 {
		return match self {
			Scheme::HTTP => HTTP_PORT,
			Scheme::HTTPS => HTTPS_PORT,
		};
	}
}

impl FromStr for Scheme {

	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		return match s {
			"http" => Ok(Scheme::HTTP),
			"https" => Ok(Scheme::HTTPS),
			_ => Err(Error::Net),
		};
	}

}

#[derive(Clone, Copy)]
pub enum Method {
	GET,
	POST,
}

impl ToString for Method {
	fn to_string(&self) -> String {
		return match self {
			Method::GET => String::from("GET"),
			Method::POST => String::from("POST"),
		};
	}
}

#[derive(Clone, Copy)]
pub enum Version {
	V10,
	V11,
	V20,
}

impl ToString for Version {
	fn to_string(&self) -> String {
		return match self {
			Version::V10 => String::from("HTTP/1.0"),
			Version::V11 => String::from("HTTP/1.1"),
			Version::V20 => String::from("HTTP/2.0"),
		};
	}
}

pub struct Request {
	method: Method,
	scheme: Scheme,
	version: Version,
	host: String,
	path: String,
	port: u16,
	headers: HashMap<String, String>,
	body: Option<Vec<u8>>,
}

pub struct Response {
	body: Vec<u8>,
	code: u16,
	headers: HashMap<String, String>,
}

impl Response {

	pub fn from_raw(buf: &[u8]) -> Result<Self> {

		let mut headers = [httparse::EMPTY_HEADER; 128];
		let mut res = httparse::Response::new(&mut headers);
		let status = res.parse(&buf)?;

		let body_pos = match status {
			httparse::Status::Complete(len) => len,
			httparse::Status::Partial => return Err(Error::Net),
		};

		let body = &buf[body_pos..];

		return Ok(Self {
			body: body.to_vec(),
			code: res.code.ok_or(Error::Net)?,
			headers: HashMap::new(),
		});

	}

	pub fn bytes(&self) -> &[u8] {
		return &self.body;
	}

	pub fn text(&self) -> String {
		return String::from_utf8_lossy(self.bytes()).to_owned().to_string();
	}

	pub fn code(&self) -> u16 {
		return self.code;
	}

}

impl Request {

	pub fn new(method: Method, url: &str) -> Result<Self> {

		let url = Url::parse(url)?;
		let scheme = url.scheme().parse::<Scheme>()?;
		let host = url.host_str().ok_or(Error::Net)?;
		let path = url.path();

		return Ok(Self {
			method: method,
			version: Version::V10,
			scheme: scheme,
			host: host.to_owned(),
			path: path.to_owned(),
			port: scheme.port(),
			headers: HashMap::new(),
			body: None,
		});

	}

	pub fn get(url: &str) -> Result<Self> {
		return Self::new(Method::GET, url);
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

	pub fn headers(&self) -> &HashMap<String, String> {
		return &self.headers;
	}

	pub fn add_header(&mut self, key: &str, value: &str) {
		self.headers.insert(key.to_owned(), value.to_owned());
	}

	pub fn body(&mut self, data: &[u8]) {
		self.body = Some(data.to_vec());
	}

	pub fn message(&self) -> Vec<u8> {

		let mut m = String::new();

		m.push_str(&format!("{} {} {}", self.method().to_string(), self.path(), self.version().to_string()));
		m.push_str("\r\n");
		m.push_str(&format!("Host: {}", self.host()));
		m.push_str("\r\n");

		for (key, val) in self.headers() {
			m.push_str(&format!("{}: {}", key, val));
			m.push_str("\r\n");
		}

		m.push_str("\r\n");

		let mut bytes = m.as_bytes().to_vec();

		if let Some(mut body) = self.body.clone() {
			bytes.append(&mut body);
		}

		return bytes;

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

				let connector = TlsConnector::new()?;
				let mut stream = connector.connect(self.host(), stream)?;

				stream.write_all(&self.message())?;
				stream.read_to_end(&mut buf)?;

			},

		};

		return Response::from_raw(&buf);

	}

}

pub fn get(url: &str) -> Result<Response> {
	return Request::get(url)?.send(None);
}

pub fn post(url: &str, data: &[u8]) -> Result<Response> {
	return Request::get(url)?.send(Some(data));
}

