// wengwengweng

use super::*;


use random::seq::SliceRandom;
use random::SeedableRng;

pub type RandSeed = u64;

pub trait Rand {
	fn rand() -> Self;
	fn rand_from_rng(rng: &mut Rng) -> Self;
}

pub trait RandRange {
	fn rand_within(r1: Self, r2: Self) -> Self;
	fn rand_within_from_rng(r1: Self, r2: Self, rng: &mut Rng) -> Self;
}

macro_rules! impl_rand {

	($ty:ty, $lo:expr, $hi:expr) => {

		impl Rand for $ty {

			fn rand() -> Self {
				return rand($lo, $hi);
			}

			fn rand_from_rng(rng: &mut Rng) -> Self {
				return rng.gen($lo, $hi);
			}

		}

	}

}

macro_rules! impl_rand_range {

	($ty:ty) => {

		impl RandRange for $ty {

			fn rand_within(r1: Self, r2: Self) -> Self {
				if r1 == r2 {
					return r1;
				}
				return random::Rng::gen_range(&mut random::thread_rng(), <$ty>::min(r1, r2), <$ty>::max(r1, r2));
			}

			fn rand_within_from_rng(r1: Self, r2: Self, rng: &mut Rng) -> Self {
				if r1 == r2 {
					return r1;
				}
				return random::Rng::gen_range(&mut rng.rng, <$ty>::min(r1, r2), <$ty>::max(r1, r2));
			}

		}

	}

}

macro_rules! impl_rand_range_vec {

	($ty:ty, $($mem:ident),*$(,)?) => {

		impl RandRange for $ty {

			fn rand_within(r1: Self, r2: Self) -> Self {
				return Self {
					$(
						$mem: rand(r1.$mem, r2.$mem),
					)*
				};
			}

			fn rand_within_from_rng(r1: Self, r2: Self, rng: &mut Rng) -> Self {
				return Self {
					$(
						$mem: rng.gen(r1.$mem, r2.$mem),
					)*
				};
			}

		}

	}

}

impl_rand_range!(f32);
impl_rand_range!(f64);
impl_rand_range!(i8);
impl_rand_range!(i16);
impl_rand_range!(i32);
impl_rand_range!(i64);
impl_rand_range!(u8);
impl_rand_range!(u16);
impl_rand_range!(u32);
impl_rand_range!(u64);
impl_rand_range!(usize);
impl_rand_range_vec!(Vec2, x, y);
impl_rand_range_vec!(Vec3, x, y, z);
impl_rand_range_vec!(Vec4, x, y, z, w);
impl_rand_range_vec!(Color, r, g, b, a);

impl_rand!(f32, 0.0, 1.0);
impl_rand!(f64, 0.0, 1.0);
impl_rand!(i8, 0, 2);
impl_rand!(i16, 0, 2);
impl_rand!(i32, 0, 2);
impl_rand!(i64, 0, 2);
impl_rand!(u8, 0, 2);
impl_rand!(u16, 0, 2);
impl_rand!(u32, 0, 2);
impl_rand!(u64, 0, 2);
impl_rand!(usize, 0, 2);
impl_rand!(Vec2, vec2!(-1), vec2!(1));
impl_rand!(Vec3, vec3!(-1), vec3!(1));
impl_rand!(Vec4, vec4!(-1), vec4!(1));
impl_rand!(Color, rgba!(0, 0, 0, 1), rgba!(1));

pub fn rand<R: RandRange>(r1: R, r2: R) -> R {
	return R::rand_within(r1, r2);
}

pub fn rand_t<R: RandRange>(r: (R, R)) -> R {
	return R::rand_within(r.0, r.1);
}

pub fn rand_from<'a, T>(t: &'a [T]) -> Option<&'a T> {
	return t.choose(&mut random::thread_rng())
}

pub fn rand_by_cell(p1: Vec2, p2: Vec2, c: usize, r: usize, p: f32, count: (usize, usize)) -> Vec<Vec2> {

	let mut pts = Vec::with_capacity(c * r);
	let gw = (p2.x - p1.x) / c as f32;
	let gh = (p2.y - p1.y) / r as f32;

	for i in 0..c {

		for j in 0..r {

			if rand(0.0, 1.0) < p {

				let pp1 = p1 + vec2!(i as f32 * gw, j as f32 * gh);
				let pp2 = pp1 + vec2!(gw, gh);

				for _ in 0..rand_t(count) {
					pts.push(rand(pp1, pp2));
				}

			}

		}

	}

	return pts;

}

#[derive(Clone)]
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

	pub fn gen<R: RandRange>(&mut self, r1: R, r2: R) -> R {
		return R::rand_within_from_rng(r1, r2, self);
	}

}

/// choose a random one from given arguments
#[macro_export]
macro_rules! choose {

	($c1:expr, $c2:expr) => {
		{
			let _rnd = rand(0.0, 1.0);
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
			let _rnd = rand(0.0, 1.0);
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
			let _rnd = rand(0.0, 1.0);
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

// use crate::math::*;

// const A: u128 = 1664525;
// const C: u128 = 1013904223;
// const M: u128 = u128::pow(2, 32);

// pub trait RandRange {
// 	fn rand_within(a: Self, b: Self, r: &mut Rng) -> Self;
// }

// impl RandRange for f32 {
// 	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
// 		return a + (b - a) * f.gen_f();
// 	}
// }

// impl RandRange for Vec2 {
// 	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
// 		return vec2!(
// 			a.x + (b.x - a.x) * f.gen_f(),
// 			a.y + (b.y - a.y) * f.gen_f(),
// 		);
// 	}
// }

// impl RandRange for Vec3 {
// 	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
// 		return vec3!(
// 			a.x + (b.x - a.x) * f.gen_f(),
// 			a.y + (b.y - a.y) * f.gen_f(),
// 			a.z + (b.z - a.z) * f.gen_f(),
// 		);
// 	}
// }

// impl RandRange for Color {
// 	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
// 		return rgba!(
// 			a.r + (b.r - a.r) * f.gen_f(),
// 			a.g + (b.g - a.g) * f.gen_f(),
// 			a.b + (b.b - a.b) * f.gen_f(),
// 			a.a + (b.a - a.a) * f.gen_f(),
// 		);
// 	}
// }

// pub struct Rng {
// 	seed: u128,
// }

// impl Rng {
// 	pub const fn new(s: u128) -> Self {
// 		return Self {
// 			seed: s,
// 		};
// 	}
// 	fn gen(&mut self) -> u128 {
// 		self.seed = (A * self.seed + C) % M;
// 		return self.seed;
// 	}
// 	pub fn gen_f(&mut self) -> f32 {
// 		return self.gen() as f32 / M as f32;
// 	}
// 	pub fn gen_range<R: RandRange>(&mut self, a: R, b: R) -> R {
// 		return R::rand_within(a, b, self);
// 	}
// }

