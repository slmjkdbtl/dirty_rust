// wengwengweng

use std::fmt;
use std::cmp::Ordering;

use crate::*;

const MODS: [&str; 17] = [

	"super",
	"cool",
	"awesome",
	"handsome",
	"badass",
	"hotdog",
	"fallen",
	"haunted",
	"king",
	"doomed",
	"forbidden",
	"unstoppable",
	"flaming",
	"unholy",
	"infernal",
	"dwarven",
	"cursed",

];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Id(&'static str, usize);

impl Id {
	pub fn new(id: usize) -> Self {
		return Self(MODS[rand!(MODS.len()) as usize], id);
	}
}

impl Ord for Id {
	fn cmp(&self, other: &Id) -> Ordering {
		return self.1.cmp(&other.1);
	}
}

impl PartialOrd for Id {
	fn partial_cmp(&self, other: &Id) -> Option<Ordering> {
		return Some(self.1.cmp(&other.1));
	}
}

impl fmt::Display for Id {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "{}{}", self.0, self.1);
	}
}

#[derive(Default)]
pub struct IdGenerator {
	last: usize,
}

impl IdGenerator {

	pub fn new() -> Self {
		return Self::default();
	}

	pub fn get(&mut self) -> Id {
		self.last += rand!(2, 7) as usize;
		return Id::new(self.last);
	}

	pub fn clear(&mut self) {
		self.last = 0;
	}

}

