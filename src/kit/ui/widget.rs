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

pub trait Widget: AsAny + 'static {
	fn draw(&self, _: &mut Ctx, _: &PanelCtx) -> Result<f32> {
		return Ok(0.0);
	}
}

