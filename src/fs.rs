// wengwengweng

//! Common File System Functions

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use directories::BaseDirs;

use crate::Error;
use crate::Result;

#[cfg(target_os = "macos")]
fn get_res_dir() -> Result<PathBuf> {

	use core_foundation::bundle;

	let bundle = bundle::CFBundle::main_bundle();
	let path = bundle
		.executable_url().ok_or(Error::IO)?
		.to_path().ok_or(Error::IO)?
		.parent().ok_or(Error::IO)?
		.parent().ok_or(Error::IO)?
		.join("Resources");

	return Ok(path);

}

#[cfg(not(target_os = "macos"))]
fn get_res_dir() -> Result<PathBuf> {

	use std::env;

	let path = env::current_exe()?
		.parent().ok_or(Error::IO)?
		.to_path_buf();

	return Ok(path);

}

fn validate_path(path: impl AsRef<Path>) -> Option<PathBuf> {

	let path = path.as_ref();

	if !Path::new(path).exists() {

		let with_res = get_res_dir().ok()?.join(path);

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
pub fn glob(pat: &str) -> Result<Vec<PathBuf>> {

	let listings = glob::glob(&format!("{}", pat))
		.or(glob::glob(&format!("{}/{}", get_res_dir()?.display(), pat)))
		.map_err(|_| Error::DirRead(PathBuf::from(pat)))?;

	return Ok(listings
		.filter(|s| s.is_ok())
		.map(|s| s.expect("failed to glob"))
		.collect());

}

/// get bytes read from given file
pub fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {

	let path = path.as_ref();
	let path = validate_path(path)
		.ok_or(Error::FileRead(path.to_owned()))?;

	return fs::read(&path)
		.map_err(|_| Error::FileRead(path.to_owned()));

}

/// get string read from given file
pub fn read_str(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();
	let path = validate_path(path)
		.ok_or(Error::FileRead(path.to_owned()))?;

	return fs::read_to_string(&path)
		.map_err(|_| Error::FileRead(path.to_owned()));

}

/// get the basename of given file
pub fn basename(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();
	let path = validate_path(path)
		.ok_or(Error::FileBasename(path.to_owned()))?;

	return Ok(
		Path::new(&path)
			.file_stem()
			.ok_or(Error::FileBasename(path.to_owned()))?
			.to_str()
			.ok_or(Error::FileBasename(path.to_owned()))?
			.to_owned()
	);

}

/// get the extension of given file
pub fn extname(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();
	let ext = path
		.extension()
		.ok_or(Error::FileExt(path.to_owned()))?
		.to_os_string()
		.into_string()?;

	return Ok(ext);

}

pub fn copy(p1: impl AsRef<Path>, p2: impl AsRef<Path>) -> Result<u64> {

	let p1 = p1.as_ref();
	let p2 = p2.as_ref();

	return fs::copy(p1, p2)
		.map_err(|_| Error::FileCopy(p1.to_owned(), p2.to_owned()));

}

pub fn mkdir(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::create_dir_all(path)
		.map_err(|_| Error::Mkdir(path.to_owned()));

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
		.map_err(|_| Error::FileRemove(path.to_owned()));

}

pub fn remove_dir(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::remove_dir(path)
		.map_err(|_| Error::DirRemove(path.to_owned()));

}

pub fn rename(path: impl AsRef<Path>, new: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::rename(path, new)
		.map_err(|_| Error::Rename(path.to_owned()));

}

pub fn write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {

	let path = path.as_ref();

	return fs::write(path, content)
		.map_err(|_| Error::FileWrite(path.to_owned()));

}

pub fn size(path: impl AsRef<Path>) -> Result<u64> {

	let path = path.as_ref();
	let len = fs::metadata(path)
		.map_err(|_| Error::FileRead(path.to_owned()))?
		.len();

	return Ok(len);

}

pub fn data_dir(name: &str) -> Result<PathBuf> {

	let dirs = BaseDirs::new().ok_or(Error::GetDataDir)?;
	let data_dir = dirs.data_dir();
	let data_dir = data_dir.join(name);

	if !data_dir.exists() {
		mkdir(&data_dir)?;
	}

	return Ok(data_dir);

}

pub fn join(a: impl AsRef<Path>, b: impl AsRef<Path>) -> PathBuf {
	return a.as_ref().join(b.as_ref());
}

