// wengwengweng

use std::collections::HashMap;

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

	from: u8,
	to: u8,
	dir: AnimDir,

}

pub struct SpriteData {

	pub tex: gfx::Texture,
	pub frames: Vec<Rect>,
	pub anims: HashMap<String, Anim>,

}

pub fn load_all_sprites(path: &str) {
	// ...
}

pub fn load_sprite(name: &'static str, img: &[u8], json: &str) {

	let res_mut = ctx_get_mut();
	let tex = gfx::Texture::from_bytes(&img);
	let (width, height) = (tex.width as f32, tex.height as f32);
	let mut frames = vec![];
	let mut anims = HashMap::new();

	if let Ok(data) = json::parse(json) {

		for i in data["frames"].members() {

			let frame = &i["frame"];
			let x = frame["x"].as_f32().unwrap();
			let y = frame["y"].as_f32().unwrap();
			let w = frame["w"].as_f32().unwrap();
			let h = frame["h"].as_f32().unwrap();

			frames.push(rect!(x / width, y / height, w / width, h / height));

		}

		for i in data["meta"]["frameTags"].members() {

			let label = i["name"].as_str().unwrap();
			let from = i["from"].as_u8().unwrap();
			let to = i["to"].as_u8().unwrap();

			let dir = match i["direction"].as_str().unwrap() {
				"forward" => AnimDir::Forward,
				"reverse" => AnimDir::Reverse,
				"pingpong" => AnimDir::PingPong,
				_ => AnimDir::Forward,
			};

			anims.insert(String::from(label), Anim {
				from: from,
				to: to,
				dir: dir,
			});

		}

	} else {

		frames = vec![rect!(0, 0, 1, 1)];

	}

	let data = SpriteData {
		tex: tex,
		frames: frames,
		anims: anims,
	};

	res_mut.sprites.insert(name, data);

}

pub fn get_sprite(name: &str) -> &SpriteData {
	return &ctx_get().sprites[name];
}

pub fn load_sound(name: &'static str, data: &'static [u8]) {

	let res_mut = ctx_get_mut();

	res_mut.sounds.insert(name, audio::Track::from_bytes(data));

}

pub fn get_sound(name: &str) -> &audio::Track {
	return &ctx_get().sounds[name];
}

