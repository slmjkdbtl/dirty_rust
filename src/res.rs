// wengwengweng

use std::collections::HashMap;

use crate::*;
use crate::math::*;

ctx!(RES: ResCtx);

struct ResCtx {
	sprites: HashMap<&'static str, SpriteData>,
}

pub fn init() {

	init_ctx(ResCtx {
		sprites: HashMap::new(),
	});

}

pub struct SpriteData {

	pub tex: gfx::Texture,
	pub frames: Vec<Rect>,

}

pub fn load_sprites(dir: &str, names: Vec<&'static str>) {

	let res_mut = get_ctx_mut();

	for name in names {

		let img_path = &format!("{}{}.png", dir, name);
		let json_path = &format!("{}{}.json", dir, name);
		let data = fs::file_read(img_path);
		let tex = gfx::Texture::from_bytes(&data);
		let mut frames = vec![];

		if fs::file_exists(json_path) {
			// ...
		} else {
			frames = vec![rect!(0, 0, 1, 1)];
		}

		res_mut.sprites.insert(name, SpriteData {
			tex: tex,
			frames: frames,
		});

	}

}

// pub fn load_all_sprites(dir: &str) {

// 	let files = fs::read_dir(dir).unwrap();

// 	for file in files {
// 	}

// }

pub fn get_sprite(name: &str) -> &SpriteData {
	return &get_ctx().sprites[name];
}

