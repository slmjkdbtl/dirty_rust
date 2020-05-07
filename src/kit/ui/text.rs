// wengwengweng

use super::*;

pub struct Text {
	pub(crate) text: String,
}

impl Text {
	pub fn new(text: &str) -> Self {
		return Self {
			text: text.to_string(),
		};
	}
}

impl Widget for Text {

	fn draw(&self, ctx: &mut Ctx, pctx: &PanelCtx) -> Result<f32> {

		let text = shapes::text(&self.text)
			.size(pctx.theme.font_size)
			.color(pctx.theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		ctx.draw(&text)?;

		return Ok(text.height());

	}

}

