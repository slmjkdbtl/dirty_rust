// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Skybox<'a> {
	skybox: &'a gfx::Skybox,
	color: Color,
}

impl<'a> Skybox<'a> {
	pub fn new(s: &'a gfx::Skybox) -> Self {
		return Self {
			skybox: s,
			color: rgba!(1),
		};
	}
}

pub fn skybox<'a>(s: &'a gfx::Skybox) -> Skybox<'a> {
	return Skybox::new(s);
}

impl<'a> Drawable for Skybox<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		ctx.gl.disable(gl::Capability::DepthTest);

		ctx.cubemap_renderer.draw(
			gl::Primitive::Triangle,
			&ctx.pipeline_cubemap,
			&gfx::UniformCubemap {
				proj: ctx.proj_3d,
				view: ctx.view_3d.remove_translation(),
				color: self.color,
				tex: self.skybox.texture().clone(),
			},
		);

		ctx.gl.enable(gl::Capability::DepthTest);

		return Ok(());

	}

}

