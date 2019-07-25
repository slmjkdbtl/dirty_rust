// wengwengweng

//! HTTP Requests & Servers

use std::io::Write;
use std::io::Read;
use std::net::TcpStream;
use std::net::TcpListener;
use std::str::FromStr;
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

unsafe impl Send for Router {}

impl Router {

	pub fn new() -> Self {
		return Self {
			handlers: Vec::new(),
		};
	}

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
	status: Status,
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
	pub fn set(&mut self, key: Header, val: &str) {
		self.map.insert(key, val.to_owned());
	}
}

impl ToString for HeaderMap {
	fn to_string(&self) -> String {
		let mut m = String::new();
		for (key, val) in &self.map {
			m.push_str(&format!("{}: {}", key.as_str(), val));
			m.push_str("\r\n");
		}
		return m;
	}
}

macro_rules! gen_headers {

	($($name:ident => $msg:expr),*$(,)?) => {

		#[derive(Clone, Copy, Hash, PartialEq, Eq)]
		pub enum Header {
			$(
				$name,
			)*
		}

		impl Header {

			pub fn as_str(&self) -> &'static str {
				return match self {
					$(
						Header::$name => $msg,
					)*
				};
			}

		}

	}

}

gen_headers! {
	Accept => "Accept",
	ContentType => "Content-Type",
	ContentLength => "Content-Length",
	Connection => "Connection",
	Host => "Host",
	Location => "Location",
}

macro_rules! gen_content_type {

	($($name:ident($($ext:expr),*) => $msg:expr),*$(,)?) => {

		#[derive(Clone, Copy)]
		pub enum ContentType {
			$(
				$name,
			)*
		}

		impl ContentType {

			pub fn as_str(&self) -> &'static str {
				return match self {
					$(
						ContentType::$name => $msg,
					)*
				}
			}

			pub fn from_ext(ext: &str) -> Option<Self> {
				return match ext {
					$(
						$(
							$ext => Some(ContentType::$name),
						)*
					)*
					_ => None,
				};
			}

		}

	}

}

gen_content_type! {
	Text("txt") => "text/plain; charset=utf-8",
	HTML("html", "htm") => "text/html; charset=utf-8",
	Markdown("md", "markdown") => "text/markdown; charset=utf-8",
	CSS("css") => "text/css; charset=utf-8",
	PNG("png") => "image/png",
	JPEG("jpg", "jpeg") => "image/jpeg",
	GIF("gif") => "image/gif",
	PDF("pdf") => "application/pdf",
	JavaScript("js") => "application/javascript; charset=utf-8",
	JSON("json") => "application/json; charset=utf-8",
	ZIP("zip") => "application/zip",
	MP3("mp3") => "audio/mpeg",
	OGG("ogg") => "audio/ogg",
	WAV("wav") => "audio/wav",
	MIDI("midi") => "audio/midi",
	TTF("ttf") => "font/ttf",
	OTF("otf") => "font/otf",
	WOFF("woff") => "font/woff",
	WOFF2("woff2") => "font/woff2",
	MP4("mp4") => "video/mp4",
}

macro_rules! gen_status {

	($($code:expr, $name:ident => $reason:expr),*$(,)?) => {

		#[derive(Clone, Copy)]
		pub enum Status {
			$(
				$name,
			)*
		}

		impl Status {

			pub fn from_code(code: u16) -> Option<Self> {
				return match code {
					$(
						$code => Some(Status::$name),
					)*
					_ => None,
				};
			}

			pub fn reason(&self) -> &'static str {
				return match self {
					$(
						Status::$name => $reason,
					)*
				};
			}

			pub fn code(&self) -> u16 {
				return match self {
					$(
						Status::$name => $code,
					)*
				};
			}

		}

	}

}

