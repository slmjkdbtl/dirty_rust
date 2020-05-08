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

		let mut height = 0.0;
		let size = wctx.theme.font_size + 6.0;
		let padding = 12.0;

		let ptext = shapes::text(&format!("{}", self.prompt))
			.size(wctx.theme.font_size)
			.color(wctx.theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		height += ptext.height() + wctx.theme.margin * 0.8;

		ctx.draw_t(mat4!().t2(vec2!(size + padding, -3.0)), &ptext)?;

		let fill = if self.checked {
			wctx.theme.bar_color
		} else {
			rgba!(0, 0, 0, 0)
		};

		ctx.draw(
			&shapes::rect(vec2!(0), vec2!(size, -size))
				.stroke(wctx.theme.border_color)
				.fill(fill)
				.line_width(2.0)
		)?;

		if self.checked {

			ctx.draw(
				&shapes::rect(vec2!(4, -4), vec2!(size, -size) + vec2!(-4, 4))
					.fill(wctx.theme.border_color)
					.stroke(wctx.theme.border_color)
					.line_width(1.0)
			)?;

		}

		let rect = Rect::new(vec2!(0), vec2!(size + padding + ptext.width(), -height - 6.0));
		let mpos = ctx.mouse_pos() - wctx.offset;

		self.hovering = col::intersect2d(rect, mpos);

		return Ok(height + 6.0);

	}

}

