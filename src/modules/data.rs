// wengwengweng

//! Save & Load User Data

use serde::ser;
use serde::de;
pub use serde::Serialize;
pub use serde::Deserialize;
use gctx::ctx;

use crate::*;

ctx!(DATA: DataCtx);

struct DataCtx {
	dir: String,
}

/// initialize pref
pub fn init(org: &str, name: &str) {

	let dir = sdl2::filesystem::pref_path(&org, &name).expect("failed to get pref dir");

	ctx_init(DataCtx {
		dir: dir,
	});

}

/// save json data
pub fn save<D: ser::Serialize>(fname: &str, data: D) {

	let dir = &ctx_get().dir;
	let path = format!("{}{}", dir, fname);
	let string = serde_json::to_string(&data).expect("failed to serialize json");

	std::fs::write(&path, string).expect(&format!("failed to write {}", path));

}

/// get json data
pub fn get<D: for<'a> de::Deserialize<'a>>(fname: &str) -> D {

	let dir = &ctx_get().dir;
	let path = format!("{}{}", dir, fname);
	let content = fs::read_str(&path);
	let data: D = serde_json::from_str(&content).expect("failed to parse json");

	return data;

}

/// check if a data file exists
pub fn exists(fname: &str) -> bool {

	let dir = &ctx_get().dir;
	let path = format!("{}{}", dir, fname);

	return fs::exists(&path);

}

