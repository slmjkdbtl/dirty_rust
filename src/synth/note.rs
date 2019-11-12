// wengwengweng

const FREQ_A: f32 = 440.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Note {
	C(i32),
	Csh(i32),
	Db(i32),
	D(i32),
	Dsh(i32),
	Eb(i32),
	E(i32),
	F(i32),
	Fsh(i32),
	Gb(i32),
	G(i32),
	Gsh(i32),
	Ab(i32),
	A(i32),
	Ash(i32),
	Bb(i32),
	B(i32),
}

impl Note {

	pub fn to_freq(&self) -> i32 {

		let offset = match self {
			Note::C(o) => -9 + o * 12,
			Note::Csh(o) => -8 + o * 12,
			Note::Db(o) => -8 + o * 12,
			Note::D(o) => -7 + o * 12,
			Note::Dsh(o) => -6 + o * 12,
			Note::Eb(o) => -6 + o * 12,
			Note::E(o) => -5 + o * 12,
			Note::F(o) => -4 + o * 12,
			Note::Fsh(o) => -3 + o * 12,
			Note::Gb(o) => -3 + o * 12,
			Note::G(o) => -2 + o * 12,
			Note::Gsh(o) => -1 + o * 12,
			Note::Ab(o) => -1 + o * 12,
			Note::A(o) => 0 + o * 12,
			Note::Ash(o) => 1 + o * 12,
			Note::Bb(o) => 1 + o * 12,
			Note::B(o) => 2 + o * 12,
		};

		return (FREQ_A * f32::powi(f32::powf(2.0, 1.0 / 12.0), offset)) as i32;

	}

}

