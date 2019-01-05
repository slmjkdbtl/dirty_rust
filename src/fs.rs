// wengwengweng

//! Handles filesystems

use std::fs;
use std::path::{Path, PathBuf};
use std::env;

use crate::*;

#[cfg(target_os = "macos")]
use core_foundation::bundle;

#[cfg(target_os = "macos")]
fn get_res_dir() -> String {

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

	return env::current_exe()
		.expect("Cannot get application dir")
		.parent().expect("Cannot get application dir")
		.to_path_buf()
		.into_os_string()
		.into_string()
		.unwrap();

}

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

pub fn glob(path: &str) -> Vec<String> {

	if let Ok(path) = validate_path(path) {
		if let Ok(entries) = fs::read_dir(&path) {
			return vec![];
		} else {
			app::error(&format!("failed to read dir \"{}\"", path));
		}
	} else {
		app::error(&format!("failed to read dir \"{}\"", path));
	}

	return vec![];
}

pub fn read_bytes(path: &str) -> Vec<u8> {

	if let Ok(path) = validate_path(path) {
		if let Ok(content) = fs::read(&path) {
			return content;
		} else {
			app::error(&format!("failed to read file \"{}\"", path));
		}
	} else {
		app::error(&format!("failed to read file \"{}\"", path));
	}


	return Vec::new();

}

pub fn read_str(path: &str) -> String {

	if let Ok(path) = validate_path(path) {
		if let Ok(content) = fs::read_to_string(&path) {
			return content;
		} else {
			app::error(&format!("failed to read file \"{}\"", path));
		}
	} else {
		app::error(&format!("failed to read file \"{}\"", path));
	}

	return String::new();

}

