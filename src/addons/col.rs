// wengwengweng

//! General Collision Detection

use crate::*;
use crate::utils::paired_iter::paired;

/// check collision between 2 rectangles
pub fn rect_rect(r1: Rect, r2: Rect) -> bool {
	return false;
}

/// check collision between 2 lines
pub fn line_line(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) -> bool {

	let a = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));
	let b = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x)) / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));

	return a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0;

}

/// check collision between a line and a polygon
pub fn line_poly(p1: Vec2, p2: Vec2, poly: &[Vec2]) -> bool {

	assert!(poly.len() >= 3, "invalid polygon");

	for (p3, p4) in paired(poly) {
		if line_line(p1, p2, *p3, *p4) {
			return true;
		}
	}

	return false;

}

/// check collision between 2 polygons
pub fn poly_poly(v1: &[Vec2], v2: &[Vec2]) -> bool {

	assert!(v1.len() >= 3, "invalid polygon");
	assert!(v2.len() >= 3, "invalid polygon");

	for (p1, p2) in paired(v1) {
		if line_poly(*p1, *p2, v2) {
			return true;
		}
	}

	return false;

}

/// check collision between a point and a polygon
pub fn point_poly(p: Vec2, poly: &[Vec2]) -> bool {

	assert!(poly.len() >= 3, "invalid polygon");

	let mut has = false;

	for (p1, p2) in paired(poly) {
		if ((p1.y > p.y && p2.y < p.y) || (p1.y < p.y && p2.y > p.y)) && (p.x < (p2.x - p1.x) * (p.y - p1.y) / (p2.y - p1.y) + p1.x) {
			has = !has;
		}
	}

	return has;

}

/// perform sat collision detection on 2 polygons
pub fn sat(p1: &[Vec2], p2: &[Vec2]) -> (bool, Vec2) {

	assert!(p1.len() >= 3, "invalid polygon");
	assert!(p2.len() >= 3, "invalid polygon");

	return (false, vec2!());

}

