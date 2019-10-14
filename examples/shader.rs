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
		return vec![
			("resolution", self.resolution.into()),
			("mouse", self.mouse.into()),
			("time", self.time.into()),
		];
	}
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let shader = gfx::Shader2D::effect(ctx, include_str!("res/twist.frag"))?;

		return Ok(Self {
			shader: shader,
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
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_2d_with(&self.shader, &TwistUniform {

			resolution: vec2!(ctx.width(), ctx.height()),
			time: ctx.time().into(),
			mouse: ctx.mouse_pos().normalize(),

		}, |ctx| {

			ctx.draw(
				shapes::rect(
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

