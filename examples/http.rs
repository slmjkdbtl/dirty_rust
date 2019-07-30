// wengwengweng

use dirty::*;

fn main() {

	let url = "https://www.example.com";

	match http::get(url) {
		Ok(res) => println!("{}", res.text()),
		Err(err) => {
			dbg!(err);
		},
	};

 }
