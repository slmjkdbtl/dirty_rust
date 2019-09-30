// wengwengweng

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use crate::Result;

pub fn get() -> Result<String> {
	let mut ctx: ClipboardContext = ClipboardProvider::new()?;
	return Ok(ctx.get_contents()?);
}

pub fn set(content: String) -> Result<()> {
	let mut ctx: ClipboardContext = ClipboardProvider::new()?;
	return Ok(ctx.set_contents(content)?);
}

