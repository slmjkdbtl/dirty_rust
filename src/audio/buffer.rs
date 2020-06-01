// wengwengweng

use std::io::Cursor;
use std::time::Duration;
use std::sync::Arc;

use super::*;

#[derive(Clone)]
pub struct AudioBuffer {
	frames: Arc<Vec<Frame>>,
	sample_rate: u32,
	duration: Duration,
}

impl AudioBuffer {

	pub fn from_bytes(data: &[u8]) -> Result<Self> {
		let src = Decoder::new(Cursor::new(data.to_owned()))?;
		return Ok(Self::from_source(src));
	}

	pub fn from_source(src: impl Source) -> Self {

		let sample_rate = src.sample_rate();
		let frames = src.into_iter().collect::<Vec<Frame>>();

		return Self {
			sample_rate: sample_rate,
			duration: Duration::from_secs_f32(frames.len() as f32 / sample_rate as f32),
			frames: Arc::new(frames),
		};

	}

	pub fn duration(&self) -> Duration {
		return self.duration;
	}

	pub fn playback(&self) -> AudioBufferPlayback {
		return AudioBufferPlayback {
			frames: self.frames.clone(),
			sample_rate: self.sample_rate,
			duration: self.duration,
			cur_pos: 0,
		};
	}

}

#[derive(Clone)]
pub struct AudioBufferPlayback {
	frames: Arc<Vec<Frame>>,
	sample_rate: u32,
	duration: Duration,
	cur_pos: usize,
}

impl Iterator for AudioBufferPlayback {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(frame) = self.frames.get(self.cur_pos) {
			self.cur_pos += 1;
			return Some(*frame);
		}
		return None;
	}

}

impl Source for AudioBufferPlayback {

	fn sample_rate(&self) -> u32 {
		return self.sample_rate;
	}

	fn seek_start(&mut self) -> Result<()> {
		self.cur_pos = 0;
		return Ok(());
	}

}

