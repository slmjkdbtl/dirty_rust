// wengwengweng

//! Common Math Functions & Types

mexport!(mat);
mexport!(rand);
export!(vec);
export!(lerp);
export!(map);
export!(dir);

pub use noise;

pub fn wave(t: f32, low: f32, hi: f32) -> f32 {
	return (f32::sin(t) + 1.0) / 2.0 * (hi - low) + low;
}

#[derive(Clone, Copy, Debug)]
pub struct BBox {
	pub min: Vec3,
	pub max: Vec3,
}

impl BBox {

	pub fn new(min: Vec3, max: Vec3) -> Self {
		return Self {
			min: min,
			max: max,
		};
	}

	pub fn transform(self, t: Mat4) -> Self {

		let ax = self.min.x;
		let ay = self.min.y;
		let az = self.min.z;
		let bx = self.max.x;
		let by = self.max.y;
		let bz = self.max.z;

		let p1 = t * vec3!(ax, by, az);
		let p2 = t * vec3!(bx, by, az);
		let p3 = t * vec3!(bx, ay, az);
		let p4 = t * vec3!(ax, ay, az);
		let p5 = t * vec3!(ax, by, bz);
		let p6 = t * vec3!(bx, by, bz);
		let p7 = t * vec3!(bx, ay, bz);
		let p8 = t * vec3!(ax, ay, bz);

		return [p2, p3, p4, p5, p6, p7, p8].iter().fold(BBox::new(p1, p1), |bbox, p| {

			let minx = f32::min(bbox.min.x, p.x);
			let miny = f32::min(bbox.min.y, p.y);
			let minz = f32::min(bbox.min.z, p.z);
			let maxx = f32::max(bbox.max.x, p.x);
			let maxy = f32::max(bbox.max.y, p.y);
			let maxz = f32::max(bbox.max.z, p.z);

			return BBox {
				min: vec3!(minx, miny, minz),
				max: vec3!(maxx, maxy, maxz),
			};

		});

	}

}

