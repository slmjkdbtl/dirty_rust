// wengwengweng

use dirty::*;
use math::*;
use input::Key;
use input::Mouse;

struct Game {
	pts: Vec<Vec2>,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			pts: vec![],
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match *e {
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

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		use shapes::*;

		ctx.draw(
			&gradient(
				ctx.coord(gfx::Origin::Top),
				ctx.coord(gfx::Origin::Bottom),
				&[
					(rgba!(0.4, 1, 1, 1), 0.0),
					(rgba!(1, 1, 0.6, 1), 0.5),
					(rgba!(1, 0.4, 0.8, 1), 1.0),
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
				.stroke(rgba!(1.0))
				.no_fill()
				.line_width(1.0)
		)?;

		ctx.draw(
			&circle(vec2!(0), 120.0)
				.fill(rgba!(1, 0, 1, 1))
		)?;

		ctx.draw(
			&rect(vec2!(-72, -54), vec2!(72, 54))
				.fill(rgba!(0, 1, 1, 1))
				.radius(12.0)
		)?;

		ctx.draw(
			&rect(vec2!(-48, -32), vec2!(48, 32))
				.stroke(rgba!(1, 1, 0, 1))
				.no_fill()
				.line_width(2.0)
				.radius(12.0)
		)?;

		ctx.draw(
			&text("geom")
				.color(rgba!(0, 0, 1, 1))
		)?;

		ctx.draw(
			&Spline::from_pts(&[
				(0.0, vec2!(-48)),
				(0.4, vec2!(36, -24)),
				(1.0, vec2!(48))
			])
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

