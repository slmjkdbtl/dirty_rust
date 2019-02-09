// wengwengweng

//! Common File Related Functions

use std::fs;
use std::path::Path;

use crate::err;

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
	return validate_path(path).is_ok();
}

fn validate_path(path: &str) -> Result<String, err::Error> {

	if !Path::new(path).exists() {

		let with_res = format!("{}/{}", get_res_dir(), path);

		if Path::new(&with_res).exists() {
			return Ok(with_res);
		} else {
			return Err(err::Error::FileSystem(path.to_owned()));
		}

	} else {

		return Ok(path.to_owned());

	}

}

/// get a list of all filenames under given directory
pub fn glob(path: &str) -> Result<Vec<String>, err::Error> {

	let mut entries: Vec<String> = Vec::new();

	if let Ok(listings) = glob::glob(path) {
		for item in listings {
			if let Ok(entry) = item {
				entries.push(entry.into_os_string().into_string().expect("failed to convert pathbuf to string"));
			}
		}
	} else {
		if let Ok(listings) = glob::glob(&format!("{}/{}", get_res_dir(), path)) {
			for item in listings {
				if let Ok(entry) = item {
					entries.push(entry.into_os_string().into_string().expect("failed to convert pathbuf to string"));
				}
			}
		} else {
			return Err(err::Error::FileSystem(path.to_owned()));
		}
	}

	return Ok(entries);

}

/// get bytes read from given file
pub fn read_bytes(path: &str) -> Result<Vec<u8>, err::Error> {

	let path = validate_path(path)?;

	if let Ok(content) = fs::read(&path) {
		return Ok(content);
	} else {
		return Err(err::Error::FileSystem(path));
	}

}

/// get string read from given file
pub fn read_str(path: &str) -> Result<String, err::Error> {

	let path = validate_path(path)?;

	if let Ok(content) = fs::read_to_string(&path) {
		return Ok(content);
	} else {
		return Err(err::Error::FileSystem(path));
	}

}

/// get the basename of given file
pub fn basename(path: &str) -> Result<String, err::Error> {

	let path = validate_path(path)?;

	if let Some(name) = Path::new(&path).file_stem() {
		return Ok(name.to_str().expect("failed to get basename").to_owned());
	} else {
		return Err(err::Error::FileSystem(path));
	}

}

