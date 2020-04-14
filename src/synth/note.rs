// wengwengweng

const A4_FREQ: f32 = 440.0;
const A4_NOTE: i32 = 69;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Note(pub i32);

impl Note {

	pub fn from_octave(i: i32, o: i32) -> Self {
		return Note(i + 12 * (o + 1));
	}

	pub fn freq(&self) -> f32 {
		return A4_FREQ * f32::powi(f32::powf(2.0, 1.0 / 12.0), self.0 - A4_NOTE);
	}

}

