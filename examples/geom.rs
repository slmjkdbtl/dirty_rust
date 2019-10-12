// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;
use input::Mouse;

struct Game {
	pts: Vec<Vec2>,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self {
			pts: vec![],
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
				if k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			MousePress(m) => {
				if m == Mouse::Left {
					self.pts.push(ctx.mouse_pos());
				}
			}
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		use shapes::*;

		ctx.draw(
			&gradient(
				ctx.coord(gfx::Origin::Top),
				ctx.coord(gfx::Origin::Bottom),
				&[
					(color!(0.4, 1, 1, 1), 0.0),
					(color!(1, 1, 0.6, 1), 0.5),
					(color!(1, 0.4, 0.8, 1), 1.0),
				],
			)
				.width(640.0)
		)?;

		for p in &self.pts {
			ctx.draw(
				&circle(*p, 3.0)
					.fill(Color::BLUE)
			)?;
		}

		ctx.draw(
			&polygon(&self.pts)
				.stroke(color!(1.0))
				.nofill()
				.line_width(1.0)
		)?;

		ctx.draw(
			&circle(vec2!(0), 120.0)
				.fill(color!(1, 0, 1, 1))
		)?;

		ctx.draw(
			&rect(vec2!(-72, -54), vec2!(72, 54))
				.fill(color!(0, 1, 1, 1))
				.radius(12.0)
		)?;

		ctx.draw(
			&rect(vec2!(-48, -32), vec2!(48, 32))
				.stroke(color!(1, 1, 0, 1))
				.nofill()
				.line_width(2.0)
				.radius(12.0)
		)?;

		ctx.draw(
			&text("geom")
				.color(color!(0, 0, 1, 1))
		)?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}
}

