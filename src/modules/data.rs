// wengwengweng

//! Save & Load User Data

use std::path::Path;
use std::path::PathBuf;

use serde::ser;
use serde::de;
use directories::BaseDirs;
pub use serde::Serialize;
pub use serde::Deserialize;

use crate::*;

pub struct Data {
	dir: PathBuf,
}

impl Data {

	pub fn new(org: &str, name: &str) -> Self {

		let dirs = BaseDirs::new().unwrap();
		let data_dir = dirs.data_dir();
		let org_dir = data_dir.join(org);

		if !org_dir.exists() {
			std::fs::create_dir(&org_dir);
		}

		let proj_dir = org_dir.join(name);

		if !proj_dir.exists() {
			std::fs::create_dir(&proj_dir);
		}

		return Self {
			dir: proj_dir,
		};

	}

	/// save json data
	pub fn save(&self, fname: impl AsRef<Path>, data: impl ser::Serialize) {

		let path = self.dir.join(fname);
		let string = serde_json::to_string(&data).expect("failed to serialize json");

		std::fs::write(&format!("{}", path.display()), string).expect(&format!("failed to write {}", path.display()));

	}

	/// get json data
	pub fn get<D: for<'a> de::Deserialize<'a>>(&self, fname: &str) -> D {

		let path = self.dir.join(fname);
		let content = fs::read_str(&format!("{}", path.display()));
		let data: D = serde_json::from_str(&content).expect("failed to parse json");

		return data;

	}

	/// check if a data file exists
	pub fn exists(&self, fname: impl AsRef<Path>) -> bool {
		return self.dir.join(fname).exists();
	}

}

