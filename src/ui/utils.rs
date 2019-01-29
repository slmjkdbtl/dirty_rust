// wengwengweng

use crate::*;
use crate::math::*;

pub fn rounded_rect(w: u32, h: u32, corner: f32) -> [Vec2; 8] {

	return [
		vec2!(0.0 + corner, 0.0 - corner),
		vec2!(w as f32 - corner, 0.0 - corner),
		vec2!(w as f32 + corner, 0.0 + corner),
		vec2!(w as f32 + corner, h as f32 - corner),
		vec2!(w as f32 - corner, h as f32 + corner),
		vec2!(0.0 + corner, h as f32 + corner),
		vec2!(0.0 - corner, h as f32 - corner),
		vec2!(0.0 - corner, 0.0 + corner),
	];

}

