// wengwengweng

use dirty::http;
use http::Response;
use http::ResponseExt;

fn main() {

	http::serve(("localhost", 8000), |req| {
		return Response::text("yo");
	});

}

