// wengwengweng

use std::collections::HashMap;

use crate::*;
use crate::math::*;

ctx!(RES: ResCtx);

struct ResCtx {
	sprites: HashMap<String, SpriteData>,
}

pub fn init() {

	init_ctx(ResCtx {
		sprites: HashMap::new(),
	});

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

pub fn load_sprite(name: &'static str, img: &[u8], json: &str) {

	let res_mut = get_ctx_mut();
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

	res_mut.sprites.insert(String::from(name), SpriteData {
		tex: tex,
		frames: frames,
		anims: anims,
	});

}

pub fn get_sprite(name: &str) -> &SpriteData {
	return &get_ctx().sprites[name];
}

