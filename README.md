![icon](icon.png)

# DIRTY
toolkit for games and stuff

## example
here's a minimal window setup:

```rust
use dirty::*;
use gfx::shapes;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		let win = &mut ctx.window;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => win.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let app = &mut ctx.app;
		let gfx = &mut ctx.gfx;
		let time = app.time().as_secs_f32();

		gfx.draw_t(
			mat4!()
				.tz(-120.0)
				.s3(vec3!(64))
				.ry(time)
				.rz(time)
				,
			&shapes::cube()
		)?;

		gfx.draw(
			&shapes::text("yo")
				.size(16.0)
		)?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = run::<Game>() {
		log!("{}", e);
	}
}
```

## facts
- `DIRTY` is short for **Dangerous Ichthyopolist Reincarnates Tropical Yeti**

