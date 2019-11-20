// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::app::*;
use input::Key;

struct Game {
	shader: gfx::Shader2D<TwistUniform>,
}

#[derive(Clone)]
struct TwistUniform {
	resolution: Vec2,
	mouse: Vec2,
	time: f32,
}

impl gfx::Uniform for TwistUniform {
	fn values(&self) -> gfx::UniformValues {
		return hashmap![
			"resolution" => &self.resolution,
			"mouse" => &self.mouse,
			"time" => &self.time,
		];
	}
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let shader = gfx::Shader2D::from_frag(ctx, include_str!("res/twist.frag"))?;

		return Ok(Self {
			shader: shader,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					Key::F => ctx.toggle_fullscreen(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_2d_with(&self.shader, &TwistUniform {

			resolution: vec2!(ctx.width(), ctx.height()),
			time: ctx.time().into(),
			mouse: ctx.mouse_pos().normalize(),

		}, |ctx| {

			ctx.draw(
				&shapes::rect(
					ctx.coord(gfx::Origin::TopLeft) + vec2!(48),
					ctx.coord(gfx::Origin::BottomRight) - vec2!(48)
				)
			)?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

