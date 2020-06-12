// wengwengweng

use std::f32::consts::PI;
use super::*;

const MIN: f32 = 0.3;
const MAX: f32 = 1.0;

pub fn spatial_pan(src: Vec3, ear: Vec3, dir: Vec3, strength: f32) -> Pan {

	let dist = Vec3::dist(src, ear);
	let volume = dist.map(0.0, strength, 1.0, 0.0);

	let a = dir.xz().unit();
	let b = (src - ear).xz().unit();

	// TODO: better logic?
	let angle = (a.y.atan2(a.x) - b.y.atan2(b.x) + 3.0 * PI) % (2.0 * PI) - PI;

	let pan = if angle > 0.0 {
		Pan::new(1.0, (angle - PI / 2.0).abs().map(0.0, PI / 2.0, MIN, MAX))
	} else {
		Pan::new((-angle - PI / 2.0).abs().map(0.0, PI / 2.0, MIN, MAX), 1.0)
	};

	return pan * volume;

}

