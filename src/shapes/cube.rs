// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Cube {
	color: Color,
}

impl Cube {

	pub fn new() -> Self {
		return Self {
			color: rgba!(1),
		};
	}

	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}

	pub fn opacity(mut self, o: f32) -> Self {
		self.color.a = o;
		return self;
	}

}

pub fn cube() -> Cube {
	return Cube::new();
}

impl Drawable for Cube {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.gfx.draw_calls += 1;

		ctx.gfx.cube_renderer.draw(
			gl::Primitive::Triangle,
			&ctx.gfx.cur_pipeline,
			&gfx::Uniform {
				proj: ctx.gfx.proj,
				view: ctx.gfx.view,
				model: ctx.gfx.transform,
				color: self.color,
				tex: ctx.gfx.empty_tex.clone(),
				custom: ctx.gfx.cur_custom_uniform.clone(),
			},
		);

		return Ok(());

	}

}

