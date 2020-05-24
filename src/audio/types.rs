// wengwengweng

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Channel {
	Left,
	Right,
}

impl Channel {
	fn as_usize(&self) -> usize {
		return match self {
			Channel::Left => 0,
			Channel::Right => 1,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChannelCount {
	One,
	Two,
}

impl ChannelCount {
	pub(super) fn to_cpal(&self) -> cpal::ChannelCount {
		return match self {
			ChannelCount::One => 1,
			ChannelCount::Two => 2,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SampleRate {
	Hz44100,
	Hz48000,
}

impl SampleRate {
	pub(super) fn to_cpal(&self) -> cpal::SampleRate {
		return match self {
			SampleRate::Hz44100 => cpal::SampleRate(44100),
			SampleRate::Hz48000 => cpal::SampleRate(48000),
		};
	}
	pub fn as_f32(&self) -> f32 {
		return match self {
			SampleRate::Hz44100 => 44100.0,
			SampleRate::Hz48000 => 48000.0,
		};
	}
}

