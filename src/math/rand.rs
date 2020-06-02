// wengwengweng

// https://en.wikipedia.org/wiki/Linear_congruential_generator

use std::time::SystemTime;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use super::*;

// param from Numerical Recipes
const A: u64 = 1664525;
const C: u64 = 1013904223;
const M: u64 = 4294967296;

pub trait RandRange {
	fn rand_within(a: Self, b: Self, r: &mut Rng) -> Self;
}

macro_rules! impl_rand {
	($ty:ty) => {
		impl RandRange for $ty {
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

impl RandRange for Vec2 {
	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
		return vec2!(
			a.x + (b.x - a.x) * f.gen_f(),
			a.y + (b.y - a.y) * f.gen_f(),
		);
	}
}

impl RandRange for Vec3 {
	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
		return vec3!(
			a.x + (b.x - a.x) * f.gen_f(),
			a.y + (b.y - a.y) * f.gen_f(),
			a.z + (b.z - a.z) * f.gen_f(),
		);
	}
}

impl RandRange for Color {
	fn rand_within(a: Self, b: Self, f: &mut Rng) -> Self {
		return rgba!(
			a.r + (b.r - a.r) * f.gen_f(),
			a.g + (b.g - a.g) * f.gen_f(),
			a.b + (b.b - a.b) * f.gen_f(),
			a.a + (b.a - a.a) * f.gen_f(),
		);
	}
}

pub struct Rng {
	seed: u64,
}

impl Rng {
	pub const fn new(s: u64) -> Self {
		return Self {
			seed: s,
		};
	}
	fn gen(&mut self) -> u64 {
		self.seed = (A * self.seed + C) % M;
		return self.seed;
	}
	pub fn gen_f(&mut self) -> f32 {
		return self.gen() as f32 / M as f32;
	}
	pub fn gen_range<R: RandRange>(&mut self, a: R, b: R) -> R {
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

pub fn rand<R: RandRange>(a: R, b: R) -> R {
	return R::rand_within(a, b, &mut DEFAULT_RNG.lock().expect("failed to lock rng mutex"));
}

pub fn rand_t<R: RandRange>(t: (R, R)) -> R {
	return R::rand_within(t.0, t.1, &mut DEFAULT_RNG.lock().expect("failed to lock rng mutex"));
}

pub fn rand_from<T>(list: &[T]) -> Option<&T> {
	return list.get(rand(0, list.len()));
}

