// wengwengweng

use dirty::*;
use dirty::math::*;
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
		return hmap![
			"u_resolution" => &self.resolution,
			"u_mouse" => &self.mouse,
			"u_time" => &self.time,
		];
	}
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let shader = gfx::Shader2D::from_frag(ctx, include_str!("res/twist.frag"))?;

		return Ok(Self {
			shader: shader,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

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

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_2d_with(&self.shader, &TwistUniform {
			resolution: vec2!(ctx.width(), ctx.height()) * ctx.dpi(),
			mouse: ctx.mouse_pos() / vec2!(ctx.width(), ctx.height()),
			time: ctx.time().into(),
		}, |ctx| {
			ctx.draw(
				&shapes::rect(
					ctx.coord(gfx::Origin::TopLeft) + vec2!(48, -48),
					ctx.coord(gfx::Origin::BottomRight) - vec2!(48, -48)
				)
			)?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

