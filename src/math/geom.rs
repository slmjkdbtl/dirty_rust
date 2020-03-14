// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray3 {
	pub origin: Vec3,
	pub dir: Vec3,
}

impl Ray3 {

	pub fn new(origin: Vec3, dir: Vec3) -> Self {
		return Self {
			origin: origin,
			dir: dir,
		};
	}

	pub fn at(&self, d: f32) -> Vec3 {
		return self.origin + self.dir * d;
	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray2 {
	pub origin: Vec2,
	pub dir: Vec2,
}

impl Ray2 {

	pub fn new(origin: Vec2, dir: Vec2) -> Self {
		return Self {
			origin: origin,
			dir: dir,
		};
	}

	pub fn at(&self, d: f32) -> Vec2 {
		return self.origin + self.dir * d;
	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line2 {
	pub p1: Vec2,
	pub p2: Vec2,
}

impl Line2 {
	pub fn new(p1: Vec2, p2: Vec2) -> Self {
		return Self {
			p1: p1,
			p2: p2,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line3 {
	pub p1: Vec3,
	pub p2: Vec3,
}

impl Line3 {
	pub fn new(p1: Vec3, p2: Vec3) -> Self {
		return Self {
			p1: p1,
			p2: p2,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
	pub min: Vec2,
	pub max: Vec2,
}

impl Rect {

	pub fn new(min: Vec2, max: Vec2) -> Self {
		return Self {
			min: min,
			max: max,
		};
	}

	pub fn center(&self) -> Vec2 {
		return (self.min + self.max) * 0.5;
	}

	pub fn width(&self) -> f32 {
		return self.max.x - self.min.x;
	}

	pub fn height(&self) -> f32 {
		return self.max.y - self.min.y;
	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
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

	pub fn max(self, other: Self) -> Self {

		let minx = f32::min(self.min.x, other.min.x);
		let miny = f32::min(self.min.y, other.min.y);
		let minz = f32::min(self.min.z, other.min.z);
		let maxx = f32::max(self.max.x, other.max.x);
		let maxy = f32::max(self.max.y, other.max.y);
		let maxz = f32::max(self.max.z, other.max.z);

		return Self {
			min: vec3!(minx, miny, minz),
			max: vec3!(maxx, maxy, maxz),
		};

	}

	pub fn min(self, other: Self) -> Self {

		let minx = f32::max(self.min.x, other.min.x);
		let miny = f32::max(self.min.y, other.min.y);
		let minz = f32::max(self.min.z, other.min.z);
		let maxx = f32::min(self.max.x, other.max.x);
		let maxy = f32::min(self.max.y, other.max.y);
		let maxz = f32::min(self.max.z, other.max.z);

		return Self {
			min: vec3!(minx, miny, minz),
			max: vec3!(maxx, maxy, maxz),
		};

	}

	pub fn center(self) -> Vec3 {
		return (self.min + self.max) * 0.5;
	}

	pub fn transform(&self, t: Mat4) -> Self {

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
	pub normal: Vec3,
	pub dist: f32,
}

impl Plane {

	pub fn new(normal: Vec3, dist: f32) -> Self {
		return Self {
			normal: normal,
			dist: dist,
		};
	}
	pub fn from_pts(p0: Vec3, p1: Vec3, p2: Vec3) -> Self {

		let normal = Vec3::cross(p1 - p0, p1 - p2).unit();
		let d = -p1.dot(normal);

		return Self::new(normal, d);

	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
	pub center: Vec2,
	pub radius: f32,
}

impl Circle {

	pub fn new(center: Vec2, radius: f32) -> Self {
		return Self {
			center: center,
			radius: radius,
		};
	}

	pub fn rect(&self) -> Rect {

		let min = self.center - vec2!(self.radius);
		let max = self.center + vec2!(self.radius);

		return Rect::new(min, max);

	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
}

impl Sphere {

	pub fn new(center: Vec3, radius: f32) -> Self {
		return Self {
			center: center,
			radius: radius,
		};
	}

	pub fn bbox(&self) -> BBox {

		let min = self.center - vec3!(self.radius);
		let max = self.center + vec3!(self.radius);

		return BBox::new(min, max);

	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
	X,
	Y,
	Z,
}

impl Axis {
	pub fn as_vec3(&self) -> Vec3 {
		return match self {
			Axis::X => vec3!(1, 0, 0),
			Axis::Y => vec3!(0, 1, 0),
			Axis::Z => vec3!(0, 0, 1),
		};
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
	Right,
	Down,
	Left,
	Up,
}

impl Dir {
	pub fn as_vec2(&self) -> Vec2 {
		return match self {
			Dir::Right => vec2!(1, 0),
			Dir::Down => vec2!(0, 1),
			Dir::Left => vec2!(-1, 0),
			Dir::Up => vec2!(0, -1),
		};
	}
}

