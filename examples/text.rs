// wengwengweng

use dirty::*;
use app::*;
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

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				let mods = ctx.key_mods();
				match *k {
					Key::Esc => ctx.quit(),
					Key::I if mods.meta => self.italic = !self.italic,
					Key::C if mods.meta => self.text.clear(),
					Key::Left if mods.meta => self.wrap -= 10.0,
					Key::Right if mods.meta => self.wrap += 10.0,
					_ => {},
				}
			},
			KeyPressRepeat(k) => {
				let mods = ctx.key_mods();
				match *k {
					Key::Back => {
						self.text.pop();
					},
					Key::Minus if mods.meta => self.size -= 1.0,
					Key::Equals if mods.meta => self.size += 1.0,
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

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {
		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));
		return Ok(());
	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

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

			let pos = ctx.coord(*a);

			let text = shapes::text(&self.text)
				.align(*a)
				.wrap(shapes::TextWrap {
					width: self.wrap,
					break_type: shapes::TextWrapBreak::Word,
				})
				.size(self.size)
				.italic(self.italic)
				.format(ctx);

			ctx.draw_t(
				mat4!()
					.t2(pos)
					,
				&text,
			)?;

			let tw = text.width();
			let th = text.height();

			ctx.draw_t(
				mat4!()
					.t2(pos)
					,
				&shapes::rect2(*a, tw, th)
					.no_fill()
					.stroke(rgba!(1))
					,
			)?;

// 			if let Some(cpos) = text.cursor_pos(self.text.len()) {

// 			}

		}

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
		.run::<Game>();
}

