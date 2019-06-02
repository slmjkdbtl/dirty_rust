// wengwengweng

//! HTTP Requests & Servers

use std::io::Write;
use std::io::Read;
use std::net::TcpStream;
use std::net::TcpListener;
use std::str::FromStr;
use std::path::Path;
use std::collections::HashMap;

use url::Url;
use native_tls::TlsConnector;

use crate::Error;
use crate::Result;

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;
const RESPONSE_BUF_SIZE: usize = 1024;

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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

impl FromStr for Method {

	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		return match s {
			"GET" => Ok(Method::GET),
			"POST" => Ok(Method::POST),
			_ => Err(Error::Net),
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

impl From<u8> for Version {
	fn from(v: u8) -> Self {
		return match v {
			1 => Version::V10,
			11 => Version::V11,
			2 => Version::V20,
			_ => Version::V10,
		};
	}
}

impl FromStr for Version {

	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		return match s {
			"HTTP/1.0" => Ok(Version::V10),
			"HTTP/1.1" => Ok(Version::V11),
			"HTTP/2.0" => Ok(Version::V20),
			_ => Err(Error::Net),
		};
	}

}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub enum Handle {
	GET(String),
	POST(String),
}

pub struct Server {
	location: String,
	port: u16,
	handlers: Vec<Box<Fn(&Request) -> Option<Response>>>,
}

unsafe impl Send for Server {}

impl Server {

	pub fn new(loc: &str, port: u16) -> Self {
		return Self {
			location: loc.to_owned(),
			port: port,
			handlers: Vec::new(),
		};
	}

	pub fn handle<F: Fn(&Request) -> Option<Response> + 'static>(&mut self, f: F) {
		self.handlers.push(Box::new(f));
	}

	pub fn get<T: AsRef<[u8]>, F: Fn() -> T + 'static>(&mut self, path: &str, f: F) {

		let path = path.to_owned();

		self.handle(move |req| {
			if req.method() == Method::GET && req.path() == path {
				return Some(Response::success(f()));
			} else {
				return None;
			}
		});

	}

	pub fn statics(&mut self, path: &str, folder: impl AsRef<Path>) {

		let path = path.to_owned();
		let folder = folder.as_ref().to_owned();

		self.handle(move |req| {

			let path = req.path();

			return None;

		});

	}

	pub fn serve(&self) -> Result<()> {

		let listener = TcpListener::bind((&self.location[..], self.port)).unwrap();

		for stream in listener.incoming() {

			let mut stream = stream?;
			let mut buf = [0; 512];

			stream.read(&mut buf)?;

			let req = Request::from_raw(&buf)?;

			for handler in &self.handlers {
				if let Some(res) = handler(&req) {
					stream.write_all(&res.message())?;
				}
			}

		}

		return Ok(());

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
	body: Vec<u8>,
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

		let body_pos = match res.parse(&buf)? {
			httparse::Status::Complete(len) => len,
			httparse::Status::Partial => return Err(Error::Net),
		};

		let body = &buf[body_pos..];

		return Ok(Self {
			body: body.to_owned(),
			code: res.code.ok_or(Error::Net)?,
			headers: HashMap::new(),
		});

	}

	pub fn success(body: impl AsRef<[u8]>) -> Self {
		return Self {
			body: body.as_ref().to_owned(),
			code: 200,
			headers: HashMap::new(),
		};
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

	pub fn message(&self) -> Vec<u8> {

		let mut m = String::new();

		m.push_str("HTTP/1.1 200 OK");
		m.push_str("\r\n");
		m.push_str("\r\n");

		let mut bytes = m.as_bytes().to_owned();

		bytes.append(&mut self.body.clone());

		return bytes;

	}

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
			headers: HashMap::new(),
			body: body.to_owned(),
		});

	}

	pub fn from_url(method: Method, url: &str) -> Result<Self> {

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
			body: Vec::new(),
		});

	}

	pub fn get(url: &str) -> Result<Self> {
		return Self::from_url(Method::GET, url);
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

	pub fn body(&mut self, data: impl AsRef<[u8]>) {
		self.body = data.as_ref().to_owned();
	}

	pub fn handle(&self) -> Handle {
		return match self.method {
			Method::GET => Handle::GET(self.path.to_owned()),
			Method::POST => Handle::POST(self.path.to_owned()),
		};
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

		bytes.append(&mut self.body.clone());

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

pub fn server(loc: &str, port: u16) -> Server {
	return Server::new(loc, port);
}

