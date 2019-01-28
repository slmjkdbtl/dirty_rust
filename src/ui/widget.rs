// wengwengweng

use std::any::Any;

/// components inside a window
pub trait Widget: Any {

	fn update(&mut self) {}
	fn draw(&self) {}
	fn get_type(&self) -> WidgetType;

}

pub enum WidgetType {
	Normal,
	Exclusive,
}

