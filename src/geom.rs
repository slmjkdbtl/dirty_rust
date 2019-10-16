// wengwengweng

//! General Collision Detection

// some code borrowed from [p5.collide2dD](https://github.com/bmoren/p5.collide2D)

use crate::*;
use crate::math::*;

fn rect_rect(r1: (Vec2, Vec2), r2: (Vec2, Vec2)) -> bool {

	let (p1, p2) = r1;
	let (p3, p4) = r2;

	return p1.x <= p3.x && p2.x >= p4.x && p1.y <= p3.y && p2.y >= p4.y;
}

fn line_rect(line: (Vec2, Vec2), rect: (Vec2, Vec2)) -> bool {

	let (p1, p2) = line;
	let (p3, p4) = rect;

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

	let (p1, p2) = rect;
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

	return Vec2::dis(center, test) <= radius;

}

fn point_circle(pt: Vec2, circle: (Vec2, f32)) -> bool {
	let (center, radius) = circle;
	return Vec2::dis(pt, center) < radius;
}

// TODO
fn line_circle(line: (Vec2, Vec2), circle: (Vec2, f32)) -> bool {

	let (p1, p2) = line;

	if point_circle(p1, circle) || point_circle(p2, circle) {
		return true;
	}

	return false;

}

// TODO
fn point_line(pt: Vec2, line: (Vec2, Vec2)) -> bool {

	let (p1, p2) = line;

	// get distance from the point to the two ends of the line
	let d1 = Vec2::dis(pt, p1);
	let d2 = Vec2::dis(pt, p2);

	// get the length of the line
	let len = Vec2::dis(p1, p2);

	return false;

}

fn point_point(p1: Vec2, p2: Vec2) -> bool {
	return p1 == p2;
}

fn circle_circle(c1: (Vec2, f32), c2: (Vec2, f32)) -> bool {

	let (p1, r1) = c1;
	let (p2, r2) = c2;

	return r1 + r2 < Vec2::dis(p1, p2);

}

fn line_line(l1: (Vec2, Vec2), l2: (Vec2, Vec2)) -> bool {

	let (p1, p2) = l1;
	let (p3, p4) = l2;

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
	let (p1, p2) = rect;
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
/// perform sat collision detection on 2 polygons
pub fn sat(p1: &[Vec2], p2: &[Vec2]) -> (bool, Vec2) {

	assert!(p1.len() >= 3, "invalid polygon");
	assert!(p2.len() >= 3, "invalid polygon");

	let get_axis = |poly: &[Vec2]| {

		let mut normals = Vec::with_capacity(poly.len());
		let len = poly.len();

		for i in 0..len {

			let p1 = poly[i];
			let p2 = poly[(i + 1) % len];

			normals.push((p1 - p2).normal().normalize());

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

pub fn gjk(p1: &[Vec2], p2: &[Vec2]) -> (bool, Vec2) {
	unimplemented!();
}

#[derive(Clone, Debug)]
pub enum Shape2D<'a> {
	Point(Vec2),
	Circle(Vec2, f32),
	Rect(Vec2, Vec2),
	Line(Vec2, Vec2),
	Polygon(&'a[Vec2]),
	Ray(Vec2, f32),
}

impl<'a> From<&'a [Vec2]> for Shape2D<'a> {
	fn from(pts: &'a [Vec2]) -> Self {
		return Self::Polygon(pts);
	}
}

pub fn overlaps(s1: Shape2D, s2: Shape2D) -> bool {

	use Shape2D::*;

	match s1 {
		Circle(center, radius) => {
			match s2 {
				Circle(center2, radius2) => {
					return circle_circle((center, radius), (center2, radius2));
				},
				Rect(p1, p2) => {
					return rect_circle((p1, p2), (center, radius));
				},
				Point(pt) => {
					return point_circle(pt, (center, radius));
				},
				Line(p1, p2) => {
					return line_circle((p1, p2), (center, radius));
				},
				Polygon(verts) => {
					// TODO
				},
				Ray(pt, angle) => {
					// TODO
				},
			}
		},
		Rect(p1, p2) => {
			match s2 {
				Circle(..) => {
					return overlaps(s2, s1);
				},
				Rect(p12, p22) => {
					return rect_rect((p1, p2), (p12, p22));
				},
				Point(pt) => {
					return point_rect(pt, (p1, p2));
				},
				Line(p12, p22) => {
					return line_rect((p12, p22), (p1, p2));
				},
				Polygon(verts) => {
					return poly_poly(&verts, &[p1, vec2!(p2.x, p1.y), p2, vec2!(p1.x, p2.y)]);
				},
				Ray(pt, angle) => {
					// TODO
				},
			}
		},
		Point(pt) => {
			match s2 {
				Circle(..)
				| Rect(..) => {
					return overlaps(s2, s1);
				},
				Point(pt2) => {
					return point_point(pt, pt2);
				},
				Line(p1, p2) => {
					return point_line(pt, (p1, p2));
				},
				Polygon(verts) => {
					return point_poly(pt, &verts);
				},
				Ray(pt, angle) => {
					// TODO
				},
			}
		},
		Line(p1, p2) => {
			match s2 {
				Circle(..)
				| Rect(..)
				| Point(..) => {
					return overlaps(s2, s1);
				},
				Line(p12, p22) => {
					return line_line((p1, p2), (p12, p22));
				},
				Polygon(verts) => {
					return line_poly((p1, p2), &verts);
				},
				Ray(pt, angle) => {
					// TODO
				},
			}
		},
		Polygon(ref verts) => {
			match s2 {
				Circle(..)
				| Rect(..)
				| Point(..)
				| Line(..) => {
					return overlaps(s2, s1);
				},
				Polygon(verts2) => {
					return poly_poly(&verts, &verts2);
				},
				Ray(pt, angle) => {
					// TODO
				},
			}
		},
		Ray(pt, angle) => {
			match s2 {
				Circle(..)
				| Rect(..)
				| Point(..)
				| Line(..)
				| Polygon(..) => {
					return overlaps(s2, s1);
				},
				Ray(pt, angle) => {
					// TODO
				},
			}
		},
	}

	return false;

}

