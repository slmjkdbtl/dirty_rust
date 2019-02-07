// wengwengweng

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

pub fn write_json<D: Serialize>(fname: &str, data: D) {

	let dir = &ctx_get().dir;
	let path = format!("{}{}", dir, fname);
	let serialized = serde_json::to_string(&data).expect("failed to serialize json");

}

pub fn get_json<'a, D: Deserialize<'a>>(fname: &str) -> D {

	let dir = &ctx_get().dir;
	let path = format!("{}{}", dir, fname);
	let content = fs::read_str(&path);
	let data: D = serde_json::from_str("").expect("failed to parse json");

	return data;

}

