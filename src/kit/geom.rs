// wengwengweng

//! General Collision Detection

// some code borrowed from [p5.collide2dD](https://github.com/bmoren/p5.collide2D)

use crate::*;
use math::*;

fn fix_pt_pair(rect: (Vec2, Vec2)) -> (Vec2, Vec2) {

	let (p1, p2) = rect;
	let pp1 = vec2!(f32::min(p1.x, p2.x), f32::min(p1.y, p2.y));
	let pp2 = vec2!(f32::max(p1.x, p2.x), f32::max(p1.y, p2.y));

	return (pp1, pp2);

}

fn rect_rect(r1: (Vec2, Vec2), r2: (Vec2, Vec2)) -> bool {

	let (p1, p2) = fix_pt_pair(r1);
	let (p3, p4) = fix_pt_pair(r2);

	return p2.x >= p3.x && p1.x <= p4.x && p2.y >= p3.y && p1.y <= p4.y;

}

fn line_rect(line: (Vec2, Vec2), rect: (Vec2, Vec2)) -> bool {

	let (p1, p2) = fix_pt_pair(line);
	let (p3, p4) = fix_pt_pair(rect);

	if point_rect(p1, rect) || point_rect(p2, rect) {
		return true;
	}

	if
		line_line((p1, p2), (p3, vec2!(p4.x, p3.y)))
		|| line_line((p1, p2), (vec2!(p4.x, p3.y), p4))
		|| line_line((p1, p2), (p4, vec2!(p3.x, p4.y)))
		|| line_line((p1, p2), (vec2!(p3.x, p4.y), p3)) {
		return true;
	}

	return false;

}

fn rect_circle(rect: (Vec2, Vec2), circle: (Vec2, f32)) -> bool {

	let (p1, p2) = fix_pt_pair(rect);
	let (center, radius) = circle;
	let mut test = center;

	if (center.x < p1.x) {
		test.x = p1.x;
	} else if (center.x > p2.x) {
		test.x = p2.x;
	}

	if (center.y < p1.y){
		test.y = p1.y;
	} else if (center.y > p2.y) {
		test.y = p2.y;
	}

	return Vec2::dist(center, test) <= radius;

}

// TODO
fn poly_circle(poly: &[Vec2], circle: (Vec2, f32)) -> bool {
	return false;
}

fn point_circle(pt: Vec2, circle: (Vec2, f32)) -> bool {
	let (center, radius) = circle;
	return Vec2::dist(pt, center) < radius;
}

fn line_circle(line: (Vec2, Vec2), circle: (Vec2, f32)) -> bool {

	let (p1, p2) = fix_pt_pair(line);

	if point_circle(p1, circle) || point_circle(p2, circle) {
		return true;
	}

	return false;

}

fn point_line(pt: Vec2, line: (Vec2, Vec2)) -> bool {

	let (p1, p2) = fix_pt_pair(line);

	// get distance from the point to the two ends of the line
	let d1 = Vec2::dist(pt, p1);
	let d2 = Vec2::dist(pt, p2);

	// get the length of the line
	let len = Vec2::dist(p1, p2);

	return false;

}

fn point_point(p1: Vec2, p2: Vec2) -> bool {
	return p1 == p2;
}

fn circle_circle(c1: (Vec2, f32), c2: (Vec2, f32)) -> bool {

	let (p1, r1) = c1;
	let (p2, r2) = c2;

	return r1 + r2 < Vec2::dist(p1, p2);

}

fn line_line(l1: (Vec2, Vec2), l2: (Vec2, Vec2)) -> bool {

	let (p1, p2) = fix_pt_pair(l1);
	let (p3, p4) = fix_pt_pair(l2);

	let a = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));
	let b = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));

	return a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0;

}

fn line_poly(line: (Vec2, Vec2), poly: &[Vec2]) -> bool {

	let len = poly.len();

	assert!(len >= 3, "invalid polygon");

	for i in 0..len {

		let p3 = poly[i];
		let p4 = poly[(i + 1) % len];

		if line_line(line, (p3, p4)) {
			return true;
		}

	}

	return false;

}

fn poly_poly(poly1: &[Vec2], poly2: &[Vec2]) -> bool {

	let len1 = poly1.len();
	let len2 = poly2.len();

	assert!(len1 >= 3, "invalid polygon");
	assert!(len2 >= 3, "invalid polygon");

	for i in 0..len1 {

		let p1 = poly1[i];
		let p2 = poly1[(i + 1) % len1];

		if line_poly((p1, p2), poly2) {
			return true;
		}

	}

	return false;

}

fn point_rect(pt: Vec2, rect: (Vec2, Vec2)) -> bool {
	let (p1, p2) = fix_pt_pair(rect);
	return pt.x >= p1.x && pt.x <= p2.x && pt.y >= p1.y && pt.y <= p2.y;
}

