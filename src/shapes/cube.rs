// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Cube;

impl Cube {
	pub fn new() -> Self {
		return Self;
	}
}

pub fn cube() -> Cube {
	return Cube::new();
}

impl Drawable for Cube {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		ctx.cube_renderer.draw(
			gl::Primitive::Triangle,
			&ctx.cur_pipeline_3d,
			&gfx::Uniform3D {
				proj: ctx.proj,
				view: ctx.view,
				model: ctx.transform,
				color: rgba!(),
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_3d.clone(),
			},
		);

		return Ok(());

	}

}

