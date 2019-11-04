// wengwengweng

pub use random;

use random::seq::SliceRandom;
use random::Rng as RngTrait;
use random::SeedableRng;

pub type RandSeed = u64;

#[macro_export]
macro_rules! rand {

	() => {
		$crate::math::random::random::<f32>()
	};

	($from:expr, $to:expr) => {
		$crate::math::random::Rng::gen_range(&mut random::thread_rng(), $from, $to)
	};

	($x:expr) => {
		$crate::rand!(0.0, $x)
	};

}

#[macro_export]
macro_rules! choose {

	($c1:expr, $c2:expr) => {
		{
			let _tmp = rand!();
			if _tmp < 1.0 / 2.0 {
				$c1
			} else {
				$c2
			}
		}
	};

	($c1:expr, $c2:expr, $c3:expr) => {
		{
			let _tmp = rand!();
			if _tmp < 1.0 / 2.0 {
				$c1
			} else {
				$c2
			}
		}
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