// TODO
fn point_poly(pt: Vec2, poly: &[Vec2]) -> bool {

	let len = poly.len();
	let mut has = false;

	assert!(len >= 3, "invalid polygon");

	for i in 0..len {

		let p1 = poly[i];
		let p2 = poly[(i + 1) % len];

		if ((p1.y > pt.y && p2.y < pt.y) || (p1.y < pt.y && p2.y > pt.y)) && (pt.x < (p2.x - p1.x) * (pt.y - p1.y) / (p2.y - p1.y) + p1.x) {
			has = !has;
		}

	}

	return has;

}

// TODO: clean
pub fn sat2d(p1: &[Vec2], p2: &[Vec2]) -> (bool, Vec2) {

	assert!(p1.len() >= 3, "invalid polygon");
	assert!(p2.len() >= 3, "invalid polygon");

	let get_axis = |poly: &[Vec2]| {

		let mut normals = Vec::with_capacity(poly.len());
		let len = poly.len();

		for i in 0..len {

			let p1 = poly[i];
			let p2 = poly[(i + 1) % len];

			normals.push((p1 - p2).normal().normalized());

		}

		return normals;

	};

	let project = |poly: &[Vec2], axis: Vec2| {

		let mut min = axis.dot(poly[1]);
		let mut max = min;

		for p in poly {

			let proj = axis.dot(*p);

			if proj < min {
				min = proj;
			} else if proj > max {
				max = proj;
			}

		}

		return (min, max);

	};

	let mut axis = Vec::with_capacity(p1.len() + p2.len());

	axis.extend(get_axis(p1));
	axis.extend(get_axis(p2));

	let mut mtv = vec2!(0);
	let mut overlap = 99999999f32;

	for a in axis {

		let (s1min, s1max) = project(p1, a);
		let (s2min, s2max) = project(p2, a);

		if s1min > s2max || s2min > s1max {
			return (false, vec2!());
		}

		let o = s2max - s1min;

		if o < overlap {

			overlap = o;
			mtv = a * o;

		}

	}

	return (true, mtv);

}

#[derive(Clone, Copy, Debug)]
pub enum Shape2D<'a> {
	Point(Vec2),
	Circle(Vec2, f32),
	Rect(Vec2, Vec2),
	Line(Vec2, Vec2),
	Polygon(&'a[Vec2]),
}

impl<'a> From<&'a [Vec2]> for Shape2D<'a> {
	fn from(pts: &'a [Vec2]) -> Self {
		return Self::Polygon(pts);
	}
}

pub fn overlap2d(s1: Shape2D, s2: Shape2D) -> bool {

	use Shape2D::*;

	return match s1 {
		Circle(center, radius) => {
			match s2 {
				Circle(center2, radius2) => circle_circle((center, radius), (center2, radius2)),
				Rect(p1, p2) => rect_circle((p1, p2), (center, radius)),
				Point(pt) => point_circle(pt, (center, radius)),
				Line(p1, p2) => line_circle((p1, p2), (center, radius)),
				Polygon(verts) => poly_circle(verts, (center, radius)),
			}
		},
		Rect(p1, p2) => {
			match s2 {
				Circle(..) => overlap2d(s2, s1),
				Rect(p12, p22) => rect_rect((p1, p2), (p12, p22)),
				Point(pt) => point_rect(pt, (p1, p2)),
				Line(p12, p22) => line_rect((p12, p22), (p1, p2)),
				Polygon(verts) => poly_poly(&verts, &[p1, vec2!(p2.x, p1.y), p2, vec2!(p1.x, p2.y)]),
			}
		},
		Point(pt) => {
			match s2 {
				Circle(..)
				| Rect(..) => overlap2d(s2, s1),
				Point(pt2) => point_point(pt, pt2),
				Line(p1, p2) => point_line(pt, (p1, p2)),
				Polygon(verts) => point_poly(pt, &verts),
			}
		},
		Line(p1, p2) => {
			match s2 {
				Circle(..)
				| Rect(..)
				| Point(..) => overlap2d(s2, s1),
				Line(p12, p22) => line_line((p1, p2), (p12, p22)),
				Polygon(verts) => line_poly((p1, p2), &verts),
			}
		},
		Polygon(ref verts) => {
			match s2 {
				Circle(..)
				| Rect(..)
				| Point(..)
				| Line(..) => overlap2d(s2, s1),
				Polygon(verts2) => poly_poly(&verts, &verts2),
			}
		},
	}

}

fn box_box(b1: BBox, b2: BBox) -> bool {
	return
		(b1.min.x <= b2.max.x && b1.max.x >= b2.min.x) &&
		(b1.min.y <= b2.max.y && b1.max.y >= b2.min.y) &&
		(b1.min.z <= b2.max.z && b1.max.z >= b2.min.z);
}

fn pt_box(pt: Vec3, b: BBox) -> bool {
	return
		(pt.x >= b.min.x && pt.x <= b.max.x) &&
		(pt.y >= b.min.y && pt.y <= b.max.y) &&
		(pt.z >= b.min.z && pt.z <= b.max.z);
}

