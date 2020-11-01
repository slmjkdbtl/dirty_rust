// wengwengweng

use super::*;

pub trait Source: Iterator<Item = Frame> {
	fn sample_rate(&self) -> u32;
	fn seek_start(&mut self) -> Result<()> {
		return Ok(());
	}
}

pub trait Stream: Send {
	fn next(&mut self) -> Frame;
}

