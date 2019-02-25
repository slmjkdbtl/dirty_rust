// wengwengweng

//! 2D Rendering

use std::collections::HashMap;

use ggl_derive::Vertex;

use crate::*;
use crate::math::*;
use crate::ggl;

const MAX_DRAWS: usize = 65536;
const MAX_STATE_STACK: usize = 64;

include!("../res/resources.rs");

// context
ctx!(G2D: G2dCtx);

struct G2dCtx {

	projection: Mat4,
	state: State,
	state_stack: Vec<State>,
	default_font: Font,
	current_font: Font,
	empty_tex: gfx::Texture,
	current_tex: Option<gfx::Texture>,
	default_shader: Shader,
	current_shader: Shader,
	renderer: ggl::BatchedMesh,
	draw_calls: usize,
	draw_calls_last: usize,

}

pub(super) fn init() {

	let renderer = ggl::BatchedMesh::new::<QuadShape>(MAX_DRAWS);
	let default_shader = Shader::from_code(DEFAULT_2D_VERT, DEFAULT_2D_FRAG);

	let default_font = Font::new(
		gfx::Texture::from_bytes(DEFAULT_FONT),
		DEFAULT_FONT_COLS,
		DEFAULT_FONT_ROWS,
		DEFAULT_FONT_CHARS,
	);

	let (width, height) = window::size();
	let projection = math::ortho(0.0, (width as f32), (height as f32), 0.0, -1.0, 1.0);

	ctx_init(G2dCtx {

		projection: projection,
		state_stack: Vec::with_capacity(MAX_STATE_STACK),
		state: State::default(),
		default_shader: default_shader.clone(),
		default_font: default_font.clone(),
		current_font: default_font,
		empty_tex: gfx::Texture::from_color(color!(1), 1, 1),
		renderer: renderer,
		current_tex: None,
		current_shader: default_shader,
		draw_calls: 0,
		draw_calls_last: 0,

	});

}

/// check if gfx is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

struct QuadShape {

	transform: Mat4,
	quad: Rect,
	color: Color,

}

impl QuadShape {
	fn new(t: Mat4, q: Rect, c: Color) -> Self {
		return Self {
			transform: t,
			quad: q,
			color: c,
		};
	}
}

impl ggl::Shape for QuadShape {

	type Vertex = Vertex2D;
	const COUNT: usize = 4;

	fn push(&self, queue: &mut Vec<f32>) {

		use crate::ggl::VertexLayout;

		let t = &self.transform;
		let q = &self.quad;
		let c = &self.color;

		Self::Vertex::new(t.forward(vec2!(0, 1)), vec2!(q.x, q.y + q.h), *c).push(queue);
		Self::Vertex::new(t.forward(vec2!(1, 1)), vec2!(q.x + q.w, q.y + q.h), *c).push(queue);
		Self::Vertex::new(t.forward(vec2!(1, 0)), vec2!(q.x + q.w, q.y), *c).push(queue);
		Self::Vertex::new(t.forward(vec2!(0, 0)), vec2!(q.x, q.y), *c).push(queue);

	}

	fn indices() -> Vec<u32> {
		return vec![0, 1, 3, 1, 2, 3];
	}

}

#[derive(Vertex)]
struct Vertex2D {
	pos: [f32; 2],
	uv: [f32; 2],
	color: [f32; 4],
}

impl Vertex2D {
	fn new(pos: Vec2, uv: Vec2, c: Color) -> Self {
		return Self {
			pos: [pos.x, pos.y],
			uv: [uv.x, uv.y],
			color: [c.r, c.g, c.b, c.a],
		};
	}
}

#[derive(Clone, Copy)]
struct State {

	transform: Mat4,
	tint: Color,
	line_width: u8,
	text_wrap: Option<u32>,

}

impl Default for State {

	fn default() -> Self {

		return Self {

			transform: Mat4::identity(),
			tint: color!(),
			line_width: 1,
			text_wrap: None,

		}

	}

}

