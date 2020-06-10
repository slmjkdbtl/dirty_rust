// wengwengweng

//! File Systems Function Wrappers That Works with App Bundles
//!
//! All functions here uses [`res_dir`](res_dir()) so that it can read file from app bundle dirs (e.g. on macOS `*.app/Content/Resources`)
//!
//! There're also extra utilitiy functions like [`glob`](glob())

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
	let path = bundled_path(path)?;

	return Ok(path
		.extension()
		.ok_or(format!("failed to get extname: {}", path.display()))?
		.to_os_string()
		.into_string()
		.map_err(|_| format!("failed to get extname: {}", path.display()))?
	);

}

