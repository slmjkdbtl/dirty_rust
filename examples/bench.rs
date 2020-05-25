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

	fn init(d: &mut Ctx) -> Result<Self> {
		Ok(Self {
			tex: gfx::Texture::from_bytes(d.gfx, include_bytes!("res/bunny.png"))?,
			count: 10000,
		})
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::Space => self.count += 500,
					_ => {},
				}
			},
			_ => {},
		}

		Ok(())

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		d.window.set_title(&format!("FPS: {} DCS: {} OBJS: {}", d.app.fps(), d.gfx.draw_calls(), self.count));

		Ok(())

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		let w = d.gfx.width();
		let h = d.gfx.height();

		for _ in 0..self.count {
			d.gfx.draw_t(
				mat4!()
					.t2(vec2!(rand(-w, w) as f32 * 0.5, rand(-h, h) as f32 * 0.5))
					,
				&shapes::sprite(&self.tex)
					,
			)?;
		}

		let c = if d.app.fps() >= 60 {
			rgba!(0, 1, 0, 1)
		} else {
			rgba!(1, 0, 0, 1)
		};

		d.gfx.draw_t(
			mat4!()
				.s2(vec2!(6))
				,
			&shapes::text(&format!("{}", d.app.fps()))
				.color(c)
		)?;

		d.gfx.draw_t(
			mat4!()
				.ty(-64.0)
				.s2(vec2!(1.5))
				,
			&shapes::text(&format!("{} bunnies", self.count)),
		)?;

		Ok(())

	}

}

fn main() {

	if let Err(err) = launcher()
// 		.hidpi(false)
		.vsync(false)
		.run::<Game>() {
		elog!("{}", err);
	}

}

