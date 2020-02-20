// wengwengweng

use dirty::http;

fn main() {

	let url = "http://www.wordsound.com";
	let url = "https://www.increpare.com";

	match http::get(url) {
		Ok(res) => println!("{}", res.body().as_text()),
		Err(err) => println!("{}", err),
	};

}
