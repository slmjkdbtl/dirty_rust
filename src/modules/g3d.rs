// wengwengweng

//! 3D Rendering

use crate::*;
use crate::math::*;
use crate::backends::gl;

const MAX_STATE_STACK: usize = 64;

include!("../res/resources.rs");

// context
ctx!(G3D: G3dCtx);

struct G3dCtx {

	projection: Mat4,
	state: State,
	state_stack: Vec<State>,
	vbuf: gl::VertexBuffer,
	ibuf: gl::IndexBuffer,
	program: gl::Program,
	empty_tex: gfx::Texture,

}

pub(super) fn init() {

	let (width, height) = window::size();
	let projection = math::perspective(45f32.to_radians(), width as f32 / height as f32, 0.1, 100.0);

	let verts = [
		// verts          // color
		-0.5, -0.5,  0.5, 1.0, 0.0, 0.0, 1.0,
		 0.5, -0.5,  0.5, 0.0, 1.0, 0.0, 1.0,
		 0.5,  0.5,  0.5, 0.0, 0.0, 1.0, 1.0,
		-0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0,
		-0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 1.0,
		 0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 1.0,
		 0.5,  0.5, -0.5, 0.0, 0.0, 1.0, 1.0,
		-0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 1.0,
	];

	let index = [
		// front
		0, 1, 2,
		2, 3, 0,
		// right
		1, 5, 6,
		6, 2, 1,
		// back
		7, 6, 5,
		5, 4, 7,
		// left
		4, 0, 3,
		3, 7, 4,
		// bottom
		4, 5, 1,
		1, 0, 4,
		// top
		3, 2, 6,
		6, 7, 3
	];

	let vbuf = gl::VertexBuffer::new(verts.len(), 7, gl::BufferUsage::Static);

	vbuf
		.data(&verts, 0);

	vbuf.attr(gl::VertexAttr::new(3, 3, 0));
	vbuf.attr(gl::VertexAttr::new(4, 4, 3));

	let ibuf = gl::IndexBuffer::new(index.len(), gl::BufferUsage::Static);

	ibuf
		.data(&index, 0);

	let program = gl::Program::new(TEMPLATE_3D_VERT, TEMPLATE_3D_FRAG);

	program
		.attr(3, "vert")
		.attr(4, "color")
		.link();

	ctx_init(G3dCtx {

		projection: projection,
		state_stack: Vec::with_capacity(MAX_STATE_STACK),
		state: State::default(),
		vbuf: vbuf,
		ibuf: ibuf,
		program: program,
		empty_tex: gfx::Texture::from_color(color!(1), 1, 1),

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
	ctx_get_mut().state = State::default();
}

pub(super) fn clear_stack() {
	ctx_get_mut().state_stack.clear();
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

/// global translate
pub fn translate(pos: Vec3) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.translate(pos);

}

/// global rotate
pub fn rotate(x: f32, y: f32, z: f32) {

	let state = &mut ctx_get_mut().state;

	if x != 0.0 {
		state.transform = state.transform.rotate(x, Dir::X);
	}

	if y != 0.0 {
		state.transform = state.transform.rotate(y, Dir::Y);
	}

	if z != 0.0 {
		state.transform = state.transform.rotate(z, Dir::Z);
	}

}

/// global scale
pub fn scale(s: Vec3) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.scale(s);

}

pub fn cube() {

	let gfx = ctx_get();
	let model = gfx.state.transform;
	let view = Mat4::identity();
	let (width, height) = window::size();
	let projection = gfx.projection;

	gfx.program.uniform_mat4("model", model.as_arr());
	gfx.program.uniform_mat4("view", view.as_arr());
	gfx.program.uniform_mat4("projection", projection.as_arr());

	gl::draw(
		&gfx.vbuf,
		&gfx.ibuf,
		&gfx.program,
		&gfx.empty_tex.handle,
		36,
	);

}

