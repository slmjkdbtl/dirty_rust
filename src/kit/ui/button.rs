// wengwengweng

use super::*;

pub struct Button {
	text: &'static str,
	pressed: bool,
}

impl Button {
	pub fn new(text: &'static str) -> Self {
		return Self {
			text: text,
			pressed: false,
		};
	}
	pub fn pressed(&self) -> bool {
		return self.pressed;
	}
}

impl Widget for Button {

	fn draw(&mut self, ctx: &mut Ctx, wctx: &WidgetCtx) -> Result<f32> {

		let text = shapes::text(&self.text)
			.size(wctx.theme.font_size)
			.color(wctx.theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		ctx.draw(&text)?;

		return Ok(text.height());

	}

}

