// wengwengweng

//! General Collision Detection

// some code borrowed from [p5.collide2dD](https://github.com/bmoren/p5.collide2D) and [three.js](https://threejs.org)

use crate::*;
use math::*;

fn fix_pt_pair(pts: (Vec2, Vec2)) -> (Vec2, Vec2) {

	let (p1, p2) = pts;
	let pp1 = vec2!(f32::min(p1.x, p2.x), f32::min(p1.y, p2.y));
	let pp2 = vec2!(f32::max(p1.x, p2.x), f32::max(p1.y, p2.y));

	return (pp1, pp2);

}

fn rect_rect(r1: Rect, r2: Rect) -> bool {

	let (p1, p2) = fix_pt_pair((r1.min, r2.max));
	let (p3, p4) = fix_pt_pair((r2.min, r2.max));

	return p2.x >= p3.x && p1.x <= p4.x && p2.y >= p3.y && p1.y <= p4.y;

}

fn line_rect(line: Line2, rect: Rect) -> bool {

	let (p1, p2) = fix_pt_pair((line.p1, line.p2));
	let (p3, p4) = fix_pt_pair((rect.min, rect.max));

	if point_rect(p1, rect) || point_rect(p2, rect) {
		return true;
	}

	if
		line_line(Line2::new(p1, p2), Line2::new(p3, vec2!(p4.x, p3.y)))
		|| line_line(Line2::new(p1, p2), Line2::new(vec2!(p4.x, p3.y), p4))
		|| line_line(Line2::new(p1, p2), Line2::new(p4, vec2!(p3.x, p4.y)))
		|| line_line(Line2::new(p1, p2), Line2::new(vec2!(p3.x, p4.y), p3)) {
		return true;
	}

	return false;

}

fn rect_circle(r: Rect, c: Circle) -> bool {

	let (p1, p2) = fix_pt_pair((r.min, r.max));
	let mut test = c.center;

	if (c.center.x < p1.x) {
		test.x = p1.x;
	} else if (c.center.x > p2.x) {
		test.x = p2.x;
	}

	if (c.center.y < p1.y){
		test.y = p1.y;
	} else if (c.center.y > p2.y) {
		test.y = p2.y;
	}

	return Vec2::dist(c.center, test) <= c.radius;

}

// TODO
fn poly_circle(poly: &[Vec2], c: Circle) -> bool {
	return false;
}

fn point_circle(pt: Vec2, c: Circle) -> bool {
	return Vec2::dist(pt, c.center) < c.radius;
}

fn line_circle(line: Line2, c: Circle) -> bool {

	let (p1, p2) = fix_pt_pair((line.p1, line.p2));

	if point_circle(p1, c) || point_circle(p2, c) {
		return true;
	}

	return false;

}

// TODO
fn point_line(pt: Vec2, line: Line2) -> bool {

	let (p1, p2) = fix_pt_pair((line.p1, line.p2));

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

fn circle_circle(c1: Circle, c2: Circle) -> bool {
	return c1.radius + c2.radius < Vec2::dist(c1.center, c2.center);
}

fn line_line(l1: Line2, l2: Line2) -> bool {

	let (p1, p2) = fix_pt_pair((l1.p1, l1.p2));
	let (p3, p4) = fix_pt_pair((l2.p1, l2.p2));

	let a = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));
	let b = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));

	return a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0;

}

