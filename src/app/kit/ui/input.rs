// wengwengweng

use super::*;

pub struct Input {
	focused: bool,
}

impl Input {
	pub fn new() -> Self {
		return Self {
			focused: false,
		};
	}
}

impl Widget for Input {

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

		return Ok(RenderResult {
			height: 0.0,
		});

	}

}

