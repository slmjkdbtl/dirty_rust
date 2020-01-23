// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Sprite3D<'a> {
	tex: &'a gfx::Texture,
	quad: Quad,
	offset: Vec2,
	flip: gfx::Flip,
	color: Color,
}

pub fn sprite3d<'a>(tex: &'a gfx::Texture) -> Sprite3D<'a> {
	return Sprite3D::new(tex);
}

// TODO: up side down?
// TODO: clean
impl<'a> Sprite3D<'a> {
	pub fn new(tex: &'a gfx::Texture) -> Self {
		return Self {
			tex: tex,
			quad: quad!(0, 0, 1, 1),
			color: rgba!(1),
			offset: vec2!(0),
			flip: gfx::Flip::None,
		};
	}
	pub fn quad(mut self, quad: Quad) -> Self {
		self.quad = quad;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn offset(mut self, offset: Vec2) -> Self {
		self.offset = offset;
		return self;
	}
	pub fn flip(mut self, flip: gfx::Flip) -> Self {
		self.flip = flip;
		return self;
	}
}

impl<'a> Drawable for Sprite3D<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);
		let offset = self.offset * -0.5;

		ctx.push(mat4!()
			.s3(vec3!(scale.x, scale.y, 1.0))
			.t3(vec3!(offset.x, offset.y, 0.0))
		, |ctx| {

			let shape = gfx::Quad3DShape {
				transform: ctx.transform,
				quad: self.quad,
				color: self.color,
				flip: self.flip,
			};

			ctx.renderer_3d.push_shape(
				gl::Primitive::Triangle,
				shape,
				&ctx.cur_pipeline_3d,
				&gfx::Uniform3D {
					proj: ctx.proj_3d,
					view: ctx.view_3d,
					model: ctx.transform,
					color: rgba!(),
					tex: self.tex.clone(),
					custom: ctx.cur_custom_uniform_3d.clone(),
				},
			)?;

			return Ok(());

		})?;

		return Ok(());

	}

}


