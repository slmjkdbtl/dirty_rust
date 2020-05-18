// wengwengweng

use dirty::*;
use math::*;
use input::Key;

struct Game {
	tex: gfx::Texture,
	count: usize,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			tex: gfx::Texture::from_bytes(ctx, include_bytes!("res/bunny.png"))?,
			count: 1000,
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

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

	fn update(&mut self, ctx: &mut Ctx, dt: std::time::Duration) -> Result<()> {

		ctx.set_title(&format!("FPS: {} DCS: {} OBJS: {}", ctx.fps(), ctx.draw_calls(), self.count));

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let w = ctx.width();
		let h = ctx.height();

		for _ in 0..self.count {
			ctx.draw_t(
				mat4!()
					.t2(vec2!(rand(-w, w) as f32 * 0.5, rand(-h, h) as f32 * 0.5))
					,
				&shapes::sprite(&self.tex)
					,
			)?;
		}

		let c = if ctx.fps() >= 60 {
			rgba!(0, 1, 0, 1)
		} else {
			rgba!(1, 0, 0, 1)
		};

		ctx.draw_t(
			mat4!()
				.s2(vec2!(6))
				,
			&shapes::text(&format!("{}", ctx.fps()))
				.color(c)
		)?;

		ctx.draw_t(
			mat4!()
				.ty(-64.0)
				.s2(vec2!(1.5))
				,
			&shapes::text(&format!("{} bunnies", self.count)),
		)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
// 		.hidpi(false)
		.vsync(false)
		.run::<Game>() {
		println!("{}", err);
	}

}

