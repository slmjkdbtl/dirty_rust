// wengwengweng

//! Math Related Types & Functions

mexport!(mat);
mexport!(rand);
export!(vec);
export!(lerp);
export!(map);
export!(noise);

pub fn wave(t: f32, low: f32, hi: f32) -> f32 {
	return (f32::sin(t) + 1.0) / 2.0 * (hi - low) + low;
}

