// wengwengweng

//! Handles resource loading

use std::collections::HashMap;

use aseprite::SpritesheetData;

use crate::*;
use crate::math::*;

ctx!(RES: ResCtx);

struct ResCtx {
	sprites: HashMap<String, SpriteData>,
	sounds: HashMap<String, audio::Sound>,
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

pub fn load_all_sprites(dir: &str) {

	let files: Vec<String> = fs::glob(&format!("{}*.png", dir))
		.iter()
		.map(|f| fs::basename(f))
		.collect();

	load_sprites(dir, files.iter().map(|s| s.as_ref()).collect());

}

pub fn load_all_sounds(dir: &str) {

	let files: Vec<String> = fs::glob(&format!("{}*.ogg", dir))
		.iter()
		.map(|f| fs::basename(f))
		.collect();

	load_sounds(dir, files.iter().map(|s| s.as_ref()).collect());

}

pub fn load_sprites(dir: &str, names: Vec<&str>) {

	for name in names {

		let img = format!("{}{}.png", dir, name);
		let json = format!("{}{}.json", dir, name);

		if fs::exists(&json) {
			load_spritesheet(name, &fs::read_bytes(&img), &fs::read_str(&json));
		} else {
			load_sprite(name, &fs::read_bytes(&img));
		}

	}

}

pub fn load_sounds(dir: &str, names: Vec<&str>) {

	for name in names {
		load_sound(name, &fs::read_bytes(&format!("{}{}.ogg", dir, name)));
	}

}

pub fn load_sprite(name: &str, img: &[u8]) {

	let res_mut = ctx_get_mut();

	if res_mut.sprites.get(name).is_some() {
		app::error(&format!("{} already used", name));
	}

	let tex = gfx::Texture::from_bytes(&img);
	let frames = vec![rect!(0, 0, 1, 1)];
	let anims = HashMap::new();

	let data = SpriteData {
		tex: tex,
		frames: frames,
		anims: anims,
	};

	res_mut.sprites.insert(name.to_owned(), data);

}

pub fn load_spritesheet(name: &str, img: &[u8], json: &str) {

	let res_mut = ctx_get_mut();

	if res_mut.sprites.get(name).is_some() {
		app::error(&format!("{} already used", name));
	}

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

	res_mut.sprites.insert(name.to_owned(), data);

}

pub fn load_sound(name: &str, data: &[u8]) {

	let res_mut = ctx_get_mut();

	if res_mut.sounds.get(name).is_some() {
		app::error(&format!("{} already used", name));
	}

	res_mut.sounds.insert(name.to_owned(), audio::Sound::from_bytes(data));

}

pub fn sprite(name: &str) -> &SpriteData {
	return &ctx_get().sprites[name];
}

pub fn sound(name: &str) -> &audio::Sound {
	return &ctx_get().sounds[name];
}

