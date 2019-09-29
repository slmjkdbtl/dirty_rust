// wengwengweng

#[cfg(not(feature = "json"))]
compile_error!("ase requires json feature");

use std::path::Path;
use std::collections::HashMap;

use aseprite::SpritesheetData;

use crate::math::*;
use crate::json;
use crate::Result;

///! Load Aseprite Spritesheets

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// anim direction
pub enum AnimDir {
	/// forward
	Forward,
	/// reverse
	Reverse,
	/// pingpong
	PingPong,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// anim
pub struct Anim {

	/// from frame
	pub from: usize,
	/// to frame
	pub to: usize,
	/// direction
	pub dir: AnimDir,

}

/// sprite data
pub struct SpriteData {

	/// frames
	pub frames: Vec<Quad>,
	/// anims
	pub anims: HashMap<String, Anim>,

}

impl SpriteData {

	pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {

		#[cfg(feature = "fs")]
		let json = crate::fs::read_str(path);

		#[cfg(not(feature = "fs"))]
		let json = std::fs::read_to_string(path);

		return Self::from_json(&json?);

	}

	pub fn from_json(json: &str) -> Result<Self> {

		let mut frames = vec![];
		let mut anims = HashMap::new();
		let data: SpritesheetData = json::decode(json)?;

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

				let dir = match anim.direction {
					aseprite::Direction::Forward => AnimDir::Forward,
					aseprite::Direction::Reverse => AnimDir::Reverse,
					aseprite::Direction::Pingpong => AnimDir::PingPong,
				};

				anims.insert(anim.name, Anim {
					from: anim.from as usize,
					to: anim.to as usize,
					dir: dir,
				});

			}

		}

		return Ok(SpriteData {
			frames: frames,
			anims: anims,
		});

	}

}

