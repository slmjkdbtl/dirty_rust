// wengwengweng

use super::*;
use gfx::Flip;
use std::mem;

const INDICES: [u32; 6] = [0, 3, 1, 1, 3, 2];

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
			tex,
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

impl<'a> gfx::Drawable for Sprite<'a> {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		let tw = self.tex.width() as f32 * self.quad.w;
		let th = self.tex.height() as f32 * self.quad.h;

		let scale = match (self.width, self.height) {
			(Some(w), Some(h)) => vec2!(w, h),
			(Some(w), None) => vec2!(w, w * th / tw),
			(None, Some(h)) => vec2!(h * tw / th, h),
			(None, None) => vec2!(tw, th),
		};

		let offset = self.offset.unwrap_or(vec2!(0));

		let t = ctx.transform
			.s2(scale)
			.t2(offset * -0.5)
			;

		let p1 = t * vec3!(-0.5, 0.5, 0.0);
		let p2 = t * vec3!(0.5, 0.5, 0.0);
		let p3 = t * vec3!(0.5, -0.5, 0.0);
		let p4 = t * vec3!(-0.5, -0.5, 0.0);

		// TODO: flip img instead of tex coord
		let q = self.quad;
		let mut u1 = vec2!(q.x, q.y);
		let mut u2 = vec2!(q.x + q.w, q.y);
		let mut u3 = vec2!(q.x + q.w, q.y + q.h);
		let mut u4 = vec2!(q.x, q.y + q.h);

// 		let mut u1 = vec2!(q.x, q.y + q.h);
// 		let mut u2 = vec2!(q.x + q.w, q.y + q.h);
// 		let mut u3 = vec2!(q.x + q.w, q.y);
// 		let mut u4 = vec2!(q.x, q.y);

		match self.flip {
			Flip::X => {
				mem::swap(&mut u1, &mut u2);
				mem::swap(&mut u3, &mut u4);
			},
			Flip::Y => {
				mem::swap(&mut u1, &mut u4);
				mem::swap(&mut u2, &mut u3);
			},
			Flip::XY => {
				mem::swap(&mut u1, &mut u3);
				mem::swap(&mut u2, &mut u4);
			},
			_ => {},
		}

		ctx.draw(
			&raw(&[
				Vertex {
					pos: p1,
					uv: u1,
					normal: vec3!(0, 0, 1),
					color: self.color,
				},
				Vertex {
					pos: p2,
					uv: u2,
					normal: vec3!(0, 0, 1),
					color: self.color,
				},
				Vertex {
					pos: p3,
					uv: u3,
					normal: vec3!(0, 0, 1),
					color: self.color,
				},
				Vertex {
					pos: p4,
					uv: u4,
					normal: vec3!(0, 0, 1),
					color: self.color,
				},
			], &INDICES)
				.texture(&self.tex)
				.transformed()
		)?;

		return Ok(());

	}

}