gen_status! {
	100, Continue => "Continue",
	101, SwitchingProtocols => "Switching Protocols",
	102, Processing => "Processing",
	200, Ok => "OK",
	201, Created => "Created",
	202, Accepted => "Accepted",
	203, NonAuthoritativeInformation => "Non-Authoritative Information",
	204, NoContent => "No Content",
	205, ResetContent => "Reset Content",
	206, PartialContent => "Partial Content",
	207, MultiStatus => "Multi-Status",
	208, AlreadyReported => "Already Reported",
	226, ImUsed => "IM Used",
	300, MultipleChoices => "Multiple Choices",
	301, MovedPermanently => "Moved Permanently",
	302, Found => "Found",
	303, SeeOther => "See Other",
	304, NotModified => "Not Modified",
	305, UseProxy => "Use Proxy",
	307, TemporaryRedirect => "Temporary Redirect",
	308, PermanentRedirect => "Permanent Redirect",
	400, BadRequest => "Bad Request",
	401, Unauthorized => "Unauthorized",
	402, PaymentRequired => "Payment Required",
	403, Forbidden => "Forbidden",
	404, NotFound => "Not Found",
	405, MethodNotAllowed => "Method Not Allowed",
	406, NotAcceptable => "Not Acceptable",
	407, ProxyAuthenticationRequired => "Proxy Authentication Required",
	408, RequestTimeout => "Request Timeout",
	409, Conflict => "Conflict",
	410, Gone => "Gone",
	411, LengthRequired => "Length Required",
	412, PreconditionFailed => "Precondition Failed",
	413, PayloadTooLarge => "Payload Too Large",
	414, UriTooLong => "URI Too Long",
	415, UnsupportedMediaType => "Unsupported Media Type",
	416, RangeNotSatisfiable => "Range Not Satisfiable",
	417, ExpectationFailed => "Expectation Failed",
	418, ImATeapot => "I'm a teapot",
	421, MisdirectedRequest => "Misdirected Request",
	422, UnprocessableEntity => "Unprocessable Entity",
	423, Locked => "Locked",
	424, FailedDependency => "Failed Dependency",
	426, UpgradeRequired => "Upgrade Required",
	428, PreconditionRequired => "Precondition Required",
	429, TooManyRequests => "Too Many Requests",
	431, RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
	451, UnavailableForLegalReasons => "Unavailable For Legal Reasons",
	500, InternalServerError => "Internal Server Error",
	501, NotImplemented => "Not Implemented",
	502, BadGateway => "Bad Gateway",
	503, ServiceUnavailable => "Service Unavailable",
	504, GatewayTimeout => "Gateway Timeout",
	505, HttpVersionNotSupported => "HTTP Version Not Supported",
	506, VariantAlsoNegotiates => "Variant Also Negotiates",
	507, InsufficientStorage => "Insufficient Storage",
	508, LoopDetected => "Loop Detected",
	510, NotExtended => "Not Extended",
	511, NetworkAuthenticationRequired => "Network Authentication Required"
}

impl Response {

	pub fn set_body(&mut self, body: impl AsRef<[u8]>) {

		self.body = body.as_ref().to_owned();
		self.set_header(Header::ContentLength, &self.body.len().to_string());

	}

	pub fn redirect(url: &str) -> Self {

		let mut headers = HeaderMap::new();

		headers.set(Header::Location, url);

		return Self {
			body: Vec::new(),
			status: Status::SeeOther,
			headers: headers,
		}

	}

	pub fn new(status: Status, t: ContentType, body: impl AsRef<[u8]>) -> Self {

		let body = body.as_ref();
		let mut headers = HeaderMap::new();

		headers.set(Header::ContentLength, &body.len().to_string());
		headers.set(Header::ContentType, t.as_str());

		return Self {
			body: body.to_owned(),
			status: status,
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
		let code = res.code.ok_or(Error::Net)?;
		let status = Status::from_code(code).ok_or(Error::Net)?;

		return Ok(Self {
			body: body.to_owned(),
			status: status,
			headers: HeaderMap::new(),
		});

	}

	pub fn bytes(&self) -> &[u8] {
		return &self.body;
	}

	pub fn text(&self) -> String {
		return String::from_utf8_lossy(self.bytes()).to_owned().to_string();
	}

	pub fn status(&self) -> Status {
		return self.status;
	}

	pub fn set_header(&mut self, key: Header, value: &str) {
		self.headers.set(key, value);
	}

	pub fn headers(&self) -> &HeaderMap {
		return &self.headers;
	}

	pub fn message(&self) -> Vec<u8> {

		let mut m = Vec::new();
		let status = self.status();

		m.extend_from_slice(&format!("HTTP/1.1 {} {}", status.code(), status.reason()).as_bytes());
		m.extend_from_slice("\r\n".as_bytes());
		m.extend_from_slice(&self.headers.to_string().as_bytes());
		m.extend_from_slice("\r\n".as_bytes());
		m.extend_from_slice(&self.body);

		return m;

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

// 				let mut config = rustls::ClientConfig::new();

// 				config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

// 				let dns_name = webpki::DNSNameRef::try_from_ascii_str(self.host())?;
// 				let mut tlssession = rustls::ClientSession::new(&Arc::new(config), dns_name);
// 				let mut tlsstream = rustls::Stream::new(&mut tlssession, &mut stream);

// 				tlsstream.write_all(&self.message())?;
// 				tlsstream.read_to_end(&mut buf)?;

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
	return Request::post(url)?.send(Some(data.as_ref()));
}

pub fn serve<F: Fn(Request) -> Response>(loc: &str, port: u16, handler: F) -> Result<()> {

	let listener = TcpListener::bind((loc, port))?;

	for stream in listener.incoming() {

		let mut stream = stream?;
		let mut buf = [0; 512];

		stream.read(&mut buf)?;

		let req = Request::from_raw(&buf)?;
		let res = handler(req);

		stream.write_all(&res.message())?;

	}

	return Ok(());

}

