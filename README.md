![icon](icon.png)

## DIRTY
personal toolkit for games and stuff

### modules
- **app** Windowing, Input, and Graphics
- **img** Image Loading & Writing
- **audio** Sound Loading & Playback
- **gkit** Gamedev Kit
- **physics** Simple Physics Simulation
- **synth** Software Synthesizer
- **term** CLI Utilities

All the modules can be configured with cargo feature:

```toml
default = [ "fs", "app", "img", "audio", ]
```

### example
here's a minimal window setup:

```rust
use dirty::*;
use app::*;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_t(mat4!()
			.t3(vec3!(0, 0, -6))
			.ry(ctx.time())
			.rz(ctx.time())
		, &shapes::cube())?;

		ctx.draw(&shapes::text("yo"))?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return run::<Game>();
}

```

### facts
- `DIRTY` is short for **Dangerous Ichthyopolist Reincarnates Tropical Yeti**

