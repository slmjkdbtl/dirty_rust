// wengwengweng

use super::*;

pub struct Router {
	handlers: Vec<Box<Fn(&Request) -> Option<Response>>>,
}

unsafe impl Send for Router {}

impl Router {

	pub fn new() -> Self {
		return Self {
			handlers: Vec::new(),
		};
	}

	pub fn get<D: AsRef<[u8]>, F: Fn() -> D + 'static>(&mut self, pat: &str, f: F) {
		self.handlers.push(Box::new(move |req| {
			f();
			return None;
		}));
	}

}

