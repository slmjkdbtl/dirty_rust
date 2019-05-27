// wengwengweng

use dirty::window::Window;
use dirty::window::Key;

fn main() {

	let mut win = Window::default();

	win.run(|ctx| {

		if ctx.key_pressed(Key::F) {
			ctx.toggle_fullscreen();
		}

		if ctx.key_pressed(Key::Escape) {
			ctx.close();
		}

	}).expect("failed to run window");

}

