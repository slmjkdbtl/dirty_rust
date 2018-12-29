// wengwengweng

use std::fs;
use crate::ctx;
use crate::gfx;
use crate::math;

ctx!(RES: ResCtx);

struct ResCtx {
	sprites: Vec<SpriteData>,
}

pub fn init() {

	init_ctx(ResCtx {
		sprites: vec![],
	});

}

struct SpriteData {
	tex: gfx::Texture,
	frames: Vec<math::Vector4>,
}

pub fn load_sprites(dir: &str, names: Vec<&str>) {

	for name in names {
		// ...
	}

}

pub fn load_all_sprites(dir: &str) {

	let files = fs::read_dir(dir).unwrap();

	for file in files {
		// ...
	}

}

pub fn get_sprite(name: &str) {
	// ...
}

