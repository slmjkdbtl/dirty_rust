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

