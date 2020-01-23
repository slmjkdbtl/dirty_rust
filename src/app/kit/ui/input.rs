// wengwengweng

use super::*;
use app::kit::textedit;

pub struct Input {
	prompt: String,
	input: textedit::Input,
	focused: bool,
	pressed: bool,
	cursor_timer: f32,
}

impl Input {
	pub fn new(prompt: &str) -> Self {
		return Self {
			prompt: String::from(prompt),
			input: textedit::Input::new(),
			focused: false,
			pressed: false,
			cursor_timer: 0.0,
		};
	}
}

impl Input {
	pub fn content(&self) -> &str {
		return &self.input.content();
	}
}

impl Widget for Input {

	fn event(&mut self, ctx: &mut app::Ctx, panel: &PanelCtx, e: &app::input::Event) {

		use app::input::Event::*;
		use app::input::Mouse;
		use app::input::Key;
		use app::kit::geom;
		use geom::Shape2D;

		match e {

			KeyPressRepeat(k) => {
				let mods = ctx.key_mods();
				match *k {
					Key::Back => {
						if self.focused {
							if ctx.key_down(Key::LAlt) || ctx.key_down(Key::RAlt) {
								self.input.del_word();
							} else {
								self.input.del();
							}
							self.cursor_timer = 0.0;
						}
					},
					Key::Left => {
						if self.focused {
							self.cursor_timer = 0.0;
							if mods.alt {
								self.input.move_prev_word();
							} else {
								self.input.move_left();
							}
						}
					},
					Key::Right => {
						if self.focused {
							self.cursor_timer = 0.0;
							if mods.alt {
								self.input.move_next_word();
							} else {
								self.input.move_right();
							}
						}
					},
					Key::Z => {
						if self.focused {
							if mods.meta {
								if mods.shift {
									self.input.redo();
								} else {
									self.input.undo();
								}
							}
							self.cursor_timer = 0.0;
						}
					},
					_ => {},
				}
			},

			MousePress(m) => {

				match *m {

					Mouse::Left => {

						let mpos = ctx.mouse_pos();
						let theme = &panel.theme;

						if geom::overlaps(
							Shape2D::Point(mpos),
							Shape2D::Rect(panel.pos, panel.pos + vec2!(panel.width - theme.padding.x * 2.0, -self.height(theme))),
						) {
							self.pressed = true;
							self.focused = true;
						} else {
							self.focused = false;
						}

					},

					_ => {},

				}

			},

			MouseRelease(m) => {

				match *m {

					Mouse::Left => {
						self.pressed = false;
					},

					_ => {},

				}

			},

			CharInput(ch) => {
				let mods = ctx.key_mods();
				if
					!mods.alt
					&& !mods.ctrl
					&& !mods.meta
				{
					if self.focused {
						self.input.insert(*ch);
					}
				}

			},

			_ => {},

		}

	}

	fn draw(&self, ctx: &mut app::Ctx, panel: &PanelCtx) -> Result<()> {

		let theme = &panel.theme;
		let fh = theme.font_size;
		let mut h = 0.0;

		ctx.draw(
			&shapes::text(&format!("{}:", &self.prompt))
				.size(fh)
				.color(theme.border_color)
				.align(gfx::Origin::TopLeft)
		)?;

		h += fh + 6.0;

		ctx.draw(
			&shapes::rect(vec2!(0, -h), vec2!(panel.width - theme.padding.x * 2.0, -h - fh - 12.0))
				.stroke(theme.border_color)
				.fill(theme.bar_color)
				.line_width(2.0)
		)?;

		let pos = self.input.cursor();
		let mut cpos = None;

		let text = shapes::text(self.input.content())
			.size(fh)
			.color(theme.border_color)
			.align(gfx::Origin::TopLeft)
			.render(ctx)
			;

		if let Some(p) = text.cursor_pos(ctx, pos) {
			cpos = Some(p);
		}

		if (self.cursor_timer * 2.0) as i32 % 2 == 0 {
			if self.focused {
				if let Some(cpos) = cpos {
					ctx.draw(
						&shapes::line(cpos + vec2!(8, -11), cpos + vec2!(8, -fh - 11.0))
							.width(2.0)
							.color(theme.border_color)
					)?;
				}
			}
		}

		ctx.draw_t(
			mat4!().t2(vec2!(6, -h - 6.0)),
			&text,
		)?;

		h += fh + 12.0;

		return Ok(());

	}

	fn height(&self, theme: &Theme) -> f32 {

		let fh = theme.font_size;
		let mut h = 0.0;

		h += fh + 6.0;
		h += fh + 12.0;

		return h;

	}

}

