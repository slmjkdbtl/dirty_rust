// wengwengweng

use dirty::window;
use dirty::window::Key;

fn main() {

	let mut win = window::Window::default();

	win.run(|ctx| {
		if ctx.key_pressed(Key::F) {
			ctx.toggle_cursor_locked();
		}
		if ctx.key_pressed(Key::Escape) {
			ctx.close();
		}
	});

}

