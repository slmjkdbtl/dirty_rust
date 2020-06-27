// wengwengweng

use super::*;

pub struct Sep;

impl Widget for Sep {

	fn draw(&mut self, gfx: &mut gfx::Gfx, ctx: &WidgetCtx) -> Result<f32> {

		let t = ctx.theme();

		gfx.draw(
			&shapes::line(vec2!(0, -t.padding * 0.5), vec2!(ctx.width(), -t.padding * 0.5))
				.color(t.border_color)
				.width(t.line_width)
				.cap(shapes::LineCap::Round)
		)?;

		return Ok(t.padding);

	}

}

