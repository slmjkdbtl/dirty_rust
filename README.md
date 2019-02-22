![icon](icon.png)

## DIRTY
personal toolkit for creating game-like experiences

### usage
add to `Cargo.toml`
```toml
[dependencies.dirty]
git = "https://github.com/slmjkdbtl/DIRTY"
```
not on crates.io yet due to duplicate crate name

### modules

core:

- **app** Lifecycles, Time and Errors
- **window** Window & Input
- **gfx** General Rendering
- **g2d** 2D Rendering
- **g3d** 3D Rendering (*TODO*)
- **audio** Sound Loading & playback
- **math** Common Math Functions & Types
- **fs** Common File Related Functions

micro helper libs:

- **res** Resource Loading & Storing
- **pref** Save & Load User Data
- **col** Common Collision Detection

### doc
clone the repo and run
```bash
cargo doc -p dirty --no-deps --open
```

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
		g2d::push();
		g2d::translate(vec2!(220, 120));
		g2d::scale(vec2!(12));
		g2d::color(color!(0, 1, 1, 1));

		// draw text
		g2d::text("yo");
		g2d::pop();

		// input
		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}
```
more under `examples/`

### notes & caveats

- `app::init()` also overrides default panic behavior (also display messages to screen if `window` is initiated)
- module contexts are handled internally
- currently using OpenGL 2.1 for better compatibility, but might use rendy in the future

### fun fact
`DIRTY` is short for **Dangerous Ichthyopolist Reincarnates Tropical Yeti**

