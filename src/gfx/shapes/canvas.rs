// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Canvas<'a> {
	canvas: &'a gfx::Canvas,
	color: Color,
	width: Option<f32>,
	height: Option<f32>,
	offset: Option<Vec2>,
}

pub fn canvas<'a>(c: &'a gfx::Canvas) -> Canvas<'a> {
	return Canvas::new(c);
}

impl<'a> Canvas<'a> {
	pub fn new(c: &'a gfx::Canvas) -> Self {
		return Self {
			canvas: c,
			color: rgba!(1),
			width: None,
			height: None,
			offset: None,
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
	pub fn width(mut self, w: f32) -> Self {
		self.width = Some(w);
		return self;
	}
	pub fn height(mut self, h: f32) -> Self {
		self.height = Some(h);
		return self;
	}
	pub fn offset(mut self, offset: Vec2) -> Self {
		self.offset = Some(offset);
		return self;
	}
}

impl<'a> Drawable for Canvas<'a> {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		let mut sprite = sprite(&self.canvas.tex())
			.color(self.color)
			.flip(gfx::Flip::Y);

		if let Some(w) = self.width {
			sprite = sprite.width(w * ctx.dpi());
		}

		if let Some(h) = self.height {
			sprite = sprite.height(h * ctx.dpi());
		}

		if let Some(o) = self.offset {
			sprite = sprite.offset(o);
		}

		ctx.draw_t(
			mat4!()
				.s2(vec2!(1.0 / ctx.dpi() as f32))
				,
			&sprite
		)?;

		return Ok(());

	}

}

