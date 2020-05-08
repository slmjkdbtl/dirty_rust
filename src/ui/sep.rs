// wengwengweng

use super::*;

pub struct Sep;

impl LightWidget for Sep {

	fn draw(&mut self, ctx: &mut Ctx, wctx: &WidgetCtx) -> Result<f32> {

		ctx.draw(
			&shapes::line(vec2!(0, -3), vec2!(wctx.width, -3))
				.color(wctx.theme.border_color)
				.width(2.0)
		)?;

		return Ok(6.0);

	}

}

