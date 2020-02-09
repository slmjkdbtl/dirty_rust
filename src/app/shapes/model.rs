// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Model<'a> {
	model: &'a gfx::Model,
	color: Color,
	prim: gl::Primitive,
	time: f32,
}

pub fn model<'a>(m: &'a gfx::Model) -> Model<'a> {
	return Model::new(m);
}

impl<'a> Model<'a> {
	pub fn new(m: &'a gfx::Model) -> Self {
		return Self {
			model: m,
			color: rgba!(1),
			prim: gl::Primitive::Triangle,
			time: 0.0,
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
	pub fn time(mut self, t: f32) -> Self {
		self.time = t;
		return self;
	}
	pub fn draw_wireframe(mut self, b: bool) -> Self {
		if b {
			self.prim = gl::Primitive::Line;
		} else {
			self.prim = gl::Primitive::Triangle;
		}
		return self;
	}
}

impl<'a> Drawable for Model<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		for t in self.model.root_nodes() {
			draw_mesh(ctx, &self, Mat4::identity(), *t);
		}

		return Ok(());

	}

}

fn draw_mesh(ctx: &mut Ctx, dctx: &Model, ptr: Mat4, id: usize) {

	let model = &dctx.model;

	if let Some(node) = model.get_node(id) {

		let mut tr = node.transform;

		if let Some(anim) = model.get_anim(id) {

			let (pos, rot, scale) = anim.get_transform(dctx.time);

			tr.pos = pos.unwrap_or(tr.pos);
			tr.rot = rot.unwrap_or(tr.rot);
			tr.scale = scale.unwrap_or(tr.scale);

		}

		let tr = ptr * tr.as_mat4();
		let tex = model.texture().unwrap_or(&ctx.empty_tex);

		for mesh in &node.meshes {

			ctx.draw_calls += 1;

			mesh.gl_mesh().draw(
				dctx.prim,
				&ctx.cur_pipeline_3d,
				&gfx::Uniform3D {
					proj: ctx.proj,
					view: ctx.view,
					model: ctx.transform * tr,
					color: dctx.color,
					tex: tex.clone(),
					custom: ctx.cur_custom_uniform_3d.clone(),
				},
			);

		}

		for c in &node.children {
			draw_mesh(ctx, dctx, tr, *c);
		}

	}

}

