// wengwengweng

use std::io::Read;
use sdl2::rwops::RWops;

pub fn file_read(path: &str) -> Vec<u8> {

	let mut r = RWops::from_file(path, "rb").expect("no file");
	let len = r.len().expect("file failed");
	let mut buf = vec![0; len];

	r.read(&mut buf);

	return buf;

}

pub fn file_exists(path: &str) -> bool {
	return RWops::from_file(path, "rb").is_ok();
}

