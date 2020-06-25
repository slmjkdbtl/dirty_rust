// wengwengweng

use super::*;

pub struct CheckBox {
	prompt: &'static str,
	checked: bool,
	hovering: bool,
}

impl CheckBox {
	pub fn new(prompt: &'static str, checked: bool,) -> Self {
		return Self {
			prompt,
			checked,
			hovering: false,
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
						return true;
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
		let sep = 12.0;

		let label_shape = shapes::text(&self.prompt.to_string())
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

		// draw box
		gfx.draw(
			&shapes::rect(vec2!(0), vec2!(size, -size))
				.stroke(theme.border_color)
				.fill(fill)
				.line_width(2.0)
		)?;

		// draw checked fill
		if self.checked {
			gfx.draw(
				&shapes::rect(vec2!(4, -4), vec2!(size, -size) + vec2!(-4, 4))
					.fill(theme.border_color)
					.stroke(theme.border_color)
					.line_width(1.0)
			)?;
		}

		// check mouse hover
		let rect = Rect::new(vec2!(0), vec2!(size + sep + label_shape.width(), -size));

		self.hovering = col::intersect2d(rect, ctx.mouse_pos());

		return Ok(size);

	}

}

