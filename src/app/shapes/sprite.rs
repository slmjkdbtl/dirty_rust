// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Sprite<'a> {
	tex: &'a gfx::Texture,
	quad: Quad,
	offset: Option<Vec2>,
	flip: gfx::Flip,
	color: Color,
	width: Option<f32>,
	height: Option<f32>,
}

impl<'a> Sprite<'a> {
	pub fn new(tex: &'a gfx::Texture) -> Self {
		return Self {
			tex: tex,
			quad: quad!(0, 0, 1, 1),
			color: rgba!(1),
			offset: None,
			flip: gfx::Flip::None,
			width: None,
			height: None,
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
		self.offset = Some(offset);
		return self;
	}
	pub fn flip(mut self, flip: gfx::Flip) -> Self {
		self.flip = flip;
		return self;
	}
	pub fn width(mut self, w: f32) -> Self {
		self.width = Some(w);
		return self;
	}
	pub fn height(mut self, h: f32) -> Self {
		self.height = Some(h);
		return self;
	}
}

pub fn sprite<'a>(tex: &'a gfx::Texture) -> Sprite<'a> {
	return Sprite::new(tex);
}

impl<'a> Drawable for Sprite<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let tw = self.tex.width() as f32 * self.quad.w;
		let th = self.tex.height() as f32 * self.quad.h;

		let scale = match (self.width, self.height) {
			(Some(w), Some(h)) => vec2!(w, h),
			(Some(w), None) => vec2!(w, w * th / tw),
			(None, Some(h)) => vec2!(h * tw / th, h),
			(None, None) => vec2!(tw, th),
		};

		let offset = self.offset.unwrap_or(vec2!(0));

		// TODO: extremely slow
		let t = ctx.transform
			.s2(scale)
			.t2(offset * -0.5)
			;

		let shape = gfx::QuadShape {
			transform: t.as_mat4(),
			quad: self.quad,
			color: self.color,
			flip: self.flip,
		};

		ctx.renderer_2d.push_shape(
			gl::Primitive::Triangle,
			shape,
			&ctx.cur_pipeline_2d,
			&gfx::Uniform2D {
				proj: ctx.proj_2d,
				tex: self.tex.clone(),
				custom: ctx.cur_custom_uniform_2d.clone(),
			}
		)?;

		return Ok(());

	}

}

