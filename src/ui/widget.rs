// wengwengweng

use std::any::Any;
use super::*;

pub trait AsAny {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
	fn as_any(&self) -> &dyn Any {
		return self;
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		return self;
	}
}

/// Trait for Building a Widget
pub trait Widget: AsAny + 'static {
	// TODO: take WidgetCtx
	/// process widget events, return if an event is processed and should stop propogation
	fn event(&mut self, _: &Event) -> bool {
		return false;
	}
	/// draw widget, return the final widget height
	fn draw(&mut self, _: &mut Gfx, _: &WidgetCtx) -> Result<f32> {
		return Ok(0.0);
	}
	/// if a widget is focused it'll be the first to process event and will be drawn on top
	fn focused(&self) -> bool {
		return false;
	}
}

