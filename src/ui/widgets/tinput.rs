// wengwengweng

use super::*;
use kit::textinput;

pub struct Input {
	buf: textinput::Input,
	label: &'static str,
	hovering: bool,
	focused: bool,
	key_mods: KeyMod,
}

impl Input {
	pub fn new(label: &'static str,) -> Self {
		return Self {
			buf: textinput::Input::new(),
			label: label,
			focused: false,
			hovering: false,
			key_mods: KeyMod::empty(),
		};
	}
	pub fn text(&self) -> String {
		return self.buf.content().to_string();
	}
}

impl Widget for Input {

	fn event(&mut self, e: &Event) -> bool {

		use Event::*;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left => {
						if self.hovering {
							self.focused = true;
							return true;
						} else {
							self.focused = false;
						}
					}
					_ => {},
				}
			},
			_ => {},
		}

		if !self.focused {
			return false;
		}

		match e {
			KeyPressRepeat(k) => {
				match *k {
					Key::Left => {
						if self.key_mods.alt {
							self.buf.move_prev_word();
						} else {
							self.buf.move_left();
						}
						return true;
					},
					Key::Right => {
						if self.key_mods.alt {
							self.buf.move_next_word();
						} else {
							self.buf.move_right();
						}
						return true;
					},
					Key::Backspace => {
						if self.key_mods.alt {
							self.buf.del_word();
						} else {
							self.buf.del();
						}
						return true;
					},
					_ => {},
				}
			},
			CharInput(ch) => {
				self.buf.insert(*ch);
				return true;
			},
			_ => {},
		}

		return false;

	}

	fn draw(&mut self, gfx: &mut gfx::Gfx, ctx: &WidgetCtx) -> Result<f32> {

		use geom::*;

		let mut y = 0.0;
		let theme = ctx.theme();

		let label_shape = shapes::text(&format!("{}:", self.label))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		y += label_shape.height() + theme.padding;

		// draw label
		gfx.draw(&label_shape)?;

		// init input text shape
		let input_shape = shapes::text(self.buf.content())
			.size(theme.font_size)
			.color(theme.border_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		// calc box area
		let box_height = input_shape.height() + theme.padding * 2.0;
		let box_area = Rect::new(vec2!(0, -y), vec2!(ctx.width(), -y - box_height));

		// calc mouse hover
		self.hovering = col::intersect2d(box_area, ctx.mouse_pos);

		let bg_color = if self.focused {
			theme.bar_color.brighten(0.1)
		} else {
			theme.bar_color
		};

		// draw box
		gfx.draw(
			&shapes::rect(box_area.p1, box_area.p2)
				.stroke(theme.border_color)
				.line_width(2.0)
				.fill(bg_color)
		)?;

		// input text shouldn't be drawn outside box
		gfx.draw_within(box_area.p1, box_area.p2, |gfx| {

			// draw input text
			gfx.draw_t(
				mat4!()
					.t2(vec2!(theme.padding, -theme.padding))
					,
				&input_shape
			)?;

			// draw cursor
			if self.focused {

				if let Some(cpos) = input_shape.cursor_pos(self.buf.cursor() as usize) {

					gfx.draw(
						&shapes::line(
							cpos + vec2!(theme.padding + 1.0, -theme.padding + 2.0),
							cpos + vec2!(theme.padding + 1.0, -theme.padding - input_shape.height() - 2.0),
						)
							.width(2.0)
							.color(theme.border_color)
					)?;

				}

			}

			return Ok(());

		})?;

		y += box_height;
		self.key_mods = ctx.key_mods();

		return Ok(y);

	}

	fn focused(&self) -> bool {
		return self.focused;
	}

}

