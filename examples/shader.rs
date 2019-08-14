// wengwengweng

use dirty::*;
use dirty::app::*;
use gfx::Coord;
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

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_with(&self.shader, |ctx| {
			ctx.draw(shapes::rect(ctx.coord(Coord::TopLeft) + vec2!(48), ctx.coord(Coord::BottomRight) - vec2!(48)))?;
			return Ok(());
		})?;

		self.shader.send("time", ctx.time());
		self.shader.send("mouse", vec2!(ctx.mouse_pos().x, ctx.mouse_pos().y) / 640.0);

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		if ctx.key_pressed(Key::F) {
			ctx.toggle_fullscreen();
		}

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

