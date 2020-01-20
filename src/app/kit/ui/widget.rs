// wengwengweng

use super::*;

pub trait Widget {
	fn event(&mut self, ctx: &mut app::Ctx, panel: &PanelCtx, e: &app::input::Event);
	fn draw(&self, ctx: &mut app::Ctx, panel: &PanelCtx) -> Result<()>;
	fn height(&self, theme: &Theme) -> f32;
}

