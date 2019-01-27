# DIRTY
simple toolkit for creating game-like experiences

### usage
add to `Cargo.toml`
```toml
[dependencies.dirty]
git = "https://github.com/slmjkdbtl/DIRTY"
```
not on crates.io yet due to duplicate crate name

### modules
- **app** Lifecycles, Time and Errors
- **window** Window & Events
- **gfx** 2D Rendering
- **g3d** 3D Rendering
- **audio** Sound loading & playback
- **math** Common Math Functions & Structs
- **fs** Common File Related Functions
- **ecs** Simple ECS
- **net** Networking

### example
```rust
use dirty::*;

fn main() {

	// init
	app::init();
	window::init("yo", 640, 480);
	audio::init();

	// main loop
	app::run(|| {

		// transform
		gfx::push();
		gfx::translate(vec2!(220, 120));
		gfx::scale(vec2!(12));
		gfx::color(color!(0, 1, 1, 1));

		// draw text
		gfx::text("yo");
		gfx::pop();

		// input
		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}
```
more under `examples/`

### notes & caveats

- upon initialization each module has its hidden `static` state that dies when program ends
- `app::init()` also overrides default panic behavior (also display messages to screen if `window` is initiated)
- currently using OpenGL 2.1 for better compatibility, but might change to gfx-hal in the future
- currently no proper error handling (a lot of internal panics, but with pretty error screen)
- on Windows, `audio::init()` must be called before `window::init()`, for some reason it crashes if tried to get audio device after SDL initialization
- planning on adding scripting support

