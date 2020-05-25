// wengwengweng

//! File Systems Function Wrappers That Works with App Bundles
//!
//! All functions here uses [`res_dir`](res_dir()) so that it can read file from app bundle dirs (e.g. on macOS `*.app/Content/Resources`)
//!
//! There're also extra utilitiy functions like [`glob`](glob()) and [`data_dir`](data_dir())

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::Result;

/// get the bundle resource dir
#[cfg(target_os = "macos")]
pub fn res_dir() -> Option<PathBuf> {

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

/// get the bundle resource dir
#[cfg(not(target_os = "macos"))]
pub fn res_dir() -> Option<PathBuf> {

	use std::env;

	let path = env::current_exe()
		.ok()?
		.parent()?
		.to_path_buf();

	return Some(path);

}

/// get the path that's prepended with the res dir, if it doesn't exit then return the input path back
pub fn bundled_path(path: impl AsRef<Path>) -> Result<PathBuf> {

	let path = path.as_ref();

	if path.exists() {

		return Ok(path.to_path_buf());

	} else {

		let with_res = res_dir()
			.ok_or(format!("file not found: {}", path.display()))?
			.join(path);

		if with_res.exists() {
			return Ok(with_res);
		} else {
			return Err(format!("file not found: {}", path.display()));
		}

	}

}

/// check if a file exists
pub fn exists(path: impl AsRef<Path>) -> bool {
	return bundled_path(path).is_ok();
}

/// get files that matches a glob pattern (e.g. `glob("img/*.png")`)
pub fn glob(pat: &str) -> Result<Vec<PathBuf>> {

	let listings = glob::glob(&pat.to_string())
		.ok()
		.or_else(|| glob::glob(&format!("{}/{}", res_dir()?.display(), pat)).ok())
		.ok_or(format!("failed to execute glob pattern {}", pat))?
		.flatten()
		.collect();

	return Ok(listings);

}

/// read bytes from a file
pub fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {

	let path = path.as_ref();
	let path = bundled_path(path)?;

	return fs::read(&path)
		.map_err(|_| format!("failed to read file {}", path.display()));

}

/// read text from a file
pub fn read_str(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();
	let path = bundled_path(path)?;

	return fs::read_to_string(&path)
		.map_err(|_| format!("failed to read file {}", path.display()));

}

/// get directory listing
pub fn read_dir(path: impl AsRef<Path>) -> Result<std::fs::ReadDir> {

	let path = path.as_ref();
	let path = bundled_path(path)?;

	return fs::read_dir(&path)
		.map_err(|_| format!("failed to read dir {}", path.display()));

}

/// get file basename
pub fn basename(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();
	let path = bundled_path(path)?;

	return Ok(
		Path::new(&path)
			.file_stem()
			.ok_or(format!("failed to get basename: {}", path.display()))?
			.to_str()
			.ok_or(format!("failed to get basename: {}", path.display()))?
			.to_owned()
	);

}

/// get file extension
pub fn extname(path: impl AsRef<Path>) -> Result<String> {

	let path = path.as_ref();

	return Ok(path
		.extension()
		.ok_or(format!("failed to get extname: {}", path.display()))?
		.to_os_string()
		.into_string()
		.map_err(|_| format!("failed to get extname: {}", path.display()))?
	);

}

/// copy file
pub fn copy(p1: impl AsRef<Path>, p2: impl AsRef<Path>) -> Result<u64> {

	let p1 = p1.as_ref();
	let p2 = p2.as_ref();

	return fs::copy(p1, p2)
		.map_err(|_| format!("failed to copy {} to {}", p1.display(), p2.display()));

}

/// create directory
pub fn mkdir(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::create_dir_all(path)
		.map_err(|_| format!("failed to create directory {}", path.display()));

}

/// check if a path is a file
pub fn is_file(path: impl AsRef<Path>) -> bool {
	return path.as_ref().is_file();
}

/// check if a path is a directory
pub fn is_dir(path: impl AsRef<Path>) -> bool {
	return path.as_ref().is_dir();
}

/// remove file
pub fn remove(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::remove_file(path)
		.map_err(|_| format!("failed to remove file {}", path.display()));

}

/// remove directory
pub fn remove_dir(path: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::remove_dir(path)
		.map_err(|_| format!("failed to remove directory {}", path.display()));

}

/// rename file
pub fn rename(path: impl AsRef<Path>, new: impl AsRef<Path>) -> Result<()> {

	let path = path.as_ref();

	return fs::rename(path, new)
		.map_err(|_| format!("failed to rename {}", path.display()));

}

/// write to a file
pub fn write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {

	let path = path.as_ref();

	return fs::write(path, content)
		.map_err(|_| format!("failed to write file {}", path.display()));

}

/// get file size
pub fn size(path: impl AsRef<Path>) -> Result<u64> {

	let path = path.as_ref();
	let len = fs::metadata(path)
		.map_err(|_| format!("failed to read file {}", path.display()))?
		.len();

	return Ok(len);

}

/// get system data dir
pub fn data_dir() -> Option<PathBuf> {
	return dirs_next::data_dir();
}

/// get system home dir
pub fn home_dir() -> Option<PathBuf> {
	return dirs_next::home_dir();
}

