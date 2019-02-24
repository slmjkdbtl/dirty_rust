// wengwengweng

//! Save & Load User Data

use serde::ser::Serialize;
use serde::de::Deserialize;

use crate::*;

ctx!(PREF: PrefCtx);

struct PrefCtx {
	dir: String,
}

/// initialize pref
pub fn init(org: &str, name: &str) {

	if !app::enabled() {
		panic!("can't init data without app");
	}

	let dir = sdl2::filesystem::pref_path(&org, &name).expect("failed to get pref dir");

	ctx_init(PrefCtx {
		dir: dir,
	});

}

/// save json data
pub fn save<D: Serialize>(fname: &str, data: D) {

	let dir = &ctx_get().dir;
	let path = format!("{}{}", dir, fname);
	let string = serde_json::to_string(&data).expect("failed to serialize json");

	std::fs::write(&path, string).expect(&format!("failed to write {}", path));

}

/// get json data
pub fn get<D: for<'a> Deserialize<'a>>(fname: &str) -> D {

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

