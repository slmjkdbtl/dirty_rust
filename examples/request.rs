// wengwengweng

use dirty::http;

fn main() -> Result<(), String> {

	let url = "http://www.wordsound.com";

	let res = http::get(url)?
		.send()?;

	println!("{}", res.into_body().into_string());

	return Ok(());

}
