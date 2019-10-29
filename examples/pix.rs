// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;

#[derive(Clone)]
pub struct PixUniform {
	pub resolution: Vec2,
	pub size: f32,
}

impl gfx::Uniform for PixUniform {
	fn values(&self) -> gfx::UniformValues {
		return vec![
			("resolution", &self.resolution),
			("size", &self.size),
		];
	}
}

pub struct PixEffect {
	shader: gfx::Shader2D<PixUniform>,
	canvas: gfx::Canvas,
}

impl PixEffect {

	pub fn new(ctx: &app::Ctx) -> Result<Self> {
		return Ok(Self {
			shader: gfx::Shader2D::from_frag(ctx, include_str!("res/pix.frag"))?,
			canvas: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
		});
	}

	pub fn render(&self, ctx: &mut app::Ctx, f: impl FnOnce(&mut app::Ctx) -> Result<()>) -> Result<()> {

		ctx.draw_on(&self.canvas, |ctx| {
			f(ctx)?;
			return Ok(());
		})?;

		return Ok(());

	}

	pub fn canvas(&self) -> &gfx::Canvas {
		return &self.canvas;
	}

	pub fn draw(&self, ctx: &mut app::Ctx, u: &PixUniform) -> Result<()> {

		ctx.draw_2d_with(&self.shader, u, |ctx| {
			ctx.draw(&shapes::canvas(&self.canvas))?;
			return Ok(());
		})?;

		return Ok(());

	}

}

