# DIRTY
simple toolkit for creating game-like experiences

### usage
add to `Cargo.toml`
```toml
[dependencies.dirty]
git = "https://github.com/slmjkdbtl/DIRTY"
```
not on crates.io yet due to duplicate crate name

### example
```rust
use dirty::*;

fn main() {

	// initialize modules
	app::init();
	window::init("yo", 640, 480);
	audio::init();
	res::init();

	// load resources
	res::load_sprite("yo", include_bytes!("yo.png"));
	res::load_sound("pop", include_bytes!("pop.ogg"));

	// main loop
	app::run(&mut || {

		// clear view
		gfx::clear();

		// transforms
		gfx::push();
		gfx::translate(vec2!(120, 120));
		gfx::scale(vec2!(4));
		gfx::color(color!(0, 0, 1, 1));

		// draw text
		gfx::text("yo");
		gfx::pop();

		gfx::push();
		gfx::translate(vec2!(240, 240));
		gfx::color(color!(1));

		// draw texture
		gfx::draw(&res::sprite("yo").tex, rect!(0, 0, 1, 1));
		gfx::pop();

		// input
		if window::key_released(Key::Space) {
			audio::sound(res::sound("pop"));
		}

		if window::key_pressed(Key::F) {
			window::set_fullscreen(!window::get_fullscreen())
		}

		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}
```
more under `examples/`

### notes

- this lib is more opinionated than idiomatic, upon initialization each module has its hidden `static` state that dies when program ends
- `app::init()` also redefines the panic behavior (also display messages to screen if `window` is initiated)
- currently using OpenGL 2.0 to support legacy machines, might change to gfx-rs in the future
- planning on supporting various scripting tools, currently doing lua and ketos, help wanted

