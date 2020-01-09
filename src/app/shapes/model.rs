// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Model<'a> {
	mesh: &'a gfx::Model,
	color: Color,
	draw_bound: bool,
	draw_wireframe: bool,
}

pub fn model<'a>(m: &'a gfx::Model) -> Model<'a> {
	return Model::new(m);
}

impl<'a> Model<'a> {
	pub fn new(m: &'a gfx::Model) -> Self {
		return Self {
			mesh: m,
			color: rgba!(1),
			draw_bound: false,
			draw_wireframe: false,
		};
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn draw_bound(mut self, b: bool) -> Self {
		self.draw_bound = b;
		return self;
	}
	pub fn draw_wireframe(mut self, b: bool) -> Self {
		self.draw_wireframe = b;
		return self;
	}
}

impl<'a> Drawable for Model<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		let tex = self.mesh.texture().unwrap_or(&ctx.empty_tex);

		let prim = if self.draw_wireframe {
			gl::Primitive::Line
		} else {
			gl::Primitive::Triangle
		};

		for m in self.mesh.meshes() {
			let data = m.data();
			m.gl_mesh().draw(
				prim,
				&ctx.cur_pipeline_3d,
				&gfx::Uniform3D {
					proj: ctx.proj_3d,
					view: ctx.view_3d,
					model: ctx.transform * data.transform,
					// TODO: ?
					color: self.color,
					tex: tex.clone(),
					custom: ctx.cur_custom_uniform_3d.clone(),
				},
			);
		}

		if self.draw_bound {
			let (min, max) = self.mesh.bound();
			ctx.draw(&rect3d(min, max))?;
		}

		return Ok(());

	}

}

