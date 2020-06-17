// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game {
	text: String,
	size: f32,
	italic: bool,
	wrap: f32,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			text: String::new(),
			size: 16.0,
			italic: false,
			wrap: 160.0,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				let mods = d.window.key_mods();
				match *k {
					Key::Esc => d.window.quit(),
					Key::I if mods.meta => self.italic = !self.italic,
					Key::C if mods.meta => self.text.clear(),
					Key::Left if mods.meta => self.wrap -= 10.0,
					Key::Right if mods.meta => self.wrap += 10.0,
					_ => {},
				}
			},
			KeyPressRepeat(k) => {
				let mods = d.window.key_mods();
				match *k {
					Key::Backspace => {
						self.text.pop();
					},
					Key::Minus if mods.meta => self.size -= 1.0,
					Key::Equal if mods.meta => self.size += 1.0,
					_ => {},
				}
			},
			CharInput(ch) => {
				self.text.push(*ch);
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {
		d.window.set_title(&format!("FPS: {} DCS: {}", d.app.fps(), d.gfx.draw_calls()));
		return Ok(());
	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		let aligns = [
			gfx::Origin::TopLeft,
			gfx::Origin::Top,
			gfx::Origin::TopRight,
			gfx::Origin::Left,
			gfx::Origin::Center,
			gfx::Origin::Right,
			gfx::Origin::BottomLeft,
			gfx::Origin::Bottom,
			gfx::Origin::BottomRight,
		];

		for a in &aligns {

			let pos = d.gfx.coord(*a);

			let text = shapes::text(&self.text)
				.align(*a)
				.wrap(shapes::TextWrap {
					width: self.wrap,
					break_type: shapes::TextWrapBreak::Word,
				})
				.size(self.size)
				.italic(self.italic)
				.format(d.gfx);

			d.gfx.draw_t(
				mat4!()
					.t2(pos)
					,
				&text,
			)?;

			let tw = text.width();
			let th = text.height();

			d.gfx.draw_t(
				mat4!()
					.t2(pos)
					,
				&shapes::rect2(*a, tw, th)
					.no_fill()
					.stroke(rgba!(1))
					,
			)?;

			if let Some(cpos) = text.cursor_pos(self.text.len()) {
				d.gfx.draw_t(mat4!().t2(pos), &shapes::circle(cpos, 3.0))?;
			}

		}

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.run::<Game>() {
		elog!("{}", e);
	}
}

