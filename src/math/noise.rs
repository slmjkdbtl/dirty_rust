// wengwengweng

use once_cell::sync::Lazy;
use super::*;

static DEFAULT_NOISE: Lazy<Noise> = Lazy::new(|| {
	return Noise::new(4096, 8);
});

#[derive(Clone)]
pub struct Noise {
	buf: Vec<f32>,
}

impl Noise {

	pub fn new(size: usize, octave: usize) -> Self {

		let seed = (0..size)
			.into_iter()
			.map(|_| {
				return rand(0.0, 1.0);
			})
			.collect::<Vec<f32>>();

		let mut buf = Vec::with_capacity(seed.len());

		for i in 0..seed.len() {

			let mut noise = 0.0;
			let mut scale_acc = 0.0;
			let mut scale = 1.0;

			for o in 0..octave {

				let pitch = seed.len() >> o;
				let n1 = (i / pitch) * pitch;
				let n2 = (n1 + pitch) % seed.len();
				let blend = (i - n1) as f32 / pitch as f32;
				let sample = (1.0 - blend) * seed[n1] + blend * seed[n2];

				noise += sample * scale;
				scale_acc += scale;
				scale = scale / 2.0;

			}

			buf.push(noise / scale_acc);

		}

		return Self {
			buf: buf,
		};

	}

	pub fn get(&self, x: f32) -> f32 {

		let x = x % self.buf.len() as f32;
		let a = f32::floor(x) as usize;
		let b = f32::ceil(x) as usize;
		let t = x % 1.0;

		return Lerp::cos_lerp(self.buf[a], self.buf[b], t);

	}

}

pub fn noise(x: f32) -> f32 {
	return DEFAULT_NOISE.get(x);
}

