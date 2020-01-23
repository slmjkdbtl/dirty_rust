// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::app::*;
use input::Key;

struct Game {
	tex: gfx::Texture,
	count: usize,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {
		return Ok(Self {
			tex: gfx::Texture::from_bytes(ctx, include_bytes!("res/bunny.png"))?,
			count: 10000,
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					Key::Space => self.count += 500,
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.set_title(&format!("FPS: {} DCS: {} OBJS: {}", ctx.fps(), ctx.draw_calls(), self.count));

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let w = ctx.gwidth() as i32;
		let h = ctx.gheight() as i32;

		for _ in 0..self.count {
			ctx.push(mat4!()
				.t2(vec2!(rand(-w, w) as f32 * 0.5, rand(-h, h) as f32 * 0.5))
			, |ctx| {
				ctx.draw(&shapes::sprite(&self.tex))?;
				return Ok(());
			})?;
		}

		ctx.push(mat4!()
			.s2(vec2!(6))
		, |ctx| {
			let fps = ctx.fps();
			let c = if fps >= 60 {
				rgba!(0, 1, 0, 1)
			} else {
				rgba!(1, 0, 0, 1)
			};
			ctx.draw(&shapes::text(&format!("{}", fps)).color(c))?;
			return Ok(());
		})?;

		ctx.push(mat4!()
			.ty(-64.0)
			.s2(vec2!(1.5))
		, |ctx| {
			ctx.draw(&shapes::text(&format!("{} bunnies", self.count)))?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
// 		.hidpi(false)
		.fps_cap(None)
		.vsync(false)
		.run::<Game>() {
		println!("{}", err);
	}

}

