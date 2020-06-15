// wengwengweng

use dirty::*;
use math::*;
use gfx::shapes;
use input::Key;

const INCRE: usize = 100;

struct Game {
	tex: gfx::Texture,
	count: usize,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			tex: gfx::Texture::from_bytes(d.gfx, include_bytes!("res/acid2.png"))?,
			count: INCRE,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::Space => self.count += INCRE,
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		d.window.set_title(&format!("FPS: {} DCS: {} OBJS: {}", d.app.fps(), d.gfx.draw_calls(), self.count));

		return Ok(());

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
					.width(32.0)
					.height(32.0)
					,
			)?;
		}

		let c = if d.app.fps() >= 60 {
			rgba!(0, 1, 0, 1)
		} else {
			rgba!(1, 0, 0, 1)
		};

		d.gfx.draw(
			&shapes::rect(-vec2!(144, 96), vec2!(144, 84))
				.fill(rgba!(0, 0, 0, 1))
		)?;

		d.gfx.draw_t(
			mat4!()
				,
			&shapes::text(&format!("{}", d.app.fps()))
				.color(c)
				.size(64.0)
		)?;

		d.gfx.draw_t(
			mat4!()
				.ty(-54.0)
				,
			&shapes::text(&format!("{} faces", self.count))
				.size(16.0)
		)?;

		return Ok(());

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

