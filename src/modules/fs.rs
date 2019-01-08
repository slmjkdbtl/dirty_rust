// wengwengweng

//! Common File Related Functions

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
		.unwrap()
		.parent()
		.unwrap()
		.join("Resources")
		.into_os_string()
		.into_string()
		.unwrap();

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
		.unwrap();

}

/// check if given file exists
pub fn exists(path: &str) -> bool {
	return validate_path(path).is_ok();
}

fn validate_path(path: &str) -> Result<String, ()> {

	if !Path::new(path).exists() {

		let with_res = format!("{}/{}", get_res_dir(), path);

		if Path::new(&with_res).exists() {
			return Ok(with_res);
		} else {
			return Err(());
		}

	} else {

		return Ok(path.to_owned());

	}

}

/// get a Vec of all filenames under given directory
pub fn glob(path: &str) -> Vec<String> {

	let mut entries: Vec<String> = Vec::new();

	if let Ok(listings) = glob::glob(path) {
		for item in listings {
			if let Ok(entry) = item {
				entries.push(entry.into_os_string().into_string().unwrap());
			}
		}
	} else {
		if let Ok(listings) = glob::glob(&format!("{}/{}", get_res_dir(), path)) {
			for item in listings {
				if let Ok(entry) = item {
					entries.push(entry.into_os_string().into_string().unwrap());
				}
			}
		} else {
			panic!("failed to read dir \"{}\"", path);
		}
	}

	return entries;

}

/// get bytes read from given file
pub fn read_bytes(path: &str) -> Vec<u8> {

	if let Ok(path) = validate_path(path) {
		if let Ok(content) = fs::read(&path) {
			return content;
		} else {
			panic!("failed to read file \"{}\"", path);
		}
	} else {
		panic!("failed to read file \"{}\"", path);
	}

}

/// get string read from given file
pub fn read_str(path: &str) -> String {

	if let Ok(path) = validate_path(path) {
		if let Ok(content) = fs::read_to_string(&path) {
			return content;
		} else {
			panic!("failed to read file \"{}\"", path);
		}
	} else {
		panic!("failed to read file \"{}\"", path);
	}

}

/// get the basename of given file
pub fn basename(path: &str) -> String {

	if let Ok(path) = validate_path(path) {
		if let Some(name) = Path::new(&path).file_stem() {
			return name.to_str().unwrap().to_owned();
		} else {
			panic!("failed to read file \"{}\"", path);
		}
	} else {
		panic!("failed to read file \"{}\"", path);
	}

}

