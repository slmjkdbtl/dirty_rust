// wengwengweng

//! Common File System Functions

use std::fs;
use std::path::Path;

#[cfg(target_os = "macos")]
fn get_res_dir() -> String {

	use core_foundation::bundle;

	let bundle = bundle::CFBundle::main_bundle();

	let exe = bundle
		.executable_url().expect("Cannot get executable dir")
		.to_path().expect("to_path error");

	return exe
		.parent()
		.expect("failed to get parent dir")
		.parent()
		.expect("failed to get parent dir")
		.join("Resources")
		.into_os_string()
		.into_string()
		.expect("failed to convert pathbuf to string")

}

#[cfg(not(target_os = "macos"))]
fn get_res_dir() -> String {

	use std::env;

	return env::current_exe()
		.expect("Cannot get application dir")
		.parent().expect("Cannot get application dir")
		.to_path_buf()
		.into_os_string()
		.into_string()
		.expect("failed to convert pathbuf to string")

}

/// check if given file exists
pub fn exists(path: &str) -> bool {
	return validate_path(path).is_some();
}

fn validate_path(path: &str) -> Option<String> {

	if !Path::new(path).exists() {

		let with_res = format!("{}/{}", get_res_dir(), path);

		if Path::new(&with_res).exists() {
			return Some(with_res);
		} else {
			return None;
		}

	} else {

		return Some(path.to_owned());

	}

}

/// get a list of all filenames under given directory
pub fn glob(path: &str) -> Vec<String> {

	let listings = glob::glob(path)
		.or(glob::glob(&format!("{}/{}", get_res_dir(), path)))
		.expect(&format!("failed to read dir \"{}\"", path));

	return listings
		.map(|s| s.expect("failed to glob"))
		.map(|s| s.into_os_string())
		.map(|s| s.into_string())
		.map(|s| s.expect("failed to glob"))
		.collect::<Vec<String>>();

}

/// get bytes read from given file
pub fn read_bytes(path: &str) -> Vec<u8> {

	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path));

	if let Ok(content) = fs::read(&path) {
		return content;
	} else {
		panic!("failed to read file \"{}\"", path);
	}

}

/// get string read from given file
pub fn read_str(path: &str) -> String {

	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path));

	if let Ok(content) = fs::read_to_string(&path) {
		return content;
	} else {
		panic!("failed to read file \"{}\"", path);
	}

}

/// get the basename of given file
pub fn basename(path: &str) -> String {

	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path));

	if let Some(name) = Path::new(&path).file_stem() {
		return name.to_str().expect("failed to get basename").to_owned();
	} else {
		panic!("failed to read file \"{}\"", path);
	}

}

