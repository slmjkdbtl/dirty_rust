// wengwengweng

use super::*;

#[derive(Clone)]
pub struct UVRect<'a> {
	p1: Vec2,
	p2: Vec2,
	color: Color,
	tex: Option<&'a gfx::Texture>,
}

impl<'a> UVRect<'a> {
	pub fn new(p1: Vec2, p2: Vec2) -> Self {
		return Self {
			p1,
			p2,
			color: rgba!(1),
			tex: None,
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
	pub fn texture(mut self, tex: &'a gfx::Texture) -> Self {
		self.tex = Some(tex);
		return self;
	}
}

pub fn uvrect<'a>(p1: Vec2, p2: Vec2) -> UVRect<'a> {
	return UVRect::new(p1, p2);
}

impl<'a> Drawable for UVRect<'a> {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		let tex = self.tex.cloned().unwrap_or(ctx.empty_tex.clone());
		let p1 = vec2!(f32::min(self.p1.x, self.p2.x), f32::min(self.p1.y, self.p2.y));
		let p2 = vec2!(f32::max(self.p1.x, self.p2.x), f32::max(self.p1.y, self.p2.y));

		ctx.draw(&raw(&[
			Vertex {
				pos: vec3!(p1.x, p1.y, 0),
				color: self.color,
				normal: vec3!(0, 0, 1),
				uv: vec2!(0, 0),
			},
			Vertex {
				pos: vec3!(p2.x, p1.y, 0),
				color: self.color,
				normal: vec3!(0, 0, 1),
				uv: vec2!(1, 0),
			},
			Vertex {
				pos: vec3!(p2.x, p2.x, 0),
				color: self.color,
				normal: vec3!(0, 0, 1),
				uv: vec2!(1, 1),
			},
			Vertex {
				pos: vec3!(p1.x, p2.y, 0),
				color: self.color,
				normal: vec3!(0, 0, 1),
				uv: vec2!(0, 1),
			},
		], &[0, 3, 1, 1, 3, 2]).texture(&tex))?;

		return Ok(());

	}

}

