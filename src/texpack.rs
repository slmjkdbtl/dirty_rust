// wengwengweng

use guillotiere::*;

use crate::Result;
use crate::Error;

pub struct Part {
	part: Allocation,
}

pub struct Atlas {
	atlas: AtlasAllocator,
}

impl Atlas {

	pub fn new(width: i32, height: i32) -> Self {
		return Self {
			atlas: AtlasAllocator::new(size2(width as i32, height as i32)),
		};
	}

	pub fn add(&mut self, w: i32, h: i32) -> Result<Part> {

		let part = self.atlas.allocate(size2(w, h)).ok_or(Error::TexPack)?;

		return Ok(Part {
			part: part,
		});

	}

	pub fn remove(&mut self, part: &Part) {
		self.atlas.deallocate(part.part.id);
	}

}

