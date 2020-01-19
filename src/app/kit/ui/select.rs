// wengwengweng

use super::*;
use std::string::ToString;

pub struct Select<T: ToString> {
	options: Vec<T>,
	cursor: usize,
	expanded: bool,
}

impl<T: ToString> Select<T> {
	pub fn new(options: Vec<T>, cursor: usize) -> Self {
		return Self {
			options: options,
			cursor: cursor,
			expanded: false,
		};
	}
	pub fn cur_option(&self) -> Option<&T> {
		return self.options.get(self.cursor);
	}
}

impl<T: ToString> Widget for Select<T> {

	fn event(&mut self, ctx: &mut app::Ctx, e: &app::input::Event) {

		use app::input::Event::*;
		use app::input::Mouse;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left => {
						let mpos = ctx.mouse_pos();
					},
					_ => {},
				}
			},
			_ => {},
		}

	}

	fn draw(&self, ctx: &mut app::Ctx, info: &PanelCtx) -> Result<RenderResult> {

		if let Some(option) = self.cur_option() {
			ctx.draw(
				&shapes::text(&option.to_string())
					.align(gfx::Origin::TopLeft)
			)?;
		}

		for option in &self.options {
			// ...
		}

		return Ok(RenderResult {
			height: 0.0,
		});

	}

}

