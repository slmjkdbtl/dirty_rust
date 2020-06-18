// wengwengweng

// https://en.wikipedia.org/wiki/Linear_congruential_generator

use std::cell::RefCell;
use instant::Instant;
use super::*;

// ANSI C
// http://citeseer.ist.psu.edu/viewdoc/download?doi=10.1.1.53.3686&rep=rep1&type=pdf
const A: u64 = 1103515245;
const C: u64 = 12345;
const M: u64 = 2147483648;

thread_local! {
	static RNG: RefCell<Rng> = RefCell::new(Rng::new(hash!(Instant::now())));
}

pub trait RandValue {
	fn rand_within(a: Self, b: Self, r: &mut Rng) -> Self;
}

macro_rules! impl_rand {
	($ty:ty) => {
		impl RandValue for $ty {
			fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
				return (a as f32 + (b as f32 - a as f32) * f.gen()) as $ty;
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
impl_rand!(i128);
impl_rand!(isize);
impl_rand!(u8);
impl_rand!(u16);
impl_rand!(u32);
impl_rand!(u64);
impl_rand!(u128);
impl_rand!(usize);

impl RandValue for Vec2 {
	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
		return vec2!(
			a.x + (b.x - a.x) * f.gen(),
			a.y + (b.y - a.y) * f.gen(),
		);
	}
}

impl RandValue for Vec3 {
	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
		return vec3!(
			a.x + (b.x - a.x) * f.gen(),
			a.y + (b.y - a.y) * f.gen(),
			a.z + (b.z - a.z) * f.gen(),
		);
	}
}

impl RandValue for Color {
	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
		return rgba!(
			a.r + (b.r - a.r) * f.gen(),
			a.g + (b.g - a.g) * f.gen(),
			a.b + (b.b - a.b) * f.gen(),
			a.a + (b.a - a.a) * f.gen(),
		);
	}
}

/// A Simple Pseudorandom Number Generator
pub struct Rng {
	seed: u64,
}

impl Rng {

	/// create new from a seed
	pub const fn new(s: u64) -> Self {
		return Self {
			seed: s,
		};
	}

	/// generate a new random f32 between 0.0 and 1.0
	pub fn gen(&mut self) -> f32 {
		self.seed = (A.wrapping_mul(self.seed).wrapping_add(C)) % M;
		return self.seed as f32 / M as f32;
	}

	/// generate between 2 values that implements [`RandValue`](trait.RandValue.html)
	pub fn gen_between<R: RandValue>(&mut self, a: R, b: R) -> R {
		return R::rand_within(a, b, self);
	}

}

/// generate a random value with the default generator
pub fn rand<R: RandValue>(a: R, b: R) -> R {
	return RNG.with(|rng| {
		return R::rand_within(a, b, &mut *rng.borrow_mut());
	});
}

/// generate a random value within tuple range
pub fn rand_t<R: RandValue>(t: (R, R)) -> R {
	return rand(t.0, t.1);
}

/// rand value in an array
pub fn rand_from<T>(list: &[T]) -> Option<&T> {
	return list.get(rand(0, list.len()));
}

