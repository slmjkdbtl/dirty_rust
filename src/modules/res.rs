// wengwengweng

//! Resource Loading

use std::collections::HashMap;

use aseprite::SpritesheetData;
use gctx::ctx;

use crate::*;
use crate::math::*;

ctx!(RES: ResCtx);

struct ResCtx {

	textures: HashMap<String, gfx::Texture>,
	sounds: HashMap<String, audio::Sound>,
	spritedata: HashMap<String, SpriteData>,

}

/// initialize res module
pub fn init() {

	ctx_init(ResCtx {
		textures: HashMap::new(),
		sounds: HashMap::new(),
		spritedata: HashMap::new(),
	});

}

/// check if res is initialized
pub fn enabled() -> bool {
	return ctx_ok();
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

	/// frames
	pub frames: Vec<Rect>,
	/// anims
	pub anims: HashMap<String, Anim>,

}

impl Default for SpriteData {
	fn default() -> Self {
		return Self {
			frames: vec![rect![0, 0, 1, 1]],
			anims: HashMap::new(),
		};
	}
}

/// load all sprites from given directory
pub fn load_all_textures_under(dir: &str) {

	let files: Vec<String> = fs::glob(&format!("{}*.png", dir))
		.into_iter()
		.map(|f| fs::basename(&f))
		.collect();

	return load_textures_under(dir, &files);

}

/// load all sounds from given directory
pub fn load_all_sounds_under(dir: &str) {

	let files: Vec<String> = fs::glob(&format!("{}*.png", dir))
		.into_iter()
		.map(|f| fs::basename(&f))
		.collect();

	return load_sounds_under(dir, &files);

}

/// load all spritedata from given directory
pub fn load_all_spritedata_under(dir: &str) {

	let files: Vec<String> = fs::glob(&format!("{}*.json", dir))
		.into_iter()
		.map(|f| fs::basename(&f))
		.collect();

	return load_spritedata_under(dir, &files);

}

/// load all textures from given directory with given names
pub fn load_textures_under<T: AsRef<str>>(dir: &str, names: &[T]) {

	for name in names {
		let name = name.as_ref();
		load_texture(name, &fs::read_bytes(&format!("{}{}.png", dir, name)));
	}

}

/// load all sounds from given directory with given names
pub fn load_sounds_under<T: AsRef<str>>(dir: &str, names: &[T]) {

	for name in names {
		let name = name.as_ref();
		load_sound(name, &fs::read_bytes(&format!("{}{}.ogg", dir, name)));
	}

}

/// load all sprite data from given directory with given names
pub fn load_spritedata_under<T: AsRef<str>>(dir: &str, names: &[T]) {

	for name in names {
		let name = name.as_ref();
		load_spritedata(name, &fs::read_str(&format!("{}{}.json", dir, name)));
	}

}

/// load a sprite data with json string
pub fn load_spritedata(
	name: &str,
	json: &str) {

	let res_mut = ctx_mut();

	let mut frames = vec![];
	let mut anims = HashMap::new();
	let data: SpritesheetData = serde_json::from_str(json).expect("failed to parse json");

	let width = data.meta.size.w;
	let height = data.meta.size.h;

	for f in data.frames {

		frames.push(rect!(
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

	res_mut.spritedata.insert(name.to_owned(), SpriteData {
		frames: frames,
		anims: anims,
	});

}

/// load a sound with raw data
pub fn load_texture(
	name: &str,
	data: &[u8]) {

	let res_mut = ctx_mut();

	res_mut.textures.insert(name.to_owned(), gfx::Texture::from_bytes(data));

}

/// load a sound with raw data
pub fn load_sound(
	name: &str,
	data: &[u8]) {

	let res_mut = ctx_mut();

	res_mut.sounds.insert(name.to_owned(), audio::Sound::from_bytes(data));

}

/// get the sprite data that is loaded with given name
pub fn spritedata(name: &str) -> &SpriteData {
	return ctx_get()
		.spritedata
		.get(name)
		.unwrap_or_else(|| panic!("failed to get sprite data {}", name));
}

/// get the texture that is loaded with given name
pub fn texture(name: &str) -> &gfx::Texture {
	return ctx_get()
		.textures
		.get(name)
		.unwrap_or_else(|| panic!("failed to get texture {}", name));
}

/// get the sound that is loaded with given name
pub fn sound(name: &str) -> &audio::Sound {
	return ctx_get()
		.sounds
		.get(name)
		.unwrap_or_else(|| panic!("failed to get sound {}", name));
}

