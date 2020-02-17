// wengwengweng

use dirty::*;
use app::*;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let map = vec![
			(gfx::Origin::TopLeft, "top left"),
			(gfx::Origin::Top, "top"),
			(gfx::Origin::TopRight, "top right"),
			(gfx::Origin::Left, "left"),
			(gfx::Origin::Center, "center"),
			(gfx::Origin::Right, "right"),
			(gfx::Origin::BottomLeft, "bottom left"),
			(gfx::Origin::Bottom, "bottom"),
			(gfx::Origin::BottomRight, "bottom right"),
		];

		for (o, t) in map {
			ctx.draw_t(
				mat4!()
					.t2(ctx.coord(o))
					,
				&shapes::text(t)
					.align(o)
					,
			)?;
		}

// 		let tt = ctx.default_font().format("Hi my name is luig\ni", gfx::FormatConf {
// 			align: gfx::Origin::TopLeft,
// 			wrap: Some(gfx::Wrap {
// 				width: 64.0,
// 				break_word: true,
// 				hyphonate: false,
// 			}),
// 			..Default::default()
// 		});
// 		ctx.draw_t(
// 			mat4!()
// 				.t2(ctx.coord(gfx::Origin::Center))
// 				,
// 			&tt,
// 		)?;
// 		ctx.draw_t(
// 			mat4!()
// 				.t2(ctx.coord(gfx::Origin::Center))
// 				,
// 			&shapes::rect(vec2!(), vec2!(tt.width(), -tt.height()))
// 				.no_fill()
// 				.stroke(rgba!(1))
// 				,
// 		)?;

		return Ok(());

	}

}

fn main() -> Result<()> {

	return launcher()
// 		.origin(gfx::Origin::TopLeft)
		.run::<Game>();
}

