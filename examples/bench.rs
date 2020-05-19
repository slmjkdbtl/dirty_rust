// wengwengweng

use dirty::*;
use math::*;
use gfx::shapes;
use input::Key;

struct Game {
	tex: gfx::Texture,
	count: usize,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {
		let gfx = &mut ctx.gfx;
		return Ok(Self {
			tex: gfx::Texture::from_bytes(gfx, include_bytes!("res/bunny.png"))?,
			count: 1000,
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		let win = &mut ctx.window;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => win.quit(),
					Key::Space => self.count += 500,
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		let win = &mut ctx.window;
		let gfx = &ctx.gfx;
		let app = &ctx.app;

		win.set_title(&format!("FPS: {} DCS: {} OBJS: {}", app.fps(), gfx.draw_calls(), self.count));

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let gfx = &mut ctx.gfx;
		let app = &mut ctx.app;
		let w = gfx.width();
		let h = gfx.height();

		for _ in 0..self.count {
			gfx.draw_t(
				mat4!()
					.t2(vec2!(rand(-w, w) as f32 * 0.5, rand(-h, h) as f32 * 0.5))
					,
				&shapes::sprite(&self.tex)
					,
			)?;
		}

		let c = if app.fps() >= 60 {
			rgba!(0, 1, 0, 1)
		} else {
			rgba!(1, 0, 0, 1)
		};

		gfx.draw_t(
			mat4!()
				.s2(vec2!(6))
				,
			&shapes::text(&format!("{}", app.fps()))
				.color(c)
		)?;

		gfx.draw_t(
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

