// wengwengweng

use dirty::*;
use app::*;
use input::Key;

struct Game {
	text: String,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			text: String::new(),
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					Key::Back => {
						self.text.pop();
					},
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

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

// 		let map = vec![
// 			(gfx::Origin::TopLeft, "top left"),
// 			(gfx::Origin::Top, "top"),
// 			(gfx::Origin::TopRight, "top right"),
// 			(gfx::Origin::Left, "left"),
// 			(gfx::Origin::Center, "center"),
// 			(gfx::Origin::Right, "right"),
// 			(gfx::Origin::BottomLeft, "bottom left"),
// 			(gfx::Origin::Bottom, "bottom"),
// 			(gfx::Origin::BottomRight, "bottom right"),
// 		];

// 		for (o, t) in map {
// 			ctx.draw_t(
// 				mat4!()
// 					.t2(ctx.coord(o))
// 					,
// 				&shapes::text(t)
// 					.align(o)
// 					,
// 			)?;
// 		}

		let text = &shapes::text(&self.text)
			.align(gfx::Origin::TopLeft)
			.wrap(shapes::TextWrap {
				width: 120.0,
				break_word: false,
				hyphonate: false,
			})
			.format(ctx);

		ctx.draw_t(
			mat4!()
				.t2(ctx.coord(gfx::Origin::TopLeft))
				,
			text,
		)?;
		ctx.draw_t(
			mat4!()
				.t2(ctx.coord(gfx::Origin::TopLeft))
				,
			&shapes::rect(vec2!(), vec2!(text.width(), -text.height()))
				.no_fill()
				.stroke(rgba!(1))
				,
		)?;

		return Ok(());

	}

}

fn main() -> Result<()> {

	return launcher()
// 		.origin(gfx::Origin::TopLeft)
		.run::<Game>();
}

