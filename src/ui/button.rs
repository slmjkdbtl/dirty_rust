// wengwengweng

use super::*;

pub struct Button {
	label: &'static str,
	clicked: bool,
	pressed: bool,
	hovering: bool,
}

impl Button {
	pub fn new(label: &'static str) -> Self {
		return Self {
			label: label,
			clicked: false,
			pressed: false,
			hovering: false,
		};
	}
	pub fn clicked(&self) -> bool {
		return self.clicked;
	}
}

impl Widget for Button {

	fn event(&mut self, e: &Event) -> bool {

		use Event::*;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left if self.hovering => {
						self.clicked = true;
						self.pressed = true;
						return true;
					},
					_ => {},
				}
			},
			MouseRelease(m) => {
				match *m {
					Mouse::Left => self.pressed = false,
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

		let label_shape = shapes::text(&self.label.to_string())
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		// calc button size
		let box_width = label_shape.width() + theme.padding * 2.0;
		let box_height = label_shape.height() + theme.padding * 2.0;

		let bg_color = if self.pressed {
			theme.border_color
		} else {
			theme.bar_color
		};

		// draw box
		gfx.draw(
			&shapes::rect(vec2!(0), vec2!(box_width, -box_height))
				.stroke(theme.border_color)
				.fill(bg_color)
				.line_width(2.0)
		)?;

		// draw label
		gfx.draw_t(
			mat4!()
				.t2(vec2!(theme.padding, -theme.padding))
				,
			&label_shape
		)?;

		// check mouse hover
		let rect = Rect::new(vec2!(0), vec2!(box_width, -box_height));

		self.hovering = col::intersect2d(rect, ctx.mouse_pos());
		self.clicked = false;

		return Ok(box_height);

	}

}

