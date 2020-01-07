// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Model<'a> {
	mesh: &'a gfx::Model,
	color: Color,
	bound: bool,
	wireframe: bool,
}

pub fn model<'a>(m: &'a gfx::Model) -> Model<'a> {
	return Model::new(m);
}

impl<'a> Model<'a> {
	pub fn new(m: &'a gfx::Model) -> Self {
		return Self {
			mesh: m,
			color: rgba!(1),
			bound: false,
			wireframe: false,
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
	pub fn bound(mut self) -> Self {
		self.bound = true;
		return self;
	}
	pub fn wireframe(mut self, b: bool) -> Self {
		self.wireframe = b;
		return self;
	}
}

impl<'a> Drawable for Model<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		let tex = self.mesh.texture().unwrap_or(&ctx.empty_tex);

		let prim = if self.wireframe {
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
					model: ctx.transform.apply(&data.transform),
					// TODO: ?
					color: self.color,
					tex: tex.clone(),
					custom: ctx.cur_custom_uniform_3d.clone(),
				},
			);
		}

		if self.bound {
			let (min, max) = self.mesh.bound();
			ctx.draw(&rect3d(min, max))?;
		}

		return Ok(());

	}

}

