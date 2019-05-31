// wengwengweng

use dirty::*;

fn main() {

	let res = http::get("https://www.example.org").expect("network error");
	println!("{}", res.text());

}

