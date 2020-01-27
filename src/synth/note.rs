// wengwengweng

const FREQ_A: f32 = 440.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NoteO {
	pub note: Note,
	pub octave: i32,
}

impl From<(Note, i32)> for NoteO {
	fn from((n, o): (Note, i32)) -> NoteO {
		return Self::new(n, o);
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Note {
	C,
	Csh,
	Db,
	D,
	Dsh,
	Eb,
	E,
	F,
	Fsh,
	Gb,
	G,
	Gsh,
	Ab,
	A,
	Ash,
	Bb,
	B,
}

impl Note {
	pub fn to_num(&self) -> i32 {
		return match self {
			Note::C => 0,
			Note::Csh => 1,
			Note::Db => 1,
			Note::D => 2,
			Note::Dsh => 3,
			Note::Eb => 3,
			Note::E => 4,
			Note::F => 5,
			Note::Fsh => 6,
			Note::Gb => 6,
			Note::G => 7,
			Note::Gsh => 8,
			Note::Ab => 8,
			Note::A => 9,
			Note::Ash => 10,
			Note::Bb => 10,
			Note::B => 11,
		};
	}
}

impl NoteO {

	pub fn new(n: Note, o: i32) -> Self {
		return Self {
			note: n,
			octave: o,
		}
	}

	pub fn from_num(n: i32) -> Self {
		let o = n / 12;
		return match n % 12 {
			-1 => (Note::Csh, o).into(),
			-2 => (Note::D, o).into(),
			-3 => (Note::Dsh, o).into(),
			-4 => (Note::E, o).into(),
			-5 => (Note::F, o).into(),
			-6 => (Note::Fsh, o).into(),
			-7 => (Note::G, o).into(),
			-8 => (Note::Gsh, o).into(),
			-9 => (Note::A, o).into(),
			-10 => (Note::Ash, o).into(),
			-11 => (Note::B, o).into(),
			0 => (Note::C, o).into(),
			1 => (Note::Csh, o).into(),
			2 => (Note::D, o).into(),
			3 => (Note::Dsh, o).into(),
			4 => (Note::E, o).into(),
			5 => (Note::F, o).into(),
			6 => (Note::Fsh, o).into(),
			7 => (Note::G, o).into(),
			8 => (Note::Gsh, o).into(),
			9 => (Note::A, o).into(),
			10 => (Note::Ash, o).into(),
			11 => (Note::B, o).into(),
			_ => unreachable!(),
		}
	}

	pub fn to_freq(&self) -> i32 {

		let o = self.octave;

		let offset = match self.note {
			Note::C => -9 + o * 12,
			Note::Csh => -8 + o * 12,
			Note::Db => -8 + o * 12,
			Note::D => -7 + o * 12,
			Note::Dsh => -6 + o * 12,
			Note::Eb => -6 + o * 12,
			Note::E => -5 + o * 12,
			Note::F => -4 + o * 12,
			Note::Fsh => -3 + o * 12,
			Note::Gb => -3 + o * 12,
			Note::G => -2 + o * 12,
			Note::Gsh => -1 + o * 12,
			Note::Ab => -1 + o * 12,
			Note::A => 0 + o * 12,
			Note::Ash => 1 + o * 12,
			Note::Bb => 1 + o * 12,
			Note::B => 2 + o * 12,
		};

		return (FREQ_A * f32::powi(f32::powf(2.0, 1.0 / 12.0), offset)) as i32;

	}

}

// impl Add<i32> for NoteO {

// 	type Output = Self;

// 	fn add(self, other: i32) -> Self {
// 	}

// }
