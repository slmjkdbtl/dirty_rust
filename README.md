![icon](icon.png)

# DIRTY
a cross-platform toolkit for making games / interactive experiences

[website](https://tool.wengwengweng.me/)

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

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		let time = d.app.time().as_secs_f32();

		d.gfx.draw_t(
			mat4!()
				.tz(-120.0)
				.s3(vec3!(64))
				.ry(time)
				.rz(time)
				,
			&shapes::cube()
		)?;

		d.gfx.draw(
			&shapes::text("yo")
				.size(16.0)
		)?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = run::<Game>() {
		elog!("{}", e);
	}
}
```

## platforms
- MacOS
- Windows
- Linux
- WASM
- iOS (WIP)
- Android (WIP)

for build instructions, check out `doc/dist.md`

## examples

the examples are under `examples/`, run them natively with

```sh
$ cargo run --example (example_name)
```

or go check out the built wasm examples [here](https://tool.wengwengweng.me/examples.html)

## doc
check out `doc/` and cargo docs

## facts
- `DIRTY` is short for **Dangerous Ichthyopolist Reincarnates Tropical Yeti**

