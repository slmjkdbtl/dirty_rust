# DIRTY
small toolkit for creating game-like experiences

## usage
add to `Cargo.toml`
```toml
[dependencies.dirty]
git = "https://github.com/slmjkdbtl/DIRTY"
```
not on crates.io yet due to conflicted crate name

## example
```rust
use dirty::*;
use dirty::math::*;

fn main() {

	// initialize modules
	app::init();
	window::init("yo", 640, 480);
	gfx::init();

	// create a texture
	let tex = gfx::Texture::from_bytes!(include_bytes!("yo.png"));

	// main loop
	app::run(&mut || {

		// clear view
		gfx::clear();

		// transforms
		gfx::push();
		gfx::translate(vec2!(120, 120));
		gfx::scale(vec2!(1));
		gfx::color(color!(0, 0, 1, 1));

		// draw text
		gfx::text("yo");
		gfx::pop();

		gfx::push();
		gfx::translate(vec2!(240, 240));
		gfx::color(color!(1));

		// draw texture
		gfx::draw(&tex, rect!(0, 0, 1, 1));
		gfx::pop();

		// input
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

