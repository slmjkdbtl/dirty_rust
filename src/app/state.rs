// wengwengweng

use crate::*;
use super::*;

pub trait State: Sized {

	fn init(_: &mut Ctx) -> Result<Self>;

	fn event(&mut self, _: &mut Ctx, _: input::Event) -> Result<()> {
		return Ok(());
	}

	fn update(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

	fn draw(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

	#[cfg(feature = "imgui")]
	fn imgui(&self, _: &imgui::Ui) -> Result<()> {
		return Ok(());
	}

	fn quit(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

}

impl State for () {
	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(());
	}
}

