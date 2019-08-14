// wengwengweng

//! General Collision Detection

use crate::*;
use crate::math::*;

fn pair<T, F: FnMut(&T, &T)>(list: &[T], mut f: F) {

	for i in 0..list.len() {

		let e1 = &list[i];

		if let Some(e2) = list.get(i + 1) {
			f(e1, e2);
		} else {
			f(e1, &list[0]);
		}

	}

}

// TODO: use (Vec2, Vec2) for rects
fn rect_rect(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) -> bool {
	return p1.x <= p3.x && p2.x >= p4.x && p1.y <= p3.y && p2.y >= p4.y;
}

// TODO
fn rect_circle(p1: Vec2, p2: Vec2, center: Vec2, radius: f32) -> bool {
	return false;
}

fn circle_circle(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> bool {
	return r1 + r2 < Vec2::dis(c1, c2);
}

fn line_line(l1: (Vec2, Vec2), l2: (Vec2, Vec2)) -> bool {

	let (p1, p2) = l1;
	let (p3, p4) = l2;

	let a = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));
	let b = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));

	return a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0;

}

fn line_poly(line: (Vec2, Vec2), poly: &[Vec2]) -> bool {

	assert!(poly.len() >= 3, "invalid polygon");

	let mut collided = false;

	pair(poly, |p3, p4| {
		if line_line(line, (*p3, *p4)) {
			collided = true;
			return;
		}
	});

	return collided;

}

fn poly_poly(poly1: &[Vec2], poly2: &[Vec2]) -> bool {

	assert!(poly1.len() >= 3, "invalid polygon");
	assert!(poly2.len() >= 3, "invalid polygon");

	let mut collided = false;

	pair(poly1, |p1, p2| {
		if line_poly((*p1, *p2), poly2) {
			collided = true;
			return;
		}
	});

	return collided;

}

fn point_rect(p: Vec2, rect: Quad) -> bool {
	return p.x >= rect.x && p.x <= rect.x + rect.w && p.y >= rect.y && p.y <= rect.y + rect.h;
}

fn point_poly(p: Vec2, poly: &[Vec2]) -> bool {

	assert!(poly.len() >= 3, "invalid polygon");

	let mut has = false;

	pair(poly, |p1, p2| {

		if ((p1.y > p.y && p2.y < p.y) || (p1.y < p.y && p2.y > p.y)) && (p.x < (p2.x - p1.x) * (p.y - p1.y) / (p2.y - p1.y) + p1.x) {
			has = !has;
		}

	});

	return has;

}

/// perform sat collision detection on 2 polygons
pub fn sat(p1: &[Vec2], p2: &[Vec2]) -> (bool, Vec2) {

	assert!(p1.len() >= 3, "invalid polygon");
	assert!(p2.len() >= 3, "invalid polygon");

	let get_axis = |poly: &[Vec2]| {

		let mut normals = Vec::with_capacity(poly.len());

		pair(poly, |p1, p2| {
			normals.push((*p1 - *p2).normal().unit());
		});

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
pub enum Shape2D {
	Point(Vec2),
	Circle(Vec2, f32),
	Rect(Vec2, Vec2),
	Line(Vec2, Vec2),
	Polygon(Vec<Vec2>),
	Ray(Vec2, f32),
}

impl From<&[Vec2]> for Shape2D {
	fn from(pts: &[Vec2]) -> Shape2D {
		return Shape2D::Polygon(pts.to_owned());
	}
}

pub fn overlaps(s1: Shape2D, s2: Shape2D) -> bool {

	use Shape2D::*;

	match s1 {
		Circle(center, radius) => {
			match s2 {
				Circle(center2, radius2) => {
					return circle_circle(center, radius, center2, radius2);
				}
				Rect(p1, p2) => {
					return rect_circle(p1, p2, center, radius);
				}
				Point(pt) => {
					// ...
				}
				Line(p1, p2) => {
					// ...
				}
				Polygon(verts) => {
					// ...
				}
				Ray(pt, angle) => {
					// ...
				}
			}
		}
		_ => {},
	}

	return false;

}

