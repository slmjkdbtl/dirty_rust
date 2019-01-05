// wengwengweng

//! Handles resource loading

use std::collections::HashMap;

use aseprite::SpritesheetData;

use crate::*;
use crate::math::*;

ctx!(RES: ResCtx);

struct ResCtx {
	sprites: HashMap<&'static str, SpriteData>,
	sounds: HashMap<&'static str, audio::Track>,
}

pub fn init() {

	ctx_init(ResCtx {
		sprites: HashMap::new(),
		sounds: HashMap::new(),
	});

}

pub fn enabled() -> bool {
	return ctx_is_ok();
}

#[derive(Debug)]
pub enum AnimDir {
	Forward,
	Reverse,
	PingPong,
}

#[derive(Debug)]
pub struct Anim {

	from: u32,
	to: u32,
	dir: AnimDir,

}

pub struct SpriteData {

	pub tex: gfx::Texture,
	pub frames: Vec<Rect>,
	pub anims: HashMap<String, Anim>,

}

pub fn load_spritesheet(name: &'static str, img: &[u8], json: &str) {

	let res_mut = ctx_get_mut();
	let tex = gfx::Texture::from_bytes(&img);
	let (width, height) = (tex.width as f32, tex.height as f32);
	let mut frames = vec![];
	let mut anims = HashMap::new();
	let data: SpritesheetData = serde_json::from_str(json).unwrap();

	for f in data.frames {

		frames.push(rect!(
			f.frame.x as f32 / width,
			f.frame.y as f32 / height,
			f.frame.w as f32 / width,
			f.frame.h as f32 / height
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
				from: anim.from,
				to: anim.to,
				dir: dir,
			});

		}

	}

	let data = SpriteData {
		tex: tex,
		frames: frames,
		anims: anims,
	};

	res_mut.sprites.insert(name, data);

}

pub fn sprite(name: &str) -> &SpriteData {
	return &ctx_get().sprites[name];
}

pub fn load_sprite(name: &'static str, img: &[u8]) {

	let res_mut = ctx_get_mut();
	let tex = gfx::Texture::from_bytes(&img);
	let (width, height) = (tex.width as f32, tex.height as f32);
	let mut frames = vec![rect!(0, 0, 1, 1)];
	let mut anims = HashMap::new();

	let data = SpriteData {
		tex: tex,
		frames: frames,
		anims: anims,
	};

	res_mut.sprites.insert(name, data);

}

pub fn load_sound(name: &'static str, data: &'static [u8]) {

	let res_mut = ctx_get_mut();

	res_mut.sounds.insert(name, audio::Track::from_bytes(data));

}

pub fn sound(name: &str) -> &audio::Track {
	return &ctx_get().sounds[name];
}

