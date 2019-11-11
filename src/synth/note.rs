// wengwengweng

const FREQ_A: f32 = 440.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NoteOctave {
	note: Note,
	octave: i32,
}

impl NoteOctave {

	pub fn new(n: Note, o: i32) -> Self {
		return Self {
			note: n,
			octave: o,
		};
	}

	pub fn to_freq(&self) -> i32 {

		let offset = match self.note {
			Note::C => -9,
			Note::Csh => -8,
			Note::Db => -8,
			Note::D => -7,
			Note::Dsh => -6,
			Note::Eb => -6,
			Note::E => -5,
			Note::F => -4,
			Note::Fsh => -3,
			Note::Gb => -3,
			Note::G => -2,
			Note::Gsh => -1,
			Note::Ab => -1,
			Note::A => 0,
			Note::Ash => 1,
			Note::Bb => 1,
			Note::B => 2,
		};

		let offset = offset + self.octave * 12;

		return (FREQ_A * f32::powi(f32::powf(2.0, 1.0 / 12.0), offset)) as i32;

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

	pub fn to_freq(&self, octave: i32) -> i32 {
		return NoteOctave::new(*self, octave).to_freq();
	}

}

