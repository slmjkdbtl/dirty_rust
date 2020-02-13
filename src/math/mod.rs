// wengwengweng

//! Common Math Functions & Structs

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

