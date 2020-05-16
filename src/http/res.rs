// wengwengweng

// TODO: parse headers

use std::collections::HashMap;

use crate::Result;
use super::*;

#[derive(Clone, Debug)]
pub struct Response {
	body: Body,
	status: Status,
	headers: HashMap<String, String>,
}

impl Response {

	pub(super) fn from(buf: &[u8]) -> Result<Self> {

		let mut pheaders = [httparse::EMPTY_HEADER; 128];
		let mut res = httparse::Response::new(&mut pheaders);

		let body_pos = match res
			.parse(&buf)
			.map_err(|_| format!("failed to parse response"))? {
			httparse::Status::Complete(len) => len,
			httparse::Status::Partial => return Err(format!("incomplete response message")),
		};

		let code = res.code
			.ok_or(format!("failed to parse response status code"))?;

		let status = Status::from_code(code)?;
		let body = &buf[body_pos..];

		let mut headers = hmap![];

		for h in pheaders.iter() {

			let key = h.name.to_string();
			let val = String::from_utf8_lossy(h.value).to_string();

			headers.insert(key, val);

		}

		return Ok(Self {
			body: Body::from_bytes(body),
			status: status,
			headers: headers,
		});

	}

	pub fn get(&self, header: &str) -> Option<&str> {
		return self.headers.get(header).map(AsRef::as_ref);
	}

	pub fn into_body(self) -> Body {
		return self.body;
	}

	pub fn status(&self) -> Status {
		return self.status;
	}

}

