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

This library was intended to be a game toolkit, but added a lot of other stuff due to my other scripting needs, and to provide a more integrated and unified scripting interface. All the modules can be configured with cargo feature:
```toml
default = [ "fs", "app", "img", "audio", ]
```
All the modules can be used with lua or as rust modules, toggle with `lua` feature

### example
here's a minimal window setup:
```rust
use dirty::*;
use dirty::app::*;
use input::Key;

struct Game;

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw(shapes::text("yo"))?;

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

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

The `dirty` binary is for running lua scripts
```sh
$ dirty frog.lua
```

If no argument is provided, it'll search for `main.lua`

### facts
- `DIRTY` is short for **Dangerous Ichthyopolist Reincarnates Tropical Yeti**

