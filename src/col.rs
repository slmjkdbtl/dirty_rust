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

/// check collision between 2 rectangles
pub fn rect_rect(r1: Quad, r2: Quad) -> bool {
	return r1.x <= r2.x && r1.x + r1.w >= r2.x && r1.y <= r2.y && r1.y + r1.h >= r2.y;
}

/// check collision between 2 lines
pub fn line_line(l1: (Vec2, Vec2), l2: (Vec2, Vec2)) -> bool {

	let (p1, p2) = l1;
	let (p3, p4) = l2;

	let a = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));
	let b = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));

	return a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0;

}

/// check collision between a line and a polygon
pub fn line_poly(line: (Vec2, Vec2), poly: &[Vec2]) -> bool {

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

/// check collision between 2 polygons
pub fn poly_poly(poly1: &[Vec2], poly2: &[Vec2]) -> bool {

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

/// check if a point is side a rect
pub fn point_rect(p: Vec2, rect: Quad) -> bool {
	return p.x >= rect.x && p.x <= rect.x + rect.w && p.y >= rect.y && p.y <= rect.y + rect.h;
}

/// check if a point is side a polygon
pub fn point_poly(p: Vec2, poly: &[Vec2]) -> bool {

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

