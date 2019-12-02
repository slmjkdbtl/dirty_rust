// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Canvas<'a> {
	canvas: &'a gfx::Canvas,
	color: Color,
}

pub fn canvas<'a>(c: &'a gfx::Canvas) -> Canvas<'a> {
	return Canvas::new(c);
}

impl<'a> Canvas<'a> {
	pub fn new(c: &'a gfx::Canvas) -> Self {
		return Self {
			canvas: c,
			color: rgba!(1),
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
}

impl<'a> Drawable for Canvas<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.push(&gfx::t()
			.s2(vec2!(1.0 / ctx.dpi() as f32))
		, |ctx| {
			return ctx.draw(&sprite(&self.canvas.tex()).color(self.color));
		})?;

		return Ok(());

	}

}

