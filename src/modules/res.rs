// wengwengweng

//! Resource Loading

use std::path::Path;
use std::collections::HashMap;

use aseprite::SpritesheetData;
use gctx::*;

use crate::*;
use crate::math::*;

ctx!(RES: Res);

pub struct Res {

	textures: HashMap<String, gfx::Texture>,
	sounds: HashMap<String, audio::Sound>,
	spritedata: HashMap<String, SpriteData>,

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

impl Res {

	pub fn new() -> Self {
		return Self {
			textures: HashMap::new(),
			sounds: HashMap::new(),
			spritedata: HashMap::new(),
		};
	}

	/// load all sprites from given directory
	pub fn load_all_textures_under(&mut self, dir: impl AsRef<Path>) {

		let dir = dir.as_ref();

		let files: Vec<String> = fs::glob(&format!("{}/*.png", dir.display()))
			.into_iter()
			.map(|f| fs::basename(&f))
			.collect();

		return self.load_textures_under(dir, &files
			.iter()
			.map(|f| f.as_ref())
			.collect::<Vec<&str>>());

	}

	/// load all sounds from given directory
	pub fn load_all_sounds_under(&mut self, dir: impl AsRef<Path>) {

		let dir = dir.as_ref();

		let files: Vec<String> = fs::glob(&format!("{}/*.png", dir.display()))
			.into_iter()
			.map(|f| fs::basename(&f))
			.collect();

		return self.load_sounds_under(dir, &files
			.iter()
			.map(|f| f.as_ref())
			.collect::<Vec<&str>>());

	}

	/// load all spritedata from given directory
	pub fn load_all_spritedata_under(&mut self, dir: impl AsRef<Path>) {

		let dir = dir.as_ref();

		let files: Vec<String> = fs::glob(&format!("{}/*.json", dir.display()))
			.into_iter()
			.map(|f| fs::basename(&f))
			.collect();

		return self.load_spritedata_under(dir, &files
			.iter()
			.map(|f| f.as_ref())
			.collect::<Vec<&str>>());

	}

	/// load all textures from given directory with given names
	pub fn load_textures_under(&mut self, dir: impl AsRef<Path>, names: &[&str]) {

		let dir = dir.as_ref();

		for name in names {
			let name = name.as_ref();
			self.load_texture(name, &fs::read_bytes(dir.join(&format!("{}.png", name))));
		}

	}

	/// load all sounds from given directory with given names
	pub fn load_sounds_under(&mut self, dir: impl AsRef<Path>, names: &[&str]) {

		let dir = dir.as_ref();

		for name in names {
			let name = name.as_ref();
			self.load_sound(name, &fs::read_bytes(dir.join(&format!("{}.ogg", name))));
		}

	}

	/// load all sprite data from given directory with given names
	pub fn load_spritedata_under(&mut self, dir: impl AsRef<Path>, names: &[&str]) {

		let dir = dir.as_ref();

		for name in names {
			let name = name.as_ref();
			self.load_spritedata(name, &fs::read_str(dir.join(&format!("{}.json", name))));
		}

	}

	/// load a sprite data with json string
	pub fn load_spritedata(
		&mut self,
		name: &str,
		json: &str) {

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

		self.spritedata.insert(name.to_owned(), SpriteData {
			frames: frames,
			anims: anims,
		});

	}

	/// load a sound with raw data
	pub fn load_texture(
		&mut self,
		name: &str,
		data: &[u8]) {

		self.textures.insert(name.to_owned(), gfx::Texture::from_bytes(data));

	}

	/// load a sound with raw data
	pub fn load_sound(
		&mut self,
		name: &str,
		data: &[u8]) {

		self.sounds.insert(name.to_owned(), audio::Sound::from_bytes(data));

	}

	/// get the sprite data that is loaded with given name
	pub fn spritedata(&self, name: &str) -> &SpriteData {
		return self
			.spritedata
			.get(name)
			.unwrap_or_else(|| panic!("failed to get sprite data {}", name));
	}

	/// get the texture that is loaded with given name
	pub fn texture(&self, name: &str) -> &gfx::Texture {
		return self
			.textures
			.get(name)
			.unwrap_or_else(|| panic!("failed to get texture {}", name));
	}

	/// get the sound that is loaded with given name
	pub fn sound(&self, name: &str) -> &audio::Sound {
		return self
			.sounds
			.get(name)
			.unwrap_or_else(|| panic!("failed to get sound {}", name));
	}

}

/// initialize res module
pub fn init() {
	ctx_init!(RES, Res::new());
}

/// check if res is initialized
pub fn enabled() -> bool {
	return ctx_ok!(RES);
}

expose!(RES(mut), load_all_spritedata_under(dir: impl AsRef<Path>));
expose!(RES(mut), load_all_textures_under(dir: impl AsRef<Path>));
expose!(RES(mut), load_all_sounds_under(dir: impl AsRef<Path>));
expose!(RES(mut), load_spritedata_under(dir: impl AsRef<Path>, names: &[&str]));
expose!(RES(mut), load_textures_under(dir: impl AsRef<Path>, names: &[&str]));
expose!(RES(mut), load_sounds_under(dir: impl AsRef<Path>, names: &[&str]));
expose!(RES(mut), load_spritedata(name: &str, json: &str));
expose!(RES(mut), load_texture(name: &str, data: &[u8]));
expose!(RES(mut), load_sound(name: &str, data: &[u8]));
expose!(RES, spritedata(name: &str) -> &'static SpriteData);
expose!(RES, texture(name: &str) -> &'static gfx::Texture);
expose!(RES, sound(name: &str) -> &'static audio::Sound);

