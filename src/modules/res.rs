// wengwengweng

//! Resource Loading

use std::collections::HashMap;

use aseprite::SpritesheetData;

use crate::*;

ctx!(RES: ResCtx);

struct ResCtx {
	sprites: HashMap<String, SpriteData>,
	sounds: HashMap<String, audio::Sound>,
}

/// initialize res module
pub fn init() {

	ctx_init(ResCtx {
		sprites: HashMap::new(),
		sounds: HashMap::new(),
	});

}

/// check if res is initialized
pub fn enabled() -> bool {
	return ctx_is_ok();
}

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

	/// texture
	pub tex: gfx::Texture,
	/// frames
	pub frames: Vec<Rect>,
	/// anims
	pub anims: HashMap<String, Anim>,

}

/// load all sprites from given directory
pub fn load_all_sprites(dir: &str) {

	let files: Vec<String> = fs::glob(&format!("{}*.png", dir))
		.into_iter()
		.map(|f| fs::basename(&f))
		.collect();

	load_sprites(dir, &files);

}

/// load all sounds from given directory
pub fn load_all_sounds(dir: &str) {

	let files: Vec<String> = fs::glob(&format!("{}*.png", dir))
		.into_iter()
		.map(|f| fs::basename(&f))
		.collect();

	load_sounds(dir, &files);

}

/// load all sprites from given directory with given names
pub fn load_sprites<T: AsRef<str>>(dir: &str, names: &[T]) {

	for name in names {

		let name = name.as_ref();
		let img = format!("{}{}.png", dir, name);
		let json = format!("{}{}.json", dir, name);

		if fs::exists(&json) {
			load_spritesheet(name, &fs::read_bytes(&img), &fs::read_str(&json));
		} else {
			load_sprite(name, &fs::read_bytes(&img));
		}

	}

}

/// load all sounds from given directory with given names
pub fn load_sounds<T: AsRef<str>>(dir: &str, names: &[T]) {

	for name in names {
		let name = name.as_ref();
		load_sound(name, &fs::read_bytes(&format!("{}{}.ogg", dir, name)));
	}

}

/// load a sprite with raw image data
pub fn load_sprite(name: &str, img: &[u8]) {

	let res_mut = ctx_get_mut();

	if res_mut.sprites.get(name).is_some() {
		panic!("{} already used", name);
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

/// load a spritesheet with raw image data and json data
pub fn load_spritesheet(name: &str, img: &[u8], json: &str) {

	let res_mut = ctx_get_mut();

	if res_mut.sprites.get(name).is_some() {
		panic!("{} already used", name);
	}

	let tex = gfx::Texture::from_bytes(&img);
	let (width, height) = (tex.width() as f32, tex.height() as f32);
	let mut frames = vec![];
	let mut anims = HashMap::new();
	let data: SpritesheetData = serde_json::from_str(json).expect("failed to parse json");

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
				from: anim.from as usize,
				to: anim.to as usize,
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

/// load a sound with raw data
pub fn load_sound(name: &str, data: &[u8]) {

	let res_mut = ctx_get_mut();

	if res_mut.sounds.get(name).is_some() {
		panic!("{} already used", name);
	}

	res_mut.sounds.insert(name.to_owned(), audio::Sound::from_bytes(data));

}

/// get sprite that is loaded with given name
pub fn sprite(name: &str) -> &SpriteData {
	return &ctx_get().sprites.get(name).unwrap_or_else(|| panic!("failed to get sprite {}", name));
}

/// get sound that is loaded with given name
pub fn sound(name: &str) -> &audio::Sound {
	return &ctx_get().sounds.get(name).unwrap_or_else(|| panic!("failed to get sound {}", name));;
}

