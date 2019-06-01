// wengwengweng

use dirty::*;
use http::Response;
use http::Method;

fn main() {

	let mut server = http::server("127.0.0.1", 7878);

	server.statics("/", "res/");

	server.get("/", || {
		return include_bytes!("res/index.html").to_vec();
	});

	server.serve();

}

