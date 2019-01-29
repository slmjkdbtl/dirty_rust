// wengwengweng

//! 3D Rendering

use crate::*;
use crate::math::*;
use crate::backends::gl;

const MAX_STATE_STACK: usize = 64;

// context
ctx!(G3D: G3dCtx);

struct G3dCtx {

	projection: Mat4,
	state: State,
	state_stack: Vec<State>,

}

pub(super) fn init() {

	let (width, height) = window::size();
	let projection = Mat4::ortho(0.0, (width as f32), (height as f32), 0.0, -1.0, 1.0);

	ctx_init(G3dCtx {

		projection: projection,
		state_stack: Vec::with_capacity(MAX_STATE_STACK),
		state: State::default(),

	});

}

/// check if g3d is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

#[derive(Clone, Copy)]
struct State {
	transform: Mat4,
}

impl Default for State {
	fn default() -> Self {
		return Self {
			transform: Mat4::identity(),
		}
	}
}

/// reset global transforms
pub fn reset() {

	let g3d_mut = ctx_get_mut();

	g3d_mut.state_stack.clear();
	g3d_mut.state = State::default();

}

/// push state
pub fn push() {

	let g3d = ctx_get_mut();
	let stack = &mut g3d.state_stack;

	if (stack.len() < MAX_STATE_STACK) {
		stack.push(g3d.state);
	} else {
		panic!("cannot push anymore");
	}

}

/// pop state
pub fn pop() {

	let mut g3d = ctx_get_mut();
	let stack = &mut g3d.state_stack;

	g3d.state = stack.pop().expect("cannot pop anymore");

}

