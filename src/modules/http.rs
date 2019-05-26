// wengwengweng

//! HTTP Requests & Servers

use crate::Result;

pub struct Response {
	text: String,
	bytes: Vec<u8>,
	status: u16,
}

impl Response {

	pub fn text(&self) -> &String {
		return &self.text;
	}

	pub fn bytes(&self) -> &Vec<u8> {
		return &self.bytes;
	}

	pub fn status(&self) -> u16 {
		return self.status;
	}

}

pub fn get(uri: &str) -> Result<Response> {

	let mut res = reqwest::get(uri)?;
	let mut buf: Vec<u8> = vec![];
	let text = res.text()?;

	res.copy_to(&mut buf)?;

	return Ok(Response {
		text: text,
		bytes: buf,
		status: res.status().as_u16(),
	})

}

