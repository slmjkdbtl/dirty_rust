// wengwengweng

use super::*;

pub struct CheckBox {
	text: String,
	checked: bool,
	pressed: bool,
}

impl CheckBox {
	pub fn new(text: &str) -> Self {
		return Self {
			text: String::from(text),
			checked: false,
			pressed: false,
		};
	}
}

impl Widget for CheckBox {

	fn event(&mut self, ctx: &mut app::Ctx, panel: &PanelCtx, e: &app::input::Event) {

		use app::input::Event::*;
		use app::input::Mouse;
		use app::kit::geom;
		use geom::Shape2D;

		match e {

			MousePress(m) => {

				match *m {

					Mouse::Left => {

						let mpos = ctx.mouse_pos();
						let theme = &panel.theme;

						if geom::overlaps(
							Shape2D::Point(mpos),
							Shape2D::Rect(panel.pos, panel.pos + vec2!(panel.width - theme.padding.x * 2.0, -self.height(theme))),
						) {
							self.pressed = true;
							self.checked = !self.checked;
						}

					},

					_ => {},

				}

			},

			MouseRelease(m) => {

				match *m {

					Mouse::Left => {
						self.pressed = false;
					},

					_ => {},

				}

			},

			_ => {},

		}

	}

	fn draw(&self, ctx: &mut app::Ctx, panel: &PanelCtx) -> Result<()> {

		let theme = &panel.theme;
		let fh = theme.font_size;
		let box_size = fh + 2.0;

		ctx.draw(
			&shapes::rect(vec2!(0), vec2!(box_size, -box_size))
				.stroke(theme.border_color)
				.fill(theme.bar_color)
				.line_width(2.0)
		)?;

		if self.checked {
			ctx.draw(
				&shapes::line(vec2!(box_size * 0.2, -box_size * 0.5), vec2!(box_size * 0.4, -box_size * 0.8))
					.color(theme.border_color)
					.width(2.0)
			)?;
			ctx.draw(
				&shapes::line(vec2!(box_size * 0.4, -box_size * 0.8), vec2!(box_size * 0.8, -box_size * 0.3))
					.color(theme.border_color)
					.width(2.0)
			)?;
// 			ctx.draw(
// 				&shapes::rect(vec2!(3, -3), vec2!(box_size - 3.0, 3.0 - box_size))
// 					.fill(theme.border_color)
// 			)?;
		}

		ctx.draw_t(
			mat4!()
				.t2(vec2!(fh + 9.0, -1.0)),
			&shapes::text(&self.text)
				.size(fh)
				.color(theme.border_color)
				.align(gfx::Origin::TopLeft)
		)?;

		return Ok(());

	}

	fn height(&self, theme: &Theme) -> f32 {

		let fh = theme.font_size;

		return fh + 2.0;

	}

}

