// wengwengweng

use super::*;

use random;
use random::seq::SliceRandom;
use random::Rng as RngTrait;
use random::SeedableRng;

pub type RandSeed = u64;

pub trait Rand {
	type Output;
	fn gen_rand(&self) -> Self::Output;
	fn gen_rand_with_seed(&self, rng: &mut Rng) -> Self::Output;
}

impl Rand for f32 {

	type Output = f32;

	fn gen_rand(&self) -> Self::Output {
		return rand((0.0, *self));
	}

	fn gen_rand_with_seed(&self, rng: &mut Rng) -> Self::Output {
		return rng.gen((0.0, *self));
	}

}

impl Rand for i32 {

	type Output = i32;

	fn gen_rand(&self) -> Self::Output {
		return rand((0, *self));
	}

	fn gen_rand_with_seed(&self, rng: &mut Rng) -> Self::Output {
		return rng.gen((0, *self));
	}

}

impl Rand for (i32, i32) {

	type Output = i32;

	fn gen_rand(&self) -> Self::Output {
		if self.0 == self.1 {
			return self.0;
		} else {
			return random::Rng::gen_range(&mut random::thread_rng(), i32::min(self.0, self.1), i32::max(self.0, self.1));
		}
	}

	fn gen_rand_with_seed(&self, rng: &mut Rng) -> Self::Output {
		if self.0 == self.1 {
			return self.0;
		} else {
			return random::Rng::gen_range(&mut rng.rng, i32::min(self.0, self.1), i32::max(self.0, self.1));
		}
	}

}

impl Rand for (f32, f32) {

	type Output = f32;

	fn gen_rand(&self) -> Self::Output {
		if self.0 == self.1 {
			return self.0;
		} else {
			return random::Rng::gen_range(&mut random::thread_rng(), f32::min(self.0, self.1), f32::max(self.0, self.1));
		}
	}

	fn gen_rand_with_seed(&self, rng: &mut Rng) -> Self::Output {
		if self.0 == self.1 {
			return self.0;
		} else {
			return random::Rng::gen_range(&mut rng.rng, f32::min(self.0, self.1), f32::max(self.0, self.1));
		}
	}

}

impl Rand for (Vec2, Vec2) {

	type Output = Vec2;

	fn gen_rand(&self) -> Self::Output {

		let x = rand((self.0.x, self.1.x));
		let y = rand((self.0.y, self.1.y));

		return vec2!(x, y);

	}

	fn gen_rand_with_seed(&self, rng: &mut Rng) -> Self::Output {

		let x = rng.gen((self.0.x, self.1.x));
		let y = rng.gen((self.0.y, self.1.y));

		return vec2!(x, y);

	}

}

impl Rand for (Vec3, Vec3) {

	type Output = Vec3;

	fn gen_rand(&self) -> Self::Output {

		let x = rand((self.0.x, self.1.x));
		let y = rand((self.0.y, self.1.y));
		let z = rand((self.0.z, self.1.z));

		return vec3!(x, y, z);

	}

	fn gen_rand_with_seed(&self, rng: &mut Rng) -> Self::Output {

		let x = rng.gen((self.0.x, self.1.x));
		let y = rng.gen((self.0.y, self.1.y));
		let z = rng.gen((self.0.z, self.1.z));

		return vec3!(x, y, z);

	}

}

impl<'a, T> Rand for &'a [T] {

	type Output = Option<&'a T>;

	fn gen_rand(&self) -> Self::Output {
		return self.choose(&mut random::thread_rng())
	}

	fn gen_rand_with_seed(&self, rng: &mut Rng) -> Self::Output {
		return self.choose(&mut rng.rng)
	}

}

pub fn rand<R: Rand>(r: R) -> R::Output {
	return r.gen_rand();
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

	pub fn gen<R: Rand>(&mut self, r: R) -> R::Output {
		return r.gen_rand_with_seed(self);
	}

}

