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
		use Method::*;
		return match self {
			GET => "GET",
			POST => "POST",
		}.to_owned();
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

pub struct Router {
	handlers: Vec<Box<Fn(&Request) -> Option<Response>>>,
}

impl Router {
	pub fn get<D: AsRef<[u8]>, F: Fn() -> D + 'static>(&mut self, pat: &str, f: F) {
		self.handlers.push(Box::new(move |req| {
			f();
			return None;
		}));
	}
}

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

#[derive(Clone)]
pub struct Response {
	body: Vec<u8>,
	code: u16,
	headers: HeaderMap,
}

#[derive(Clone)]
pub struct HeaderMap {
	map: HashMap<Header, String>,
}

impl HeaderMap {
	pub fn new() -> Self {
		return Self {
			map: HashMap::new(),
		};
	}
	pub fn add(&mut self, key: Header, val: &str) {
		self.map.insert(key, val.to_owned());
	}
}

impl ToString for HeaderMap {
	fn to_string(&self) -> String {
		let mut m = String::new();
		for (key, val) in &self.map {
			m.push_str(&format!("{}: {}", key.to_string(), val));
			m.push_str("\r\n");
		}
		return m;
	}
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Header {
	ContentType,
	Connection,
	Host,
}

impl ToString for Header {
	fn to_string(&self) -> String {
		use Header::*;
		return match self {
			ContentType => "Content-Type",
			Connection => "Connection",
			Host => "Host",
		}.to_owned();
	}
}

pub enum ContentType {
	Text,
	HTML,
	CSS,
	JavaScript,
	JSON,
	Markdown,
	PNG,
	JPEG,
	GIF,
	PDF,
	MP3,
	OGG,
	WAV,
	MIDI,
	TTF,
	OTF,
	WOFF,
	WOFF2,
	MP4,
	ZIP,
}

impl ToString for ContentType {
	fn to_string(&self) -> String {
		use ContentType::*;
		return match self {
			Text => "text/plain; charset=utf-8",
			HTML => "text/html; charset=utf-8",
			Markdown => "text/markdown; charset=utf-8",
			CSS => "text/css; charset=utf-8",
			PNG => "image/png",
			JPEG => "image/jpeg",
			GIF => "image/gif",
			PDF => "application/pdf",
			JavaScript => "application/javascript; charset=utf-8",
			JSON => "application/json; charset=utf-8",
			ZIP => "application/zip",
			MP3 => "audio/mpeg",
			OGG => "audio/ogg",
			WAV => "audio/wav",
			MIDI => "audio/midi",
			TTF => "font/ttf",
			OTF => "font/otf",
			WOFF => "font/woff",
			WOFF2 => "font/woff2",
			MP4 => "video/mp4",
		}.to_owned();
	}
}

impl ContentType {

	pub fn from_ext(ext: &str) -> Option<Self> {
		use ContentType::*;
		return match ext {
			"txt" => Some(Text),
			"html" => Some(HTML),
			"md" => Some(Markdown),
			"css" => Some(CSS),
			"png" => Some(PNG),
			"jpeg" => Some(JPEG),
			"jpg" => Some(JPEG),
			"gif" => Some(GIF),
			"pdf" => Some(PDF),
			"js" => Some(JavaScript),
			"json" => Some(JSON),
			"mp3" => Some(MP3),
			"ogg" => Some(OGG),
			"wav" => Some(WAV),
			"midi" => Some(MIDI),
			"ttf" => Some(TTF),
			"otf" => Some(OTF),
			"woff" => Some(WOFF),
			"woff2" => Some(WOFF2),
			"mp4" => Some(MP4),
			"zip" => Some(ZIP),
			_ => None,
		};
	}

}

impl Response {

	pub fn new(t: ContentType, body: impl AsRef<[u8]>) -> Self {

		let mut headers = HeaderMap::new();

		headers.add(Header::ContentType, &t.to_string());

		return Self {
			body: body.as_ref().to_owned(),
			code: 200,
			headers: headers,
		};

	}

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
			headers: HeaderMap::new(),
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

	pub fn add_header(&mut self, key: Header, value: &str) {
		self.headers.add(key, value);
	}

	pub fn headers(&self) -> &HeaderMap {
		return &self.headers;
	}

	pub fn message(&self) -> Vec<u8> {

		let mut m = String::new();

		m.push_str("HTTP/1.1 200 OK");
		m.push_str("\r\n");
		m.push_str(&self.headers.to_string());
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

		headers.add(Header::Host, host);

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

	pub fn add_header(&mut self, key: Header, value: &str) {
		self.headers.add(key, value);
	}

	pub fn body(&mut self, data: impl AsRef<[u8]>) {
		self.body = data.as_ref().to_owned();
	}

	pub fn message(&self) -> Vec<u8> {

		let mut m = String::new();

		m.push_str(&format!("{} {} {}", self.method().to_string(), self.path(), self.version().to_string()));
		m.push_str("\r\n");
		m.push_str(&self.headers.to_string());
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

pub fn post(url: &str, data: impl AsRef<[u8]>) -> Result<Response> {
	return Request::get(url)?.send(Some(data.as_ref()));
}

pub fn serve<F: Fn(Request) -> Response>(loc: &str, port: u16, handler: F) -> Result<()> {

	let listener = TcpListener::bind((loc, port))?;

	for stream in listener.incoming() {

		let mut stream = stream?;
		let mut buf = [0; 512];

		stream.read(&mut buf)?;

		let req = Request::from_raw(&buf)?;
		let res = handler(req);

		stream.write_all(&res.message());

	}

	return Ok(());

}

