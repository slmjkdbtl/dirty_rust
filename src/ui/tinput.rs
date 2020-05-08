// wengwengweng

use super::*;
use kit::textedit;

pub struct Input {
	buf: textedit::Input,
	prompt: &'static str,
	hovering: bool,
	focused: bool,
}

impl Input {
	pub fn new(prompt: &'static str,) -> Self {
		return Self {
			buf: textedit::Input::new(),
			prompt: prompt,
			focused: false,
			hovering: false,
		};
	}
	pub fn text(&self) -> String {
		return self.buf.content().to_string();
	}
}

impl Widget for Input {

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) {

		use input::Event::*;
		use input::Key;
		use input::Mouse;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left => self.focused = self.hovering,
					_ => {},
				}
			},
			_ => {},
		}

		if !self.focused {
			return;
		}

		match e {
			KeyPressRepeat(k) => {
				match *k {
					Key::Left => self.buf.move_left(),
					Key::Right => self.buf.move_right(),
					Key::Back => self.buf.del(),
					_ => {},
				}
			},
			CharInput(ch) => {
				self.buf.insert(*ch)
			},
			_ => {},
		}

	}

	fn draw(&mut self, ctx: &mut Ctx, wctx: &WidgetCtx) -> Result<f32> {

		use geom::*;

		let mut height = 0.0;

		let ptext = shapes::text(&format!("{}:", self.prompt))
			.size(wctx.theme.font_size)
			.color(wctx.theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		height += ptext.height() + wctx.theme.margin * 0.8;

		ctx.draw(&ptext)?;

		let itext = shapes::text(self.buf.content())
			.size(wctx.theme.font_size)
			.color(wctx.theme.border_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		let cpos = itext.cursor_pos(self.buf.cursor() as usize);

		let padding = 9.0;
		let box_height = itext.height() + padding * 2.0;

		let rect = Rect::new(vec2!(0, -height), vec2!(wctx.width, -height - box_height));
		let mpos = ctx.mouse_pos() - wctx.offset;

		self.hovering = col::intersect2d(rect, mpos);

		let c = if self.focused {
			wctx.theme.bar_color.brighten(0.1)
		} else {
			wctx.theme.bar_color
		};

		ctx.draw(
			&shapes::rect(
				vec2!(0, -height),
				vec2!(wctx.width - 4.0, -height - box_height)
			)
				.stroke(wctx.theme.border_color)
				.line_width(2.0)
				.fill(c)
		)?;

		ctx.draw_t(
			mat4!()
				.t2(vec2!(padding, -height - padding))
				,
			&itext
		)?;

		if self.focused {

			if let Some(cpos) = cpos {

				ctx.draw(
					&shapes::line(
						cpos + vec2!(padding + 3.0, -height - padding + 3.0),
						cpos + vec2!(padding + 3.0, -height - padding - itext.height() - 3.0),
					)
						.width(2.0)
						.color(wctx.theme.border_color)
				)?;

			}

		}

		height += box_height;

		return Ok(height);

	}

}

