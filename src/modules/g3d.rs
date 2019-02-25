// wengwengweng

//! 3D Rendering

use ggl_derive::Vertex;

use crate::*;
use crate::math::*;
use crate::gfx::*;
use crate::ggl;

const MAX_STATE_STACK: usize = 64;

include!("../res/resources.rs");

// context
ctx!(G3D: G3dCtx);

struct G3dCtx {

	projection: Mat4,
	cam: Cam,
	state: State,
	state_stack: Vec<State>,
	default_shader: Shader,
	current_shader: Shader,
	empty_tex: Texture,
	cube_mesh: ggl::Mesh,

}

pub(super) fn init() {

	let (width, height) = window::size();
	let projection = math::perspective(45f32.to_radians(), width as f32 / height as f32, 0.1, 100.0);

	let default_shader = Shader::from_code(DEFAULT_3D_VERT, DEFAULT_3D_FRAG);

	let cube_verts = [

		Vertex::new(vec3!(-0.5, -0.5, 0.5), vec2!(), color!(1, 0, 0, 1)),
		Vertex::new(vec3!(0.5, -0.5, 0.5), vec2!(), color!(0, 1, 0, 1)),
		Vertex::new(vec3!(0.5, 0.5, 0.5), vec2!(), color!(0, 0, 1, 1)),
		Vertex::new(vec3!(-0.5, 0.5, 0.5), vec2!(), color!(1, 1, 1, 1)),
		Vertex::new(vec3!(-0.5, -0.5, -0.5), vec2!(), color!(1, 0, 0, 1)),
		Vertex::new(vec3!(0.5, -0.5, -0.5), vec2!(), color!(0, 1, 0, 1)),
		Vertex::new(vec3!(0.5, 0.5, -0.5), vec2!(), color!(0, 0, 1, 1)),
		Vertex::new(vec3!(-0.5, 0.5, -0.5), vec2!(), color!(1, 1, 1, 1)),

	];

	let cube_indices = [
		0, 1, 2,
		2, 3, 0,
		1, 5, 6,
		6, 2, 1,
		7, 6, 5,
		5, 4, 7,
		4, 0, 3,
		3, 7, 4,
		4, 5, 1,
		1, 0, 4,
		3, 2, 6,
		6, 7, 3,
	];

	let cube_mesh = ggl::Mesh::new(&cube_verts, &cube_indices);

	ctx_init(G3dCtx {

		projection: projection,
		cam: Cam::new(vec3!(), 0.0, 0.0),
		state_stack: Vec::with_capacity(MAX_STATE_STACK),
		state: State::default(),
		default_shader: default_shader.clone(),
		current_shader: default_shader.clone(),
		empty_tex: Texture::from_color(color!(1), 1, 1),
		cube_mesh: cube_mesh,

	});

}

struct Cam {
	front: Vec3,
	pos: Vec3,
}

impl Cam {

	fn new(pos: Vec3, yaw: f32, pitch: f32) -> Self {

		let mut c = Self {
			front: vec3!(),
			pos: vec3!(),
		};

		c.set_pos(pos);
		c.set_angle(yaw, pitch);

		return c;

	}

	fn as_mat(&self) -> Mat4 {
		return math::lookat(self.pos, self.pos + self.front, vec3!(0, 1, 0));
	}

	fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
	}

	fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.front.x = pitch.cos() * yaw.cos();
		self.front.y = pitch.sin();
		self.front.z = pitch.cos() * yaw.sin();
		self.front = self.front.unit();

	}

}

#[derive(Vertex)]
struct Vertex {
	pos: [f32; 3],
	uv: [f32; 2],
	color: [f32; 4],
}

impl Vertex {
	fn new(pos: Vec3, uv: Vec2, c: Color) -> Self {
		return Self {
			pos: [pos.x, pos.y, pos.z],
			uv: [uv.x, uv.y],
			color: [c.r, c.g, c.b, c.a],
		};
	}
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
pub fn rotate(r: Vec3) {

	let state = &mut ctx_get_mut().state;

	if r.x != 0.0 {
		state.transform = state.transform.rotate(r.x, Dir::X);
	}

	if r.y != 0.0 {
		state.transform = state.transform.rotate(r.y, Dir::Y);
	}

	if r.z != 0.0 {
		state.transform = state.transform.rotate(r.z, Dir::Z);
	}

}

/// global scale
pub fn scale(s: Vec3) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.scale(s);

}

/// draw a 2d texture in 3d space
pub fn flag() {
	// ...
}

/// set camera angle
pub fn look(yaw: f32, pitch: f32) {
	ctx_get_mut().cam.set_angle(yaw, pitch);
}

/// set camera pos
pub fn cam(eye: Vec3) {
	ctx_get_mut().cam.set_pos(eye);
}

/// get camera pos
pub fn get_cam() -> Vec3 {
	return ctx_get().cam.pos;
}

/// get camera front
pub fn get_front() -> Vec3 {
	return ctx_get().cam.front;
}

/// draw a cube
pub fn cube() {

	let gfx = ctx_get();
	let model = gfx.state.transform;
	let view = gfx.cam.as_mat();
	let projection = gfx.projection;

	gfx.current_shader.send_mat4("model", model);
	gfx.current_shader.send_mat4("view", view);
	gfx.current_shader.send_mat4("projection", projection);
	gfx.current_shader.send_float("time", app::time());
	gfx.cube_mesh.draw(&gfx.empty_tex.handle, &gfx.current_shader.program);

}

pub(super) fn begin() {}

pub(super) fn end() {

	g3d::reset();
	ctx_get_mut().state_stack.clear();

}

gen_templated_shader!(Shader, TEMPLATE_3D_VERT, TEMPLATE_3D_FRAG, DEFAULT_3D_VERT, DEFAULT_3D_FRAG);

