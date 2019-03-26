// wengwengweng

use dirty::*;
use dirty::data::Serialize;
use dirty::data::Deserialize;

// the data struct has to implement Serialize and Deserialize
#[derive(Debug, Serialize, Deserialize)]
struct Duck {
	name: String,
	size: usize,
}

fn main() {

	// init with organization name and app name
	data::init("dirty", "test");

	let d = Duck {
		name: "Jack".to_owned(),
		size: 12,
	};

	// save data to a file
	data::save("data.json", &d);

	// get data from file
	let mut d: Duck = data::get("data.json");

	dbg!(&d);

	// change data
	d.size = 100000;
	data::save("data.json", &d);
	dbg!(data::get::<Duck>("data.json"));

}

