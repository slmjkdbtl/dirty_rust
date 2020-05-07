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

	fn draw(&self, ctx: &mut Ctx, theme: &Theme) -> Result<f32> {

		let text = shapes::text(&self.text)
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		ctx.draw_t(
			mat4!(),
			&text,
		)?;

		return Ok(text.height());

	}

}

