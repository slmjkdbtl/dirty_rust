// wengwengweng

//! Common File System Functions

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use directories::BaseDirs;

use crate::Error;
use crate::Result;

#[cfg(target_os = "macos")]
fn get_res_dir() -> Option<PathBuf> {

	use core_foundation::bundle;

	let bundle = bundle::CFBundle::main_bundle();

	let path = bundle
		.executable_url()?
		.to_path()?
		.parent()?
		.parent()?
		.join("Resources");

	return Some(path);

}

#[cfg(not(target_os = "macos"))]
fn get_res_dir() -> Option<PathBuf> {

	use std::env;

	let path = env::current_exe()
		.ok()?
		.parent()?
		.to_path_buf();

	return Some(path);

}

fn validate_path(path: impl AsRef<Path>) -> Result<PathBuf> {

	let path = path.as_ref();

	if Path::new(path).exists() {

		return Ok(path.to_path_buf());

	} else {

		let with_res = get_res_dir()
			.ok_or(Error::Fs(format!("file not found: {}", path.display())))?
			.join(path);

		if Path::new(&with_res).exists() {
			return Ok(with_res);
		} else {
			return Err(Error::Fs(format!("file not found: {}", path.display())));
		}

	}

}

/// check if given file exists
pub fn exists(path: impl AsRef<Path>) -> bool {
	return validate_path(path).is_ok();
}

/// get a list of all filenames under given directory
pub fn glob(pat: &str) -> Result<Vec<PathBuf>> {

	let listings = glob::glob(&format!("{}", pat))
		.ok()
		.or_else(|| glob::glob(&format!("{}/{}", get_res_dir()?.display(), pat)).ok())
		.ok_or(Error::Fs(format!("failed to execute glob pattern {}", pat)))?
		.flatten()
		.collect();

	return Ok(listings);

}

/// get bytes read from given file
pub fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {

	let path = path.as_ref();
	let path = validate_path(path)?;

	return fs::read(&path)
		.map_err(|_| Error::Fs(format!("failed to read file {}", path.display())));

}

/// get string read from given file
pub fn read_str(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();
	let path = validate_path(path)?;

	return fs::read_to_string(&path)
		.map_err(|_| Error::Fs(format!("failed to read file {}", path.display())));

}

/// get the basename of given file
pub fn basename(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();
	let path = validate_path(path)?;

	return Ok(
		Path::new(&path)
			.file_stem()
			.ok_or(Error::Fs(format!("failed to get basename: {}", path.display())))?
			.to_str()
			.ok_or(Error::Fs(format!("failed to get basename: {}", path.display())))?
			.to_owned()
	);

}

/// get the extension of given file
pub fn extname(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();

	return Ok(path
		.extension()
		.ok_or(Error::Fs(format!("failed to get extname: {}", path.display())))?
		.to_os_string()
		.into_string().map_err(|_| Error::Fs(format!("failed to get extname: {}", path.display())))?
	);

}

pub fn copy(p1: impl AsRef<Path>, p2: impl AsRef<Path>) -> Result<u64> {

	let p1 = p1.as_ref();
	let p2 = p2.as_ref();

	return fs::copy(p1, p2)
		.map_err(|_| Error::Fs(format!("failed to copy {} to {}", p1.display(), p2.display())));

}

pub fn mkdir(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::create_dir_all(path)
		.map_err(|_| Error::Fs(format!("failed to create directory {}", path.display())));

}

pub fn is_file(path: impl AsRef<Path>) -> bool {
	return path.as_ref().is_file();
}

pub fn is_dir(path: impl AsRef<Path>) -> bool {
	return path.as_ref().is_dir();
}

pub fn remove(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::remove_file(path)
		.map_err(|_| Error::Fs(format!("failed to remove file {}", path.display())));

}

pub fn remove_dir(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::remove_dir(path)
		.map_err(|_| Error::Fs(format!("failed to remove directory {}", path.display())));

}

pub fn rename(path: impl AsRef<Path>, new: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::rename(path, new)
		.map_err(|_| Error::Fs(format!("failed to rename {}", path.display())));

}

pub fn write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {

	let path = path.as_ref();

	return fs::write(path, content)
		.map_err(|_| Error::Fs(format!("failed to write file {}", path.display())));

}

pub fn size(path: impl AsRef<Path>) -> Result<u64> {

	let path = path.as_ref();
	let len = fs::metadata(path)
		.map_err(|_| Error::Fs(format!("failed to read file {}", path.display())))?
		.len();

	return Ok(len);

}

pub fn data_dir(name: &str) -> Result<PathBuf> {

	let dirs = BaseDirs::new().ok_or(Error::Fs("failed to get data dir".into()))?;
	let data_dir = dirs.data_dir();
	let data_dir = data_dir.join(name);

	if !data_dir.exists() {
		mkdir(&data_dir)?;
	}

	return Ok(data_dir);

}

