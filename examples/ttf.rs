// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game {
	font: gfx::TruetypeFont,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let mut font = gfx::TruetypeFont::from_bytes(d.gfx, include_bytes!("res/Zpix.ttf"), 12)?;

		// TODO: temperarily have to cache manually
		font.cache_str("营养过剩")?;
		font.cache_str("1agyo+-好")?;

		return Ok(Self {
			font: font,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::F => d.window.toggle_fullscreen(),
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw_t(
			mat4!()
				.s2(vec2!(12))
				,
			&shapes::text("1agyo+-好")
				.font(&self.font)
				,
		)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

