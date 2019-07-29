// wengwengweng

use std::io::Write;
use std::io::Read;
use std::net::TcpListener;

use crate::Result;
use super::*;

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

