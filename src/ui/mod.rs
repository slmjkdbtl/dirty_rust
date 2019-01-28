// wengwengweng

use crate::*;
use crate::math::*;

const BAR_HEIGHT: u32 = 40;
const CORNER: f32 = 1.4;

ctx!(UI: UICtx);

struct UICtx {
	windows: Vec<Window>,
}

/// initialize res module
pub fn init() {

	ctx_init(UICtx {
		windows: Vec::new(),
	});

}

enum WindowState {
	Idle,
	Dragged(Vec2),
	Active,
}

pub struct Window {

	title: String,
	pos: Vec2,
	width: u32,
	height: u32,
	draw: Box<FnMut()>,
	state: WindowState,

}

pub fn draw() {

	let ctx_mut = ctx_get_mut();

	gfx::push();
	gfx::reset();

	for w in &mut ctx_mut.windows {

		update_window(w);
		draw_window(w);

	}

	gfx::pop();

}

fn update_window(w: &mut Window) {

	let mpos = window::mouse_pos();

	if let WindowState::Dragged(pos) = w.state {
		w.pos = mpos - pos;
	}

}

fn draw_window(w: &Window) {

	gfx::push();
	gfx::translate(w.pos);

	gfx::color(color!(0, 0.35, 0.35, 1));
	gfx::rect(vec2!(w.width, w.height));

	gfx::color(color!(0, 0.51, 0.51, 1));
	gfx::rect(vec2!(w.width, BAR_HEIGHT));

	gfx::line_width(3);
	gfx::color(color!(0.02, 0.18, 0.18, 1));

	let pts = [
		vec2!(0.0 + CORNER, 0.0 - CORNER),
		vec2!(w.width as f32 - CORNER, 0.0 - CORNER),
		vec2!(w.width as f32 + CORNER, 0.0 + CORNER),
		vec2!(w.width as f32 + CORNER, w.height as f32 - CORNER),
		vec2!(w.width as f32 - CORNER, w.height as f32 + CORNER),
		vec2!(0.0 + CORNER, w.height as f32 + CORNER),
		vec2!(0.0 - CORNER, w.height as f32 - CORNER),
		vec2!(0.0 - CORNER, 0.0 + CORNER),
	];

	gfx::poly(&pts);

	gfx::line(vec2!(0, BAR_HEIGHT), vec2!(w.width, BAR_HEIGHT));

	gfx::push();

	match w.state {
		WindowState::Idle => gfx::color(color!(0.56, 0.76, 0.76, 1)),
		WindowState::Dragged(_) | WindowState::Active => gfx::color(color!(1)),
	}

	gfx::translate(vec2!(12, 5));
	gfx::scale(vec2!(2));
	gfx::text(&w.title);
	gfx::pop();

	gfx::pop();

}

pub fn window<F: FnMut()>(title: &str, pos: Vec2, width: u32, height: u32, f: F) {

	let ctx = ctx_get();
	let ctx_mut = ctx_get_mut();

	ctx_mut.windows.push(Window {

		title: String::from(title),
		pos: pos,
		width: width,
		height: height,
		draw: Box::new(|| {}),
		state: WindowState::Idle,

	});

}

