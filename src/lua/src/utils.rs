// wengwengweng

use std::io;
use std::fs::File;
use std::io::Read;

pub fn fread(fname: &str) -> Result<String, io::Error> {

	let mut file = File::open(fname)?;
	let mut buffer = String::new();

	file.read_to_string(&mut buffer)?;

	return Ok(buffer);

}

