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

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) {

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

	fn draw(&mut self, ctx: &mut Ctx, wctx: &WidgetCtx) -> Result<f32> {

		use geom::*;

		let theme = &wctx.theme;
		let padding = 9.0;

		let ptext = shapes::text(&format!("{}", self.text))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		let bw = ptext.width() + padding * 2.0;
		let bh = ptext.height() + padding * 2.0;

		let bg_color = if self.pressed {
			theme.border_color
		} else {
			theme.bar_color
		};

		ctx.draw(
			&shapes::rect(vec2!(0), vec2!(bw, -bh))
				.stroke(theme.border_color)
				.fill(bg_color)
				.line_width(2.0)
		)?;

		ctx.draw_t(
			mat4!()
				.t2(vec2!(padding, -padding))
				,
			&ptext
		)?;

		let rect = Rect::new(vec2!(0), vec2!(bw, -bh));
		let mpos = ctx.mouse_pos() - wctx.offset;

		self.hovering = col::intersect2d(rect, mpos);
		self.clicked = false;

		return Ok(bh);

	}

}

