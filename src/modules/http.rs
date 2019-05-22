// wengwengweng

//! Networking

use std::io::Write;
use std::io::Read;
use std::net::TcpStream;

use crate::err::*;

pub fn get(uri: &str) -> Result<String, Error> {

	let mut res = reqwest::get(uri)?;
	let text = res.text()?;

	return Ok(text);

}

pub fn get_bytes(uri: &str) -> Result<Vec<u8>, Error> {

	let mut res = reqwest::get(uri)?;
	let mut buf: Vec<u8> = vec![];

	res.copy_to(&mut buf)?;

	return Ok(buf);

}