fn line_poly(line: Line2, poly: &[Vec2]) -> bool {

	let len = poly.len();

	assert!(len >= 3, "invalid polygon");

	for i in 0..len {

		let p3 = poly[i];
		let p4 = poly[(i + 1) % len];

		if line_line(line, Line2::new(p3, p4)) {
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

		if line_poly(Line2::new(p1, p2), poly2) {
			return true;
		}

	}

	return false;

}

fn point_rect(pt: Vec2, rect: Rect) -> bool {
	let (p1, p2) = fix_pt_pair((rect.min, rect.max));
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

fn poly_rect(p: &[Vec2], r: Rect) -> bool {
	let (p1, p2) = fix_pt_pair((r.min, r.max));
	return poly_poly(&p, &[p1, vec2!(p2.x, p1.y), p2, vec2!(p1.x, p2.y)])
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
	Circle(Circle),
	Rect(Rect),
	Line(Line2),
	Polygon(&'a[Vec2]),
}

macro_rules! from_s2d {
	($from:ty => $to:tt) => {
		impl<'a> From<$from> for Shape2D<'a> {
			fn from(p: $from) -> Shape2D<'a> {
				return Shape2D::$to(p);
			}
		}
	}
}

from_s2d!(Vec2 => Point);
from_s2d!(Rect => Rect);
from_s2d!(Circle => Circle);
from_s2d!(Line2 => Line);
from_s2d!(&'a [Vec2] => Polygon);

pub fn intersect2d<'a>(s1: impl Into<Shape2D<'a>>, s2: impl Into<Shape2D<'a>>) -> bool {

	use Shape2D::*;

	let s1 = s1.into();
	let s2 = s2.into();

	return match s1 {
		Circle(c) => {
			match s2 {
				Circle(c2) => circle_circle(c, c2),
				Rect(r) => rect_circle(r, c),
				Point(pt) => point_circle(pt, c),
				Line(l) => line_circle(l, c),
				Polygon(verts) => poly_circle(verts, c),
			}
		},
		Rect(r) => {
			match s2 {
				Circle(..) => intersect2d(s2, s1),
				Rect(r2) => rect_rect(r, r2),
				Point(pt) => point_rect(pt, r),
				Line(l) => line_rect(l, r),
				Polygon(verts) => poly_rect(verts, r),
			}
		},
		Point(pt) => {
			match s2 {
				Circle(..)
				| Rect(..) => intersect2d(s2, s1),
				Point(pt2) => point_point(pt, pt2),
				Line(l) => point_line(pt, l),
				Polygon(verts) => point_poly(pt, &verts),
			}
		},
		Line(l) => {
			match s2 {
				Circle(..)
				| Rect(..)
				| Point(..) => intersect2d(s2, s1),
				Line(l2) => line_line(l, l2),
				Polygon(verts) => line_poly(l, &verts),
			}
		},
		Polygon(ref verts) => {
			match s2 {
				Circle(..)
				| Rect(..)
				| Point(..)
				| Line(..) => intersect2d(s2, s1),
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

fn box_plane(b: BBox, p: Plane) -> bool {

	let mut min;
	let mut max;

	if ( p.normal.x > 0.0 ) {

		min = p.normal.x * b.min.x;
		max = p.normal.x * b.max.x;

	} else {

		min = p.normal.x * b.max.x;
		max = p.normal.x * b.min.x;

	}

	if ( p.normal.y > 0.0 ) {

		min += p.normal.y * b.min.y;
		max += p.normal.y * b.max.y;

	} else {

		min += p.normal.y * b.max.y;
		max += p.normal.y * b.min.y;

	}

	if ( p.normal.z > 0.0 ) {

		min += p.normal.z * b.min.z;
		max += p.normal.z * b.max.z;

	} else {

		min += p.normal.z * b.max.z;
		max += p.normal.z * b.min.z;

	}

	return ( min <= - p.constant && max >= - p.constant );

}

pub fn sphere_plane(s: Sphere, p: Plane) -> bool {
	return f32::abs( p.normal.dot( s.center ) + p.constant ) <= s.radius;
}

fn ray_plane(r: Ray3, p: Plane) -> bool {

	let denominator = p.normal.dot( r.dir );

	if ( denominator == 0.0 ) {

		// line is coplanar, return origin
		if ( p.normal.dot( r.origin ) + p.constant == 0.0 ) {
// 			return 0;
			return true;
		}

		// Null is preferable to undefined since undefined means.... it is undefined

		return false;

	}

	let t = - ( r.origin.dot( p.normal ) + p.constant ) / denominator;

	// Return if the ray never intersects the plane

	return t >= 0.0;
// 	return t >= 0 ? t : null;

}

pub fn ray_sphere(r: Ray3, s: Sphere) -> bool {

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
// 		return r.at( t1, target );
	}

	return true;
// 	return r.at( t0, target );

}

pub fn ray_box(r: Ray3, b: BBox) -> bool {

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
// 	return r.at( tmin >= 0 ? tmin : tmax, target );

}

// TODO
fn ray_pt(r: Ray3, pt: Vec3) -> bool {
	return false;
}

// TODO
fn ray_ray(r1: Ray3, r2: Ray3) -> bool {
	return false;
}

#[derive(Clone, Copy, Debug)]
pub enum Shape3D {
	Point(Vec3),
	Box(BBox),
	Sphere(Sphere),
	Ray(Ray3),
	Plane(Plane),
	Line(Line3),
}

macro_rules! from_s3d {
	($from:ty => $to:tt) => {
		impl From<$from> for Shape3D {
			fn from(p: $from) -> Shape3D {
				return Shape3D::$to(p);
			}
		}
	}
}

from_s3d!(Vec3 => Point);
from_s3d!(BBox => Box);
from_s3d!(Sphere => Sphere);
from_s3d!(Ray3 => Ray);
from_s3d!(Plane => Plane);
from_s3d!(Line3 => Line);

pub fn intersect3d(s1: impl Into<Shape3D>, s2: impl Into<Shape3D>) -> bool {

	use Shape3D::*;

	let s1 = s1.into();
	let s2 = s2.into();

	return match s1 {
		Box(b) => {
			match s2 {
				Box(b2) => box_box(b, b2),
				Point(pt) => pt_box(pt, b),
				Sphere(s) => sphere_box(s, b),
				Ray(r) => ray_box(r, b),
				Plane(p) => box_plane(b, p),
				Line(l) => todo!(),
			}
		},
		Point(pt) => {
			match s2 {
				Box(..) => intersect3d(s2, s1),
				Point(p2) => pt == p2,
				Sphere(s) => sphere_pt(s, pt),
				Ray(r) => ray_pt(r, pt),
				Plane(p) => todo!(),
				Line(l) => todo!(),
			}
		},
		Sphere(s) => {
			match s2 {
				Box(..)
				| Point(..) => intersect3d(s2, s1),
				Sphere(s2) => sphere_sphere(s, s2),
				Ray(r) => ray_sphere(r, s),
				Plane(p) => sphere_plane(s, p),
				Line(l) => todo!(),
			}
		},
		Ray(r) => {
			match s2 {
				Box(..)
				| Point(..)
				| Sphere(..) => intersect3d(s2, s1),
				Ray(r2) => ray_ray(r, r2),
				Plane(p) => ray_plane(r, p),
				Line(l) => todo!(),
			}
		},
		Plane(p) => {
			match s2 {
				Box(..)
				| Point(..)
				| Sphere(..)
				| Ray(..) => intersect3d(s2, s1),
				Plane(p) => todo!(),
				Line(l) => todo!(),
			}
		},
		Line(l) => {
			match s2 {
				Box(..)
				| Point(..)
				| Sphere(..)
				| Ray(..)
				| Plane(..) => intersect3d(s2, s1),
				Line(l2) => todo!(),
			}
		},
	}

}

