// wengwengweng

use dirty::http;

fn main() {

	let url = "https://www.example.com";

	match http::get(url) {
		Ok(res) => println!("{}", res.text()),
		Err(err) => println!("{}", err),
	};

}
