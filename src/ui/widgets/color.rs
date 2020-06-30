// wengwengweng

use super::*;

pub struct ColorPicker {
	label: &'static str,
	color: Color,
	hovering: bool,
	pressing: bool,
}

impl ColorPicker {
	pub fn new(label: &'static str, c: Color) -> Self {
		return Self {
			label: label,
			color: c,
			hovering: false,
			pressing: false,
		};
	}
	pub fn color(&self) -> Color {
		return self.color;
	}
}

impl Widget for ColorPicker {

	fn event(&mut self, e: &Event) -> bool {

		use Event::*;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left => {
						if self.hovering {
							self.color = rand(rgba!(0, 0, 0, 1), rgba!(1, 1, 1, 1));
							self.pressing = true;
							return true;
						}
					},
					_ => {},
				}
			},
			MouseRelease(m) => {
				match *m {
					Mouse::Left => {
						self.pressing = false;
					}
					_ => {},
				}
			},
			_ => {},
		}

		return false;

	}

	fn draw(&mut self, gfx: &mut gfx::Gfx, ctx: &WidgetCtx) -> Result<f32> {

		use geom::*;

		let theme = ctx.theme();

		let label_shape = shapes::text(&format!("{}:", self.label))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		let box_x = label_shape.width() + theme.padding;
		let height = label_shape.height() + theme.padding;
		let width = height * 2.0;

		// draw label
		gfx.draw_t(mat4!().ty(-theme.padding * 0.5), &label_shape)?;

		let scale = if self.pressing {
			0.95
		} else {
			1.0
		};

		let area = Rect::new(
			vec2!(box_x + width * (1.0 - scale), -height * (1.0 - scale)),
			vec2!(box_x + width * scale, -height * scale),
		);

		self.hovering = col::intersect2d(area, ctx.mouse_pos());

		gfx.draw(
			&shapes::Rect::from_rect(area)
				.fill(self.color)
				.stroke(theme.border_color)
				.line_width(theme.line_width)
				.line_join(shapes::LineJoin::Round)
		)?;

		return Ok(height);

	}

}

