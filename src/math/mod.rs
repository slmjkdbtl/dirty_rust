// wengwengweng

//! Common Math Functions

pub(crate) mod vec;
pub(crate) mod mat;
pub(crate) mod rand;

/// clamp a number within range
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
	if x < min {
		return min;
	} else if x > max {
		return max;
	} else {
		return x;
	}
}

/// linear interpolation
pub fn lerp(from: f32, to: f32, amount: f32) -> f32 {
	return from + (to - from) * clamp(amount, 0.0, 1.0);
}

/// cubic interpolation
pub fn smooth(from: f32, to: f32, amount: f32) -> f32 {

	let t = clamp(amount, 0.0, 1.0);
	let m = t * t * (3.0 - 2.0 * t);

	return from + (to - from) * m;

}

