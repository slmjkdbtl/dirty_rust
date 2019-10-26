// wengwengweng

use std::path::Path;
use std::io::Cursor;
use std::collections::HashMap;

use crate::math::*;
use crate::Result;

///! Load Aseprite Spritesheets

#[derive(Copy, Clone, Debug)]
/// anim
pub struct Anim {

	/// from frame
	pub from: usize,
	/// to frame
	pub to: usize,

}

/// sprite data
pub struct SpriteData {

	/// frames
	pub frames: Vec<Quad>,
	/// anims
	pub anims: HashMap<String, Anim>,

}

impl SpriteData {

	#[cfg(feature = "json")]
	pub fn from_json(json: &str) -> Result<Self> {

		use crate::codec::json;

		let mut frames = vec![];
		let mut anims = HashMap::new();
		let data: ase_json::SpritesheetData = json::decode(json)?;

		let width = data.meta.size.w;
		let height = data.meta.size.h;

		for f in data.frames {

			frames.push(Quad::new(
				f.frame.x as f32 / width as f32,
				f.frame.y as f32 / height as f32,
				f.frame.w as f32 / width as f32,
				f.frame.h as f32 / height as f32
			));

		}

		if let Some(frame_tags) = data.meta.frame_tags {

			for anim in frame_tags {

				let mut from = anim.from;
				let mut to = anim.to;

				if let ase_json::Direction::Reverse = anim.direction {
					std::mem::swap(&mut from, &mut to);
				}

				anims.insert(anim.name, Anim {
					from: from as usize,
					to: to as usize,
				});

			}

		}

		return Ok(SpriteData {
			frames: frames,
			anims: anims,
		});

	}

}

pub use ase_bin::Aseprite;

fn parse(b: &[u8]) -> Result<Aseprite> {
	return Ok(ase_bin::Aseprite::from_read(&mut Cursor::new(b))?);
}

