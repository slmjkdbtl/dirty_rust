// wengwengweng

use random::seq::SliceRandom;
use random::Rng as RngTrait;
use random::SeedableRng;

pub type RandSeed = u64;

#[macro_export]
macro_rules! rand {

	() => {
		random::random::<f32>()
	};

	($from:expr, $to:expr) => {
		random::Rng::gen_range(&mut random::thread_rng(), $from, $to)
	};

	($x:expr) => {
		$crate::rand!(0, $x)
	};

}

pub fn rand_from<'a, T>(t: &'a [T]) -> Option<&'a T> {
	return t.choose(&mut random::thread_rng())
}

pub struct Rng {
	rng: random::rngs::StdRng,
}

impl Rng {

	pub fn new() -> Self {
		return Self {
			rng: random::rngs::StdRng::from_entropy(),
		};
	}

	pub fn from_seed(s: RandSeed) -> Self {
		return Self {
			rng: random::rngs::StdRng::seed_from_u64(s),
		};
	}

	// TODO: generic
	pub fn gen(&mut self) -> f32 {
		return self.rng.gen();
	}

}

