// wengwengweng

use std::io::Read;
use sdl2::rwops::RWops;

pub fn file_exists(path: &str) -> bool {
	return RWops::from_file(path, "rb").is_ok();
}

// pub fn file_read(path: &str) -> &'static str {
	// ...
// }

// pub fn dir_glob() {
	// ...
// }

