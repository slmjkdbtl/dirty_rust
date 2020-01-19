// wengwengweng

use super::*;

pub trait Widget {

	fn event(&mut self, ctx: &mut app::Ctx, e: &app::input::Event);
	fn draw(&self, ctx: &mut app::Ctx, info: &PanelCtx) -> Result<RenderResult>;

	fn title(&self) -> Option<String> {
		return None;
	}

}

#[derive(Clone, Copy, Debug)]
pub struct RenderResult {
	pub height: f32,
}

