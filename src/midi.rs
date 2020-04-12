// wengwengweng

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
	NoteOn(i32, f32),
	NoteOff(i32, f32),
	Continuous(i32, f32),
	Pitch(f32, f32),
	Unknown(Vec<u8>)
}

fn join<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
	return match (a, b) {
		(Some(a), Some(b)) => Some((a, b)),
		_ => None,
	};
}

impl Msg {

	pub fn from(msg: &[u8]) -> Msg {

		let first = match msg.get(0) {
			Some(b) => b,
			None => return Msg::Unknown(msg.to_vec()),
		};

		let second = msg.get(1);
		let third = msg.get(2);

		match first {
			// note off
			0x80..=0x8f => {
				if let Some((note, vel)) = join(second, third) {
					return Msg::NoteOff(*note as i32, *vel as f32 / 128.0);
				}
			},
			// note on
			0x90..=0x9f => {
				if let Some((note, vel)) = join(second, third) {
					return Msg::NoteOn(*note as i32, *vel as f32 / 128.0);
				}
			},
			// continuous
			0xb0..=0xbf => {
				if let Some((id, val)) = join(second, third) {
					return Msg::Continuous(*id as i32, *val as f32 / 128.0);
				}
			},
			// pitch
			0xe0..=0xef => {
				if let Some((lsb, msb)) = join(second, third) {
					return Msg::Pitch(*lsb as f32 / 128.0, *msb as f32 / 128.0);
				}
			},
			_ => {},
		}

		return Msg::Unknown(msg.to_vec());

	}

}

