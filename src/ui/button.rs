// wengwengweng

use super::*;

pub struct Button {
	text: &'static str,
	clicked: bool,
	pressed: bool,
	hovering: bool,
}

impl Button {
	pub fn new(text: &'static str) -> Self {
		return Self {
			text: text,
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

	fn event(&mut self, d: &mut Ctx, e: &input::Event) {

		use input::Event::*;
		use input::Mouse;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left if self.hovering => {
						self.clicked = true;
						self.pressed = true;
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

	}

	fn draw(&mut self, gfx: &mut gfx::Gfx, wctx: &WidgetCtx) -> Result<f32> {

		use geom::*;

		let theme = &wctx.theme;

		let ptext = shapes::text(&format!("{}", self.text))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		let bw = ptext.width() + theme.padding * 2.0;
		let bh = ptext.height() + theme.padding * 2.0;

		let bg_color = if self.pressed {
			theme.border_color
		} else {
			theme.bar_color
		};

		gfx.draw(
			&shapes::rect(vec2!(0), vec2!(bw, -bh))
				.stroke(theme.border_color)
				.fill(bg_color)
				.line_width(2.0)
		)?;

		gfx.draw_t(
			mat4!()
				.t2(vec2!(theme.padding, -theme.padding))
				,
			&ptext
		)?;

		let rect = Rect::new(vec2!(0), vec2!(bw, -bh));

		self.hovering = col::intersect2d(rect, wctx.mouse_pos);
		self.clicked = false;

		return Ok(bh);

	}

}

