// wengwengweng

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
	return Path::new(path).exists();
}

pub fn validate_path(path: &str) -> String {

	if !exists(path) {

		let with_res = format!("{}/{}", get_res_dir(), path);

		if exists(&with_res) {
			return with_res;
		} else {
			fail!("can't find file {}", path);
		}

	} else {

		return path.to_owned();

	}

}

pub fn read_bytes(path: &str) -> Vec<u8> {

	let path = validate_path(path);

	return fs::read(&path).unwrap_or_else(|s| {
		fail!("failed to read {}", path);
	});

}

pub fn read_str(path: &str) -> String {

	let path = validate_path(path);

	return fs::read_to_string(&path).unwrap_or_else(|s| {
		fail!("failed to read {}", path);
	});

}

