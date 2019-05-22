// wengwengweng

//! Common File System Functions

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::Error;

#[cfg(target_os = "macos")]
fn get_res_dir() -> PathBuf {

	use core_foundation::bundle;

	let bundle = bundle::CFBundle::main_bundle();
	let path = bundle
		.executable_url().expect("Cannot get executable dir")
		.to_path().expect("to_path error")
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.join("Resources");

	return path;

}

#[cfg(not(target_os = "macos"))]
fn get_res_dir() -> PathBuf {

	use std::env;

	let path = env::current_exe()
		.expect("Cannot get application dir")
		.parent().expect("Cannot get application dir")
		.to_path_buf();

	return path;

}

fn validate_path(path: impl AsRef<Path>) -> Option<PathBuf> {

	let path = path.as_ref();

	if !Path::new(path).exists() {

		let with_res = get_res_dir().join(path);

		if Path::new(&with_res).exists() {
			return Some(with_res);
		} else {
			return None;
		}

	} else {

		return Some(path.to_owned());

	}

}

/// check if given file exists
pub fn exists(path: impl AsRef<Path>) -> bool {
	return validate_path(path).is_some();
}

/// get a list of all filenames under given directory
pub fn glob(pat: &str) -> Vec<String> {

	let listings = glob::glob(&format!("{}", pat))
		.or(glob::glob(&format!("{}/{}", get_res_dir().display(), pat)))
		.expect(&format!("failed to read dir \"{}\"", pat));

	return listings
		.map(|s| s.expect("failed to glob"))
		.map(|s| s.into_os_string())
		.map(|s| s.into_string())
		.map(|s| s.expect("failed to glob"))
		.collect::<Vec<String>>();

}

/// get bytes read from given file
pub fn read_bytes(path: impl AsRef<Path>) -> Vec<u8> {

	let path = path.as_ref();
	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path.display()));

	if let Ok(content) = fs::read(&path) {
		return content;
	} else {
		panic!("failed to read file \"{}\"", path.display());
	}

}

/// get string read from given file
pub fn read_str(path: impl AsRef<Path>) -> String {

	let path = path.as_ref();
	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path.display()));

	if let Ok(content) = fs::read_to_string(&path) {
		return content;
	} else {
		panic!("failed to read file \"{}\"", path.display());
	}

}

/// get the basename of given file
pub fn basename(path: impl AsRef<Path>) -> String {

	let path = path.as_ref();
	let path = validate_path(path).expect(&format!("failed to read file \"{}\"", path.display()));

	if let Some(name) = Path::new(&path).file_stem() {
		return name.to_str().expect("failed to get basename").to_owned();
	} else {
		panic!("failed to read file \"{}\"", path.display());
	}

}

pub fn copy(p1: impl AsRef<Path>, p2: impl AsRef<Path>) -> Result<u64, Error> {
	return Ok(fs::copy(p1, p2)?);
}

pub fn mkdir(path: impl AsRef<Path>) -> Result<(), Error> {
	return Ok(fs::create_dir_all(path)?);
}

pub fn is_file(path: impl AsRef<Path>) -> bool {
	return path.as_ref().is_file();
}

pub fn is_dir(path: impl AsRef<Path>) -> bool {
	return path.as_ref().is_dir();
}

pub fn remove(path: impl AsRef<Path>) -> Result<(), Error> {
	return Ok(fs::remove_file(path)?);
}

pub fn remove_dir(path: impl AsRef<Path>) -> Result<(), Error> {
	return Ok(fs::remove_dir(path)?);
}

pub fn rename(old: impl AsRef<Path>, new: impl AsRef<Path>) -> Result<(), Error> {
	return Ok(fs::rename(old, new)?);
}

pub fn write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<(), Error> {
	return Ok(fs::write(path, content)?);
}

