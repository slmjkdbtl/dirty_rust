// wengwengweng

use super::*;

pub struct CheckBox {
	label: &'static str,
	checked: bool,
	hovering: bool,
	pressing: bool,
}

impl CheckBox {
	pub fn new(label: &'static str, checked: bool,) -> Self {
		return Self {
			label: label,
			checked: checked,
			hovering: false,
			pressing: false,
		};
	}
	pub fn checked(&self) -> bool {
		return self.checked;
	}
}

impl Widget for CheckBox {

	fn event(&mut self, e: &Event) -> bool {

		use Event::*;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left if self.hovering => {
						self.checked = !self.checked;
						self.pressing = true;
						return true;
					}
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
		let size = theme.font_size + 6.0;
		let sep = theme.padding;

		let label_shape = shapes::text(self.label)
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		// draw label
		gfx.draw_t(mat4!().t2(vec2!(size + sep, -3.0)), &label_shape)?;

		let fill = if self.checked {
			theme.bar_color
		} else {
			rgba!(0, 0, 0, 0)
		};

		let scale = if self.pressing {
			0.95
		} else {
			1.0
		};

		// draw box
		gfx.draw(
			&shapes::rect(
				vec2!(size * (1.0 - scale), -size * (1.0 - scale)),
				vec2!(size * scale, -size * scale),
			)
				.stroke(theme.border_color)
				.line_join(shapes::LineJoin::Round)
				.fill(fill)
				.line_width(theme.line_width)
		)?;

		// draw checked fill
		if self.checked {
			gfx.draw(
				&shapes::rect(
					vec2!(size * (1.0 - scale) + 4.0, -size * (1.0 - scale) - 4.0),
					vec2!(size, -size) * scale + vec2!(-4, 4)
				)
					.fill(theme.border_color)
					.stroke(theme.border_color)
					.line_width(theme.line_width)
					.line_join(shapes::LineJoin::Round)
			)?;
		}

		// check mouse hover
		let area = Rect::new(vec2!(0), vec2!(size + sep + label_shape.width(), -size));

		self.hovering = col::intersect2d(area, ctx.mouse_pos());

		return Ok(size);

	}

}

