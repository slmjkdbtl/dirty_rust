// wengwengweng

use dirty::*;
use math::*;
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

// 		ctx.draw_t(
// 			mat4!()
// 				.tz(-120.0)
// 				.s3(vec3!(64))
// 				.ry(ctx.time())
// 				.rz(ctx.time())
// 				,
// 			&shapes::cube()
// 		)?;

// 		ctx.draw(
// 			&shapes::text("yo")
// 				.size(16.0)
// 		)?;

		let mut pts = vec![];
		let mut x = 0.0;
		let mut dt = 2.0;
		let op = vec2!(0);

		pts.push(op);

		while x <= 240.0 {

			x += dt;

			let w1 = f32::sin(x / 60.0) * wave(ctx.time() * 2.0, 12.0, 16.0);
			let w2 = f32::sin(x / 60.0 * 2.0) * wave(ctx.time() * 1.0, 6.0, 8.0);
			let w3 = f32::sin(x / 60.0 * 3.0) * wave(ctx.time() * 1.5, 9.0, 12.0);

			let pt = vec2!(x, w1 + w2 + w3);
			let angle = Vec2::angle(op, pt);
			let pt = Vec2::from_angle(angle + f32::to_radians(-45.0)) * pt.len();

			pts.push(pt);

		}

		for i in 0..pts.len() - 1 {

			let p1 = pts[i];
			let p2 = pts[i + 1];

			ctx.draw(&shapes::line(p1, p2))?;

		}

		return Ok(());

	}

}

fn main() -> Result<()> {
	return run::<Game>();
}

