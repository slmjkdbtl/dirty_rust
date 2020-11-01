// wengwengweng

use super::*;

/// Audio Buffer in Memory
#[derive(Clone)]
pub struct AudioBuffer {
	frames: Vec<Frame>,
	sample_rate: u32,
}

impl AudioBuffer {

	pub fn from_frames(frames: Vec<Frame>, sample_rate: u32) -> Self {
		return Self {
			frames: frames,
			sample_rate: sample_rate,
		};
	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {
		let src = Decoder::new(Cursor::new(data.to_owned()))?;
		return Ok(Self::from_source(src));
	}

	pub fn from_source(src: impl Source) -> Self {

		let sample_rate = src.sample_rate();
		let frames = src.into_iter().collect::<Vec<Frame>>();

		return Self {
			sample_rate: sample_rate,
			frames: frames,
		};

	}

	pub fn duration(&self) -> Duration {
		return Duration::from_secs_f32(self.frames.len() as f32 / self.sample_rate as f32);
	}

	pub fn frames(&self) -> &[Frame] {
		return &self.frames;
	}

	pub fn frames_mut(&mut self) -> &mut [Frame] {
		return &mut self.frames;
	}

	pub fn sample_rate(&self) -> u32 {
		return self.sample_rate;
	}

	pub fn process(&mut self, e: &mut impl Effect) {
		for f in &mut self.frames {
			*f = e.process(*f);
		}
	}

}

