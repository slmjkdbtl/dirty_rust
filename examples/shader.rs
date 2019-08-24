// wengwengweng

use dirty::*;
use dirty::app::*;
use gfx::Origin;
use input::Key;

struct Game {
	shader: gfx::Shader,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let shader = gfx::Shader::effect(ctx, include_str!("res/twist.frag"))?;

		shader.send("time", ctx.time());
		shader.send("mouse", vec2!(ctx.mouse_pos().x, ctx.mouse_pos().y));
		shader.send("resolution", vec2!(ctx.width(), ctx.height()));

		return Ok(Self {
			shader: shader,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Escape {
					ctx.quit();
				}
				if *k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_with(&self.shader, |ctx| {
			ctx.draw(shapes::rect(ctx.coord(Origin::TopLeft) + vec2!(48), ctx.coord(Origin::BottomRight) - vec2!(48)))?;
			return Ok(());
		})?;

		self.shader.send("time", ctx.time());
		self.shader.send("mouse", vec2!(ctx.mouse_pos().x, ctx.mouse_pos().y) / 640.0);

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

