// wengwengweng

const A4_FREQ: f32 = 440.0;
const A4_NOTE: i32 = 69;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
	C,
	Cs,
	Db,
	D,
	Ds,
	Eb,
	E,
	F,
	Fs,
	Gb,
	G,
	Gs,
	Ab,
	A,
	As,
	Bb,
	B,
}

impl Key {

	pub fn to_n(self) -> i32 {

		return match self {
			Key::C => 0,
			Key::Cs => 1,
			Key::Db => 1,
			Key::D => 2,
			Key::Ds => 3,
			Key::Eb => 3,
			Key::E => 4,
			Key::F => 5,
			Key::Fs => 6,
			Key::Gb => 6,
			Key::G => 7,
			Key::Gs => 8,
			Key::Ab => 8,
			Key::A => 9,
			Key::As => 10,
			Key::Bb => 10,
			Key::B => 11,
		};

	}

	pub fn to_note(self, o: i32) -> Note {
		return Note::from_key(self, o);
	}

}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Note {
	n: i32,
}

impl Note {

	pub fn new(n: i32) -> Self {
		return Self {
			n: n,
		};
	}

	pub fn from_octave(n: i32, o: i32) -> Self {
		return Self::new(n + (o + 1) * 12);
	}

	pub fn from_key(k: Key, o: i32) -> Self {
		return Self::from_octave(k.to_n(), o);
	}

	pub fn freq(&self) -> f32 {
		return A4_FREQ * f32::powi(f32::powf(2.0, 1.0 / 12.0), self.n - A4_NOTE);
	}

}

impl From<i32> for Note {
	fn from(n: i32) -> Note {
		return Note::new(n);
	}
}

impl From<(i32, i32)> for Note {
	fn from((n, o): (i32, i32)) -> Note {
		return Note::from_octave(n, o);
	}
}

impl From<(Key, i32)> for Note {
	fn from((k, o): (Key, i32)) -> Note {
		return Note::from_key(k, o);
	}
}

