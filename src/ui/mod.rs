// wengwengweng

use crate::*;

const BAR_HEIGHT: u32 = 36;
const CORNER: f32 = 1.4;

pub fn window(title: &str, width: u32, height: u32) {

	gfx::push();

	gfx::color(color!(0, 0.35, 0.35, 1));
	gfx::rect(vec2!(width, height));

	gfx::color(color!(0, 0.51, 0.51, 1));
	gfx::rect(vec2!(width, BAR_HEIGHT));

	gfx::line_width(3);
	gfx::color(color!(0.02, 0.18, 0.18, 1));

	let pts = [
		vec2!(0.0 + CORNER, 0.0 - CORNER),
		vec2!(width as f32 - CORNER, 0.0 - CORNER),
		vec2!(width as f32 + CORNER, 0.0 + CORNER),
		vec2!(width as f32 + CORNER, height as f32 - CORNER),
		vec2!(width as f32 - CORNER, height as f32 + CORNER),
		vec2!(0.0 + CORNER, height as f32 + CORNER),
		vec2!(0.0 - CORNER, height as f32 - CORNER),
		vec2!(0.0 - CORNER, 0.0 + CORNER),
	];

	gfx::poly(&pts);

	gfx::line(vec2!(0, BAR_HEIGHT), vec2!(width, BAR_HEIGHT));

	gfx::push();
	gfx::color(color!(0.56, 0.76, 0.76, 1));
	gfx::translate(vec2!(12, 8));
	gfx::scale(vec2!(1.6));
	gfx::text(title);
	gfx::pop();

	gfx::pop();

}

