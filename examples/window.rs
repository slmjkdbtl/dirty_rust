// wengwengweng

use dirty::*;
use dirty::window::Key;

fn main() {

	let mut win = window::Window::new(window::Conf::default()).expect("failed to create window");

	win.run(|ctx| {

		if ctx.key_pressed(Key::F) {
			ctx.toggle_fullscreen();
		}

		if ctx.key_pressed(Key::Escape) {
			ctx.close();
		}

	}).expect("failed to run window");

}