/// reset global transforms and style states
pub fn reset() {
	ctx_get_mut().state = State::default();
}

pub(super) fn flush() {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();
	let renderer = &mut gfx_mut.renderer;

	if let Some(tex) = &gfx.current_tex {

		gfx.current_shader.send_mat4("projection", gfx.projection);
		gfx.current_shader.send_float("time", app::time());
		renderer.flush(&*tex.handle, &gfx.current_shader.program);
		gfx_mut.draw_calls += 1;
		gfx_mut.current_tex = None;

	}

}

pub fn draw_calls() -> usize {
	return ctx_get().draw_calls_last;
}

pub(super) fn begin() {}

pub(super) fn end() {

	let ctx = ctx_get_mut();

	flush();
	reset();
	ctx.draw_calls_last = ctx.draw_calls;
	ctx.draw_calls = 0;
	ctx.state_stack.clear();

}

pub(super) fn flip_projection() {

	let g2d_mut = ctx_get_mut();
	let (width, height) = window::size();

	g2d_mut.projection = math::ortho(0.0, (width as f32), 0.0, (height as f32), -1.0, 1.0);

}

pub(super) fn unflip_projection() {

	let g2d_mut = ctx_get_mut();
	let (width, height) = window::size();

	g2d_mut.projection = math::ortho(0.0, (width as f32), (height as f32), 0.0, -1.0, 1.0);

}

/// draw a texture with visible quad area
pub fn draw(tex: &gfx::Texture, quad: Rect) {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();
	let renderer = &mut gfx_mut.renderer;
	let t = gfx.state.transform.scale(vec3!(tex.width() as f32 * quad.w, tex.height() as f32 * quad.h, 1.0));
	let color = gfx.state.tint;

	let wrapped_tex = Some(tex.clone());

	if gfx.current_tex != wrapped_tex {
		if gfx.current_tex.is_some() {
			flush();
		}
		gfx_mut.current_tex = wrapped_tex;
	}

	renderer.push(QuadShape::new(t, quad, color));

}

/// render a canvas
pub fn render(c: &gfx::Canvas) {
	draw(&c.tex, rect!(0, 0, 1, 1));
}

/// draw text
pub fn text(s: &str) {

	let gfx = ctx_get();
	let font = &gfx.default_font;
	let w = font.grid_size.x * font.tex.width() as f32;
	let h = font.grid_size.y * font.tex.height() as f32;

	let next_line = |st: &str| {

		push();
		translate(vec2!(0, h));
		text(st);
		pop();

	};

	for (i, ch) in s.chars().enumerate() {

		let x = i as f32 * w;

		if let Some(wrap) = gfx.state.text_wrap {

			if x >= wrap as f32 {
				return next_line(&s[i..s.len()]);
			}

		}

		push();
		translate(vec2!(x, 0.0));

		if ch == '\n' {

			pop();

			return next_line(&s[(i + 1) .. s.len()]);

		} else if ch != ' ' {

			let quad = font.map.get(&ch).unwrap_or_else(|| panic!("font does not contain char '{}'", ch));

			draw(&font.tex, *quad);

		}

		pop();

	}

}

/// draw rectangle with size
pub fn rect(size: Vec2) {

	let gfx = ctx_get();

	push();
	scale(size);
	draw(&gfx.empty_tex, rect!(0, 0, 1, 1));
	pop();

}

/// draw line
pub fn line(p1: Vec2, p2: Vec2) {

	let gfx = ctx_get();
	let len = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
	let rot = (p2.y - p1.y).atan2(p2.x - p1.x);

	push();
	translate(p1);
	rotate(rot);
	rect(vec2!(len, gfx.state.line_width));
	pop();

}

/// apply a shader effect
pub fn set_effect(s: &Shader) {
	flush();
	ctx_get_mut().current_shader = s.clone();
}

