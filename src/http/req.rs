// wengwengweng

use url::Url;

use crate::Result;
use super::*;

#[derive(Clone)]
pub struct Request {
	method: Method,
	scheme: Scheme,
	version: Version,
	host: String,
	path: String,
	port: u16,
	auth: Option<Auth>,
	content_type: Option<ContentType>,
	body: Body,
}

impl Request {

	pub(super) fn new(method: Method, url: &str) -> Result<Self> {

		let url = Url::parse(url).map_err(|_| format!("failed to parse url"))?;
		let scheme = url
			.scheme()
			.parse::<Scheme>()
			.map_err(|_| format!("failed to parse url scheme"))?;

		let host = url
			.host_str()
			.ok_or(format!("failed to parse url host addr"))?;

		let path = url.path();

		return Ok(Self {
			method: method,
			version: Version::V10,
			scheme: scheme,
			host: host.to_owned(),
			path: path.to_owned(),
			port: scheme.port(),
			auth: None,
			content_type: None,
			body: Body::empty(),
		});

	}

	pub fn send_bytes(mut self, data: &[u8]) -> Result<Response> {
		self.body = Body::from_bytes(data);
		return client::send(self);
	}

	pub fn send_text(mut self, data: &str) -> Result<Response> {
		self.body = Body::from_string(data);
		if self.content_type.is_none() {
			self.content_type = Some(ContentType::Text);
		}
		return client::send(self);
	}

	#[cfg(feature = "json")]
	pub fn send_json<D: serde::ser::Serialize>(mut self, data: D) -> Result<Response> {
		self.body = Body::from_json(data)?;
		if self.content_type.is_none() {
			self.content_type = Some(ContentType::JSON);
		}
		return client::send(self);
	}

	pub fn content_type(mut self, t: ContentType) -> Self {
		self.content_type = Some(t);
		return self;
	}

	pub fn auth(mut self, auth: Auth) -> Self {
		self.auth = Some(auth);
		return self;
	}

	pub fn send(self) -> Result<Response> {
		return client::send(self);
	}

	pub(super) fn host(&self) -> &str {
		return &self.host;
	}

	pub(super) fn port(&self) -> u16 {
		return self.port;
	}

	pub(super) fn scheme(&self) -> Scheme {
		return self.scheme;
	}

	pub(super) fn into_msg(self) -> Vec<u8> {

		let mut msg = format!("{} {} {}\r\n", self.method.as_str(), self.path, self.version.as_str());

		msg.push_str(&format!("Host: {}\r\n", self.host));

		if let Some(ctype) = self.content_type {
			msg.push_str(&format!("Content-Type: {}\r\n", ctype.as_str()));
		}

		if let Some(auth) = self.auth {
			msg.push_str(&format!("Authorization: {}\r\n", auth.to_string()));
		}

		msg.push_str("\r\n");

		let mut payload = msg.into_bytes();

		payload.append(&mut self.body.into_bytes());

		return payload;

	}

}