fn sphere_pt(s: Sphere, pt: Vec3) -> bool {
	return Vec3::dist(pt, s.center) < s.radius;
}

fn sphere_sphere(s1: Sphere, s2: Sphere) -> bool {
	return Vec3::dist(s1.center, s2.center) < (s1.radius + s2.radius);
}

fn sphere_box(s: Sphere, b: BBox) -> bool {

	let x = f32::max(b.min.x, f32::min(s.center.x, b.max.x));
	let y = f32::max(b.min.y, f32::min(s.center.y, b.max.y));
	let z = f32::max(b.min.z, f32::min(s.center.z, b.max.z));

	return Vec3::dist(vec3!(x, y, z), s.center) < s.radius;

}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
}

pub fn ray_sphere(r: Ray, s: Sphere) -> bool {

	let d = s.center - r.origin;
	let tca = d.dot(r.dir);
	let d2 = d.dot(d) - tca * tca;
	let radius2 = s.radius * s.radius;

	if (d2 > radius2) {
		return false;
	}

	let thc = f32::sqrt(radius2 - d2);

	let t0 = tca - thc;
	let t1 = tca + thc;

	if (t0 < 0.0 && t1 < 0.0)  {
		return false;
	}

	if (t0 < 0.0) {
		return true;
// 		return this.at( t1, target );
	}

	return true;
// 	return this.at( t0, target );

}

pub fn ray_box(r: Ray, b: BBox) -> bool {

	let mut tmin;
	let mut tmax;
	let tymin;
	let tymax;
	let tzmin;
	let tzmax;

	let invdirx = 1.0 / r.dir.x;
	let invdiry = 1.0 / r.dir.y;
	let invdirz = 1.0 / r.dir.z;

	if ( invdirx >= 0.0 ) {

		tmin = ( b.min.x - r.origin.x ) * invdirx;
		tmax = ( b.max.x - r.origin.x ) * invdirx;

	} else {

		tmin = ( b.max.x - r.origin.x ) * invdirx;
		tmax = ( b.min.x - r.origin.x ) * invdirx;

	}

	if ( invdiry >= 0.0 ) {

		tymin = ( b.min.y - r.origin.y ) * invdiry;
		tymax = ( b.max.y - r.origin.y ) * invdiry;

	} else {

		tymin = ( b.max.y - r.origin.y ) * invdiry;
		tymax = ( b.min.y - r.origin.y ) * invdiry;

	}

	if ( ( tmin > tymax ) || ( tymin > tmax ) ) {
		return false;
	}

	if ( tymin > tmin || tmin != tmin ) {
		tmin = tymin;
	}

	if ( tymax < tmax || tmax != tmax ) {
		tmax = tymax;
	}

	if ( invdirz >= 0.0 ) {

		tzmin = ( b.min.z - r.origin.z ) * invdirz;
		tzmax = ( b.max.z - r.origin.z ) * invdirz;

	} else {

		tzmin = ( b.max.z - r.origin.z ) * invdirz;
		tzmax = ( b.min.z - r.origin.z ) * invdirz;

	}

	if ( ( tmin > tzmax ) || ( tzmin > tmax ) ) {
		return false;
	}

	if ( tzmin > tmin || tmin != tmin ) {
		tmin = tzmin;
	}

	if ( tzmax < tmax || tmax != tmax ) {
		tmax = tzmax;
	}

	if ( tmax < 0.0 ) {
		return false;
	}

	return true;
// 	return this.at( tmin >= 0 ? tmin : tmax, target );

}

fn ray_pt(r: Ray, pt: Vec3) -> bool {
	return false;
}

fn ray_ray(r1: Ray, r2: Ray) -> bool {
	return false;
}

#[derive(Clone, Copy, Debug)]
pub enum Shape3D {
	Point(Vec3),
	Box(BBox),
	Sphere(Sphere),
	Ray(Ray),
}

// TODO
pub fn overlap3d(s1: Shape3D, s2: Shape3D) -> bool {

	use Shape3D::*;

	return match s1 {
		Box(b) => {
			match s2 {
				Box(b2) => box_box(b, b2),
				Point(pt) => pt_box(pt, b),
				Sphere(s) => sphere_box(s, b),
				Ray(r) => ray_box(r, b),
			}
		},
		Point(pt) => {
			match s2 {
				Box(..) => overlap3d(s2, s1),
				Point(p2) => pt == p2,
				Sphere(s) => sphere_pt(s, pt),
				Ray(r) => ray_pt(r, pt),
			}
		},
		Sphere(s) => {
			match s2 {
				Box(..)
				| Point(..) => overlap3d(s2, s1),
				Sphere(s2) => sphere_sphere(s, s2),
				Ray(r) => ray_sphere(r, s),
			}
		},
		Ray(r) => {
			match s2 {
				Box(..)
				| Point(..)
				| Sphere(..) => overlap3d(s2, s1),
				Ray(r2) => ray_ray(r, r2),
			}
		}
	}

}

