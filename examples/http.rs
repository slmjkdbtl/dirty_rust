// wengwengweng

use dirty::*;
use http::Response;
use http::ContentType;
use http::Status;

fn main() {

	http::serve("0.0.0.0", 7878, |req| {
		return Response::new(Status::Ok, ContentType::Text, "yo");
	});

}

