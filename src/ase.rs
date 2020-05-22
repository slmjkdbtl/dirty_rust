// wengwengweng

//! Parse Aseprite Spritesheet Data

use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;

use crate::Result;
use crate::math::*;

#[derive(Serialize, Deserialize)]
struct SpritesheetData {
	frames: Vec<Frame>,
	meta: Metadata,
}

#[derive(Serialize, Deserialize)]
struct Frame {
	frame: Rect,
}

#[derive(Serialize, Deserialize)]
struct Rect {
	x: u32,
	y: u32,
	w: u32,
	h: u32,
}

#[derive(Serialize, Deserialize)]
struct Dimensions {
	w: u32,
	h: u32,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
	size: Dimensions,
	#[serde(rename = "frameTags")]
	frame_tags: Option<Vec<FrameTag>>,
}

#[derive(Serialize, Deserialize)]
struct FrameTag {
	name: String,
	from: u32,
	to: u32,
	direction: Direction,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum Direction {
	#[serde(rename="forward")]
	Forward,
	#[serde(rename="reverse")]
	Reverse,
	#[serde(rename="pingpong")]
	Pingpong,
}

#[derive(Copy, Clone, Debug)]
pub struct Anim {
	pub from: usize,
	pub to: usize,
}

#[derive(Clone, Debug)]
pub struct SpriteData {
	pub frames: Vec<Quad>,
	pub anims: HashMap<String, Anim>,
}

#[cfg(feature = "json")]
pub fn parse(json: &str) -> Result<SpriteData> {

	use crate::codec::json;

	let mut anims = HashMap::new();
	let data: SpritesheetData = json::decode(json)?;

	let width = data.meta.size.w;
	let height = data.meta.size.h;

	let frames = data.frames
		.iter()
		.map(|f| {
			return Quad::new(
				f.frame.x as f32 / width as f32,
				f.frame.y as f32 / height as f32,
				f.frame.w as f32 / width as f32,
				f.frame.h as f32 / height as f32
			);
		})
		.collect::<Vec<Quad>>();

	if let Some(frame_tags) = data.meta.frame_tags {

		for anim in frame_tags {

			let mut from = anim.from;
			let mut to = anim.to;

			if let Direction::Reverse = anim.direction {
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

