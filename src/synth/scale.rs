// wengwengweng

use super::*;

pub const MAJOR: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
pub const MINOR: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
pub const PENTA: [i32; 5] = [0, 2, 4, 7, 9];

pub struct Scale {
	key: Note,
	scale: Vec<i32>,
}

impl Scale {

	pub fn new(key: Note, s: &[i32]) -> Self {
		return Self {
			key: key,
			scale: s.to_vec(),
		}
	}

	pub fn get(&self, i: i32) -> NoteO {

		let mut o = i / self.scale.len() as i32;
		let mut n = i % self.scale.len() as i32;
		let base = self.key.to_num();

		if n < 0 {
			n = self.scale.len() as i32 + n;
		}

		if i < 0 {
			o -= 12;
		}

		let nn = o * 12 + self.scale[n as usize];

		return NoteO::from_num(base + nn);

	}

}

