// wengwengweng

use super::*;

pub trait Source: Iterator<Item = Frame> {
	fn sample_rate(&self) -> SampleRate;
}