/// stop shader effects and use default shader
pub fn set_effect_default() {
	flush();
	ctx_get_mut().current_shader = ctx_get().default_shader.clone();
}

/// apply a custom font
pub fn set_font(f: &Font) {
	ctx_get_mut().current_font = f.clone();
}

/// use default font
pub fn set_font_default() {
	ctx_get_mut().current_font = ctx_get().default_font.clone();
}

/// draw polygon with vertices
pub fn poly(pts: &[Vec2]) {

	for (i, p) in pts.iter().enumerate() {

		if (i == pts.len() - 1) {
			line(*p, pts[0]);
		} else {
			line(*p, pts[i + 1]);
		}

	}

}

/// set global tint
pub fn color(tint: Color) {
	ctx_get_mut().state.tint = tint;
}

/// set line width
pub fn line_width(line_width: u8) {
	ctx_get_mut().state.line_width = line_width;
}

/// set text wrap
pub fn text_wrap(wrap: u32) {
	ctx_get_mut().state.text_wrap = Some(wrap);
}

/// push state
pub fn push() {

	let gfx = ctx_get_mut();
	let stack = &mut gfx.state_stack;

	if (stack.len() < MAX_STATE_STACK) {
		stack.push(gfx.state);
	} else {
		panic!("cannot push anymore");
	}

}

/// pop state
pub fn pop() {

	let mut gfx = ctx_get_mut();
	let stack = &mut gfx.state_stack;

	gfx.state = stack.pop().expect("cannot pop anymore");

}

/// global translate
pub fn translate(pos: Vec2) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.translate(vec3!(pos.x, pos.y, 0.0));

}

/// global rotate
pub fn rotate(rot: f32) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.rotate(rot, Dir::Z);

}

/// global 3d rotation
pub fn rotate3d(x: f32, y: f32, z: f32) {

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
pub fn scale(s: Vec2) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.scale(vec3!(s.x, s.y, 1.0));

}

/// warp a 2d point through current transformed matrix
pub fn warp(pt: Vec2) -> Vec2 {

	let gfx = ctx_get();
	let trans = gfx.state.transform;

	return trans.forward(pt);

}

/// warp multiple points
pub fn multi_warp(pts: &[Vec2]) -> Vec<Vec2> {

	 return pts.iter()
		.map(|&p| warp(p))
		.collect();

}

/// inverse warp a 2d point through current transformed matrix
pub fn inverse_warp(pt: Vec2) -> Vec2 {

	let gfx = ctx_get();
	let trans = gfx.state.transform;

	return trans.inverse().forward(pt);

}

/// get the current transform matrix
pub fn get_matrix() -> Mat4 {
	return ctx_get().state.transform;
}

/// get the current transform matrix
pub fn set_matrix(m: Mat4) {
	ctx_get_mut().state.transform = m;
}

/// bitmap font
#[derive(PartialEq, Clone)]
pub struct Font {

	tex: gfx::Texture,
	map: HashMap<char, Rect>,
	grid_size: Vec2,

}

impl Font {

	/// creat a bitmap font from a texture, and grid of characters
	pub fn new(tex: gfx::Texture, cols: usize, rows: usize, chars: &str) -> Self {

		let mut map = HashMap::new();
		let grid_size = vec2!(1.0 / cols as f32, 1.0 / rows as f32);

		assert!(tex.width() % cols as u32 == 0, "font size not right");
		assert!(tex.height() % rows as u32 == 0, "font size not right");

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, rect!(

				(i % cols) as f32 * grid_size.x,
				(i / cols) as f32 * grid_size.y,
				grid_size.x,
				grid_size.y

			));

		}

		return Self {

			tex: tex,
			map: map,
			grid_size: grid_size,

		}

	}

}

gen_templated_shader!(Shader, TEMPLATE_2D_VERT, TEMPLATE_2D_FRAG, DEFAULT_2D_VERT, DEFAULT_2D_FRAG);

