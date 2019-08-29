![icon](icon.png)

## DIRTY
personal toolkit for things

### modules
- **app** Windowing, Input, and Graphics
- **img** Image Loading & Writing
- **audio** Sound Loading & playback
- **fs** Common File System Functions
- **http** Simple HTTP Client & Server
- **term** TUI Utilities
- **col** Common Collision Detections
- **ase** Load Aseprite Spritesheets

All the modules can be configured with cargo feature:

```toml
default = [ "fs", "app", "img", "audio", ]
```
All the modules can be used with lua or as rust modules, toggle with `lua` feature

### example
here's a minimal window setup:

```rust
// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game;

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.push(&gfx::t()
			.translate_3d(vec3!(0, 0, 3))
			.rotate_y(ctx.time().into())
			.rotate_z(ctx.time().into())
		, |ctx| {
			return ctx.draw(shapes::cube());
		})?;

		ctx.draw(shapes::text("yo"))?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::run::<Game>() {
		println!("{}", err);
	}
}
```

### cli

The `dirty` binary is for running scripts

```sh
$ dirty frog.lua
```

If no argument is provided, it'll search for `main.{lua,py}`

### examples

- **window** basic window and a cube
- **conf** window / graphics configs
- **sprite** a 2d animating sprite
- **effect** custom shader effects
- **cow** displaying 3d models from obj files
- **mask** simple stencil mask
- **ttf** drawing text with truetype fonts
- **geom** basic geomatry / collision detections
- **dither** image processing
- **request** making an http/https request
- **server** a simple http server

### facts
- `DIRTY` is short for **Dangerous Ichthyopolist Reincarnates Tropical Yeti**

