// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Model<'a> {
	model: &'a gfx::Model,
	color: Color,
	draw_wireframe: bool,
}

pub fn model<'a>(m: &'a gfx::Model) -> Model<'a> {
	return Model::new(m);
}

impl<'a> Model<'a> {
	pub fn new(m: &'a gfx::Model) -> Self {
		return Self {
			model: m,
			color: rgba!(1),
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
	pub fn draw_wireframe(mut self, b: bool) -> Self {
		self.draw_wireframe = b;
		return self;
	}
}

impl<'a> Drawable for Model<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let prim = if self.draw_wireframe {
			gl::Primitive::Line
		} else {
			gl::Primitive::Triangle
		};

		for t in self.model.scene() {
			draw_mesh(ctx, prim, &self.model, Mat4::identity(), *t);
		}

		return Ok(());

	}

}

fn draw_mesh(ctx: &mut Ctx, prim: gl::Primitive, model: &gfx::Model, ptr: Mat4, id: usize) {

	if let Some(node) = model.get_node(id) {

		let mut tr = node.transform;

		if let Some(anim) = model.get_anim(id) {

			let t = ctx.time();
			let tt = t - f32::floor(t / 0.5) * 0.5;
			let (pos, rot, scale) = anim.get_transform(tt);

			tr.pos = pos.unwrap_or(tr.pos);
			tr.rot = rot.unwrap_or(tr.rot);
			tr.scale = scale.unwrap_or(tr.scale);

		}

		let tr = ptr * tr.as_mat4();

		if let Some(meshes) = &node.meshes {

			for mesh in meshes {

				let tex = model.texture().unwrap_or(&ctx.empty_tex);

				ctx.draw_calls += 1;

				mesh.gl_mesh().draw(
					prim,
					&ctx.cur_pipeline_3d,
					&gfx::Uniform3D {
						proj: ctx.proj_3d,
						view: ctx.view_3d,
						model: ctx.transform * tr,
						color: rgba!(1),
						tex: tex.clone(),
						custom: ctx.cur_custom_uniform_3d.clone(),
					},
				);

			}

		}

		for c in &node.children {
			draw_mesh(ctx, prim, model, tr, *c);
		}

	}

}

