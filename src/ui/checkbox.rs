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
			prompt: prompt,
			checked: checked,
			hovering: false,
		};
	}
	pub fn checked(&self) -> bool {
		return self.checked;
	}
}

impl Widget for CheckBox {

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) {

		use input::Event::*;
		use input::Key;
		use input::Mouse;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left if self.hovering => self.checked = !self.checked,
					_ => {},
				}
			},
			_ => {},
		}

	}

	fn draw(&mut self, ctx: &mut Ctx, wctx: &WidgetCtx) -> Result<f32> {

		use geom::*;

		let theme = &wctx.theme;
		let size = theme.font_size + 6.0;
		let sep = 12.0;

		let ptext = shapes::text(&format!("{}", self.prompt))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		ctx.draw_t(mat4!().t2(vec2!(size + sep, -3.0)), &ptext)?;

		let fill = if self.checked {
			theme.bar_color
		} else {
			rgba!(0, 0, 0, 0)
		};

		ctx.draw(
			&shapes::rect(vec2!(0), vec2!(size, -size))
				.stroke(theme.border_color)
				.fill(fill)
				.line_width(2.0)
		)?;

		if self.checked {

			ctx.draw(
				&shapes::rect(vec2!(4, -4), vec2!(size, -size) + vec2!(-4, 4))
					.fill(theme.border_color)
					.stroke(theme.border_color)
					.line_width(1.0)
			)?;

		}

		let rect = Rect::new(vec2!(0), vec2!(size + sep + ptext.width(), -size));
		let mpos = ctx.mouse_pos() - wctx.offset;

		self.hovering = col::intersect2d(rect, mpos);

		return Ok(size);

	}

}

