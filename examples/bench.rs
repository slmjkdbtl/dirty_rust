// wengwengweng

use dirty::*;
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

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
				if k == Key::Space {
					self.count += 500;
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

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		let w = ctx.width() as i32;
		let h = ctx.height() as i32;

		for _ in 0..self.count {
			ctx.push(&gfx::t()
				.translate(vec2!(rand!(-w, w) * 0.5, rand!(-h, h) * 0.5))
			, |ctx| {
				ctx.draw(shapes::sprite(&self.tex))?;
				return Ok(());
			})?;
		}

		ctx.push(&gfx::t()
			.scale(vec2!(6))
		, |ctx| {
			let fps = ctx.fps();
			let c = if fps >= 60 {
				color!(0, 1, 0, 1)
			} else {
				color!(1, 0, 0, 1)
			};
			ctx.draw(shapes::text(&format!("{}", fps)).color(c))?;
			return Ok(());
		})?;

		ctx.push(&gfx::t()
			.translate(vec2!(0, 64))
			.scale(vec2!(1.5))
		, |ctx| {
			ctx.draw(shapes::text(&format!("{} bunnies", self.count)))?;
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

