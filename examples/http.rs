// wengwengweng

use dirty::*;
use http::Response;
use http::ContentType;

fn main() {

	http::serve("localhost", 7878, |req| {
		return Response::new(ContentType::Text, "yo");
	});

}

