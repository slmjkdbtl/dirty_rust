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

struct SpriteData {

	tex: gfx::Texture,
	frames: Vec<Rect>,

}

pub fn load_sprites(dir: &str, names: Vec<&str>) {

	let res_mut = get_ctx_mut();

	for name in names {

		let img_path = format!("{}/{}.png", dir, name);
		let json_path = format!("{}/{}.json", dir, name);

		if fs::file_exists(&json_path[..]) {
			// ...
		} else {
			// ...
		}

// 		res_mut.sprites.insert(name, SpriteData {
// 			tex: tex,
// 			frames: frames,
// 		});

	}

}

// pub fn load_all_sprites(dir: &str) {

// 	let files = fs::read_dir(dir).unwrap();

// 	for file in files {
// 	}

// }

// pub fn get_sprite(name: &str) {
// }

