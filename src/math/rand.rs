// wengwengweng

use super::*;

use random;
use random::seq::SliceRandom;
use random::SeedableRng;

pub type RandSeed = u64;

pub trait Rand {
	fn gen_rand(r1: Self, r2: Self) -> Self;
	fn gen_rand_from_rng(r1: Self, r2: Self, rng: &mut Rng) -> Self;
}

macro_rules! impl_rand {

	($ty:ty) => {

		impl Rand for $ty {

			fn gen_rand(r1: Self, r2: Self) -> Self {
				if r1 == r2 {
					return r1;
				}
				return random::Rng::gen_range(&mut random::thread_rng(), <$ty>::min(r1, r2), <$ty>::max(r1, r2));
			}

			fn gen_rand_from_rng(r1: Self, r2: Self, rng: &mut Rng) -> Self {
				if r1 == r2 {
					return r1;
				}
				return random::Rng::gen_range(&mut rng.rng, <$ty>::min(r1, r2), <$ty>::max(r1, r2));
			}

		}

	}

}

macro_rules! impl_rand_vec {

	($ty:ty, $($mem:ident),*$(,)?) => {

		impl Rand for $ty {

			fn gen_rand(r1: Self, r2: Self) -> Self {
				return Self {
					$(
						$mem: rand(r1.$mem, r2.$mem),
					)*
				};
			}

			fn gen_rand_from_rng(r1: Self, r2: Self, rng: &mut Rng) -> Self {
				return Self {
					$(
						$mem: rng.gen(r1.$mem, r2.$mem),
					)*
				};
			}

		}

	}

}

impl_rand!(f32);
impl_rand!(f64);
impl_rand!(i8);
impl_rand!(i16);
impl_rand!(i32);
impl_rand!(i64);
impl_rand!(u8);
impl_rand!(u16);
impl_rand!(u32);
impl_rand!(u64);
impl_rand!(usize);
impl_rand_vec!(Vec2, x, y);
impl_rand_vec!(Vec3, x, y, z);
impl_rand_vec!(Vec4, x, y, z, w);
impl_rand_vec!(Color, r, g, b, a);

pub fn rand<R: Rand>(r1: R, r2: R) -> R {
	return R::gen_rand(r1, r2);
}

pub fn rand_t<R: Rand>(r: (R, R)) -> R {
	return R::gen_rand(r.0, r.1);
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

	pub fn gen<R: Rand>(&mut self, r1: R, r2: R) -> R {
		return R::gen_rand_from_rng(r1, r2, self);
	}

}

#[macro_export]
macro_rules! choose {

	($c1:expr, $c2:expr) => {
		{
			let _rnd = rand((0.0, 1.0));
			let _step = 1.0 / 2.0;
			if _rnd < _step * 1.0 {
				$c1
			} else {
				$c2
			}
		}
	};

	($c1:expr, $c2:expr, $c3:expr) => {
		{
			let _rnd = rand((0.0, 1.0));
			let _step = 1.0 / 3.0;
			if _rnd < _step * 1.0 {
				$c1
			} else if _rnd > _step * 1.0 && _rnd < _step * 2.0 {
				$c2
			} else {
				$c3
			}
		}
	};

	($c1:expr, $c2:expr, $c3:expr, $c4: expr) => {
		{
			let _rnd = rand((0.0, 1.0));
			let _step = 1.0 / 4.0;
			if _rnd < _step * 1.0 {
				$c1
			} else if _rnd > _step * 1.0 && _rnd < _step * 2.0 {
				$c2
			} else if _rnd > _step * 2.0 && _rnd < _step * 3.0 {
				$c3
			} else {
				$c4
			}
		}
	};

}
