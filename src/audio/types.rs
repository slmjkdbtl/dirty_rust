// wengwengweng

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Channel {
	Left,
	Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChannelCount {
	One,
	Two,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SampleRate {
	Hz44100,
	Hz48000,
}

