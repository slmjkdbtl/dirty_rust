// wengwengweng

// https://en.wikipedia.org/wiki/Linear_congruential_generator

use std::time::SystemTime;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use super::*;

// ANSI C
// https://en.wikipedia.org/wiki/Linear_congruential_generator#cite_note-16
const A: u64 = 1103515245;
const C: u64 = 12345;
const M: u64 = 2147483648;

pub trait RandValue {
	fn rand_within(a: Self, b: Self, r: &mut Rng) -> Self;
}

macro_rules! impl_rand {
	($ty:ty) => {
		impl RandValue for $ty {
			fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
				return (a as f32 + (b as f32 - a as f32) * f.gen_f()) as $ty;
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
			a.x + (b.x - a.x) * f.gen_f(),
			a.y + (b.y - a.y) * f.gen_f(),
		);
	}
}

impl RandValue for Vec3 {
	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
		return vec3!(
			a.x + (b.x - a.x) * f.gen_f(),
			a.y + (b.y - a.y) * f.gen_f(),
			a.z + (b.z - a.z) * f.gen_f(),
		);
	}
}

impl RandValue for Color {
	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
		return rgba!(
			a.r + (b.r - a.r) * f.gen_f(),
			a.g + (b.g - a.g) * f.gen_f(),
			a.b + (b.b - a.b) * f.gen_f(),
			a.a + (b.a - a.a) * f.gen_f(),
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
	fn gen(&mut self) -> u64 {
		self.seed = (A * self.seed + C) % M;
		return self.seed;
	}
	/// generate a new random f32 between 0.0 and 1.0
	pub fn gen_f(&mut self) -> f32 {
		return self.gen() as f32 / M as f32;
	}
	/// generate between 2 values that implements [`RandValue`](trait.RandValue.html)
	pub fn gen_between<R: RandValue>(&mut self, a: R, b: R) -> R {
		return R::rand_within(a, b, self);
	}
}

pub(crate) static DEFAULT_RNG: Lazy<Mutex<Rng>> = Lazy::new(|| {

	#[cfg(not(web))]
	let t = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.expect("failed to get system time")
		.as_secs();

	#[cfg(web)]
	let t = js_sys::Date::now() as u64;

	return Mutex::new(Rng::new(t));

});

/// generate a random value with the default generator
pub fn rand<R: RandValue>(a: R, b: R) -> R {
	return R::rand_within(a, b, &mut DEFAULT_RNG.lock().expect("failed to lock rng mutex"));
}

/// [`rand`](fn.rand.html) but for tuple
pub fn rand_t<R: RandValue>(t: (R, R)) -> R {
	return R::rand_within(t.0, t.1, &mut DEFAULT_RNG.lock().expect("failed to lock rng mutex"));
}

/// rand value in an array
pub fn rand_from<T>(list: &[T]) -> Option<&T> {
	return list.get(rand(0, list.len()));
}

