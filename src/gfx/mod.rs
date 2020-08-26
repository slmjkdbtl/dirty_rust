// wengwengweng

//! Graphics
//!
//! ## Drawing Stuff
//!
//! Gfx provides drawing primitives throught the [`shapes`](shapes/index.html) modules.
//!
//! A basic draw operation may look like this:
//! ```ignore
//! gfx.draw(&shapes::text("hi"))?;
//! ```
//!
//! All shapes uses a builder pattern for configs:
//! ```ignore
//! gfx.draw(
//!     &shapes::sprite(&self.tex)
//!         .offset(vec2!(-1.0))
//!         .color(rgba!(0, 0, 1, 1))
//!         .flip(Flip::Y)
//!         ,
//! )?;
//! ```
//!
//! You can transform objects with [`draw_t`](struct.Gfx.html#method.draw_t):
//! ```ignore
//! gfx.draw_t(
//!     mat4!()
//!         .t3(vec2!(120))
//!         .rx(f32::to_radians(90.0))
//!         .s3(vec2!(2))
//!         ,
//!     &shapes::model(&self.model)
//!         .color(rgba!(0, 1, 1, 1))
//!         ,
//! )?;
//! ```
//!
//! There's also [`push_t`](struct.Gfx.html#method.push_t) that transforms every draw operations in the callback:
//! ```ignore
//! gfx.push_t(mat4!().t2(vec2!(120)), |gfx| {
//!
//!     gfx.draw(&shapes::text("we"))?;
//!     gfx.draw(&shapes::text("are"))?;
//!     gfx.draw(&shapes::text("all"))?;
//!     gfx.draw(&shapes::text("transformed"))?;
//!
//!     return Ok(());
//!
//! })?;
//! ```
//! This kind of callback pattern can be seen in a lot of functions under [`Gfx`](struct.Gfx.html), as it's using an stateless architecture for rendering states.
//!
//!
//! ## Canvas
//!
//! You can use an off-screen framebuffer with [`Canvas`](struct.Canvas.html) and [`draw_on`](struct.Gfx.html#method.draw_on):
//!
//! Use [`draw_on`](struct.Gfx.html#method.draw_on) to use custom camera
//!
//! ```ignore
//! // init
//! let canvas = Canvas::new(&gfx, 120, 120)?;
//!
//! // mostly called in update, but also can be in init if you're not updating
//! gfx.draw_on(&canvas, |gfx| {
//!     gfx.draw(&shapes::text("anything"))?;
//!     return Ok(());
//! })?;
//! ```
//! Canvases can be used for a lot of things: post-processing, screenshots, ...
//!
//! note that binding to a canvas resets the projection & view matrix, you may want to rebind your camera in a canvas call
//!
//! also remember to resize canvas when window resizes if you have a fullscreen canvas, and recreate canvas when window DPI changes
//!
//! ## Camera
//!
//! Cameras implement the [`Camera`](trait.Camera.html) trait, which lets you define your own projection and view matrix.
//!
//! We provide 2 built in cameras, [`OrthoCam`](struct.OrthoCam.html) and [`PerspectiveCam`](struct.PerspectiveCam.html).
//!
//! Use [`use_cam`](struct.Gfx.html#method.use_cam) to use custom camera
//!
//! ```ignore
//! let cam = gfx::PerspectiveCam {
//!    fov: f32::to_radians(60.0),
//!    up: vec3!(0, 1, 0),
//!    aspect: d.gfx.width() as f32 / d.gfx.height() as f32,
//!    near: 0.1,
//!    far: 1024.0,
//!    pos: vec3!(0),
//!    dir: vec3!(0, 0, -1),
//! };
//!
//! d.gfx.use_cam(&cam, |gfx| {
//!     // draw stuff with cam
//!     return Ok(());
//! })?;
//! ```
//!
//! ## Shader
//!
//! Use [`Shader`](struct.Shader.html) to create custom shaders. It requires a type that implements [`UniformLayout`](trait.UniformLayout.html), a minimal example:
//!
//! Use [`draw_with`](struct.Gfx.html#method.draw_with) to use custom camera
//!
//! ```glsl
//! // blue.frag
//! uniform float u_blueness;
//! fn frag() {
//!     return default_color() * u_blueness * vec4(0.0, 0.0, 1.0, 1.0);
//! }
//! ```
//!
//! ```ignore
//! struct BlueUniform {
//!     blueness: f32,
//! }
//!
//! impl UniformLayout for BlueUniform {
//!     fn data(&self) -> UniformDatas {
//!         return hmap![
//!             "u_blueness": &self.blueness,
//!         ];
//!     }
//! }
//!
//! // init
//! let shader = Shader::<BlueUniform>::from_frag(gfx, include_str!("blue.frag"))?;
//!
//! // draw
//! gfx.draw_with(&shader, &BlueUniform {
//!     blueness: 1.0,
//! }, |gfx| {
//!     return Ok(());
//! })?;
//! ```
//!
//! custom shaders have access to these following inputs:
//!
//! | prefix  | type      | name          | desc                            | visibility |
//! |---------|-----------|---------------|---------------------------------|------------|
//! | varing  | vec3      | v_pos         | vertex position                 | all        |
//! | varing  | vec3      | v_normal      | vertex normal                   | all        |
//! | varing  | vec2      | v_uv          | vertex texture coord            | all        |
//! | varing  | vec4      | v_color       | vertex color                    | all        |
//! | uniform | mat4      | u_model       | uniform model matrix            | vert       |
//! | uniform | mat4      | u_proj        | uniform projection matrix       | vert       |
//! | uniform | mat4      | u_view        | uniform view matrix             | vert       |
//! | uniform | mat4      | u_view        | uniform view matrix             | vert       |
//! | uniform | sampler2D | u_tex         | current texture                 | frag       |
//! | uniform | vec4      | u_color       | uniform color                   | frag       |
//! |         | vec4()    | default_pos   | get the default vertex position | vert       |
//! |         | vec4()    | default_color | get the default fragment color  | frag       |

import!(buffer);
import!(pipeline);
import!(batch);

export!(types);
export!(desc);
export!(mesh);
export!(texture);
export!(canvas);
export!(shader);
export!(transform);
export!(camera);
export!(font);
export!(uniform);
export!(model);

pub mod shapes;
pub mod fonts;
pub mod shaders;

use std::mem;
use std::rc::Rc;
use std::marker::PhantomData;
use std::collections::HashMap;

use glow::HasContext;
use serde::Serialize;
use serde::Deserialize;

use crate::*;
use math::*;
use window::*;
use geom::*;

const DRAW_COUNT: usize = 65536;
const DEFAULT_NEAR: f32 = -4096.0;
const DEFAULT_FAR: f32 = 4096.0;

/// The Graphics Context. See [mod-level doc](index.html) for usage.
pub struct Gfx {

	gl: Rc<glow::Context>,

	width: i32,
	height: i32,
	dpi: f32,

	proj: Mat4,
	view: Mat4,
	transform: Mat4,

	renderer: BatchedRenderer<Vertex, Uniform>,

	empty_tex: gfx::Texture,

	default_pipeline: Pipeline<gfx::Vertex, gfx::Uniform>,
	cur_pipeline: Pipeline<gfx::Vertex, gfx::Uniform>,
	cur_custom_uniform: Option<Vec<(&'static str, UniformData)>>,

	on_canvas: bool,

	default_font: gfx::BitmapFont,

	draw_calls_last: usize,
	draw_calls: usize,

}

pub trait GLCtx {
	fn gl(&self) -> &Rc<glow::Context>;
}

impl GLCtx for Gfx {
	fn gl(&self) -> &Rc<glow::Context> {
		return &self.gl;
	}
}

impl GLCtx for Rc<glow::Context> {
	fn gl(&self) -> &Rc<glow::Context> {
		return &self;
	}
}

impl Gfx {

	pub(crate) fn new(window: &Window, conf: &conf::Conf) -> Result<Self> {

		let gl = window.gl();

		use types::*;

		unsafe {

			gl.enable(Capability::Blend.as_glow());
			gl.enable(Capability::DepthTest.as_glow());
			gl.blend_func(BlendFac::SrcAlpha.as_glow(), BlendFac::OneMinusSrcAlpha.as_glow());
			gl.depth_func(Cmp::LessOrEqual.as_glow());

			// TODO: cull face doesn't work with some of the default geoms
			if conf.cull_face {
				gl.enable(Capability::CullFace.as_glow());
				gl.cull_face(Face::Back.as_glow());
				gl.front_face(CullMode::CounterClockwise.as_glow());
			}

			if conf.multi_sample.is_some() {
				gl.enable(Capability::MultiSample.as_glow());
			}

			let cc = conf.clear_color;

			gl.clear_color(cc.r, cc.g, cc.b, cc.a);
			gl.clear(Surface::Color.as_glow());
			gl.clear(Surface::Depth.as_glow());
			gl.clear(Surface::Stencil.as_glow());

		}

		let cam = OrthoCam {
			width: conf.width as f32,
			height: conf.height as f32,
			near: DEFAULT_NEAR,
			far: DEFAULT_FAR,
		};

		let vert_src = shaders::TEMPLATE_VERT.replace("{{user}}", shaders::DEFAULT_VERT);
		let frag_src = shaders::TEMPLATE_FRAG.replace("{{user}}", shaders::DEFAULT_FRAG);
		#[cfg(any(web, mobile))]
		let frag_src = format!("{}{}", "precision mediump float;", frag_src);

		let pipeline = Pipeline::new(gl, &vert_src, &frag_src)?;

		let font_data = conf.default_font
			.clone()
			.take()
			.unwrap_or(fonts::UNSCII);

		let font = gfx::BitmapFont::from_data(gl, font_data)?;

		let init_state = GLState {
			blend: BlendState {
				rgb_src: BlendFac::SrcAlpha,
				rgb_dest: BlendFac::OneMinusSrcAlpha,
				a_src: BlendFac::SrcAlpha,
				a_dest: BlendFac::OneMinusSrcAlpha,
			},
			depth_write: true,
			depth_test: Some(Cmp::LessOrEqual),
			stencil_write: false,
			stencil_test: None,
		};

		return Ok(Self {

			width: window.width(),
			height: window.height(),
			dpi: window.dpi(),

			renderer: BatchedRenderer::<Vertex, Uniform>::new(gl, DRAW_COUNT, DRAW_COUNT)?,

			view: cam.view(),
			proj: cam.proj(),
			transform: mat4!(),

			default_pipeline: pipeline.clone(),
			cur_pipeline: pipeline,
			cur_custom_uniform: None,

			on_canvas: false,

			draw_calls_last: 0,
			draw_calls: 0,

			empty_tex: Texture::from_raw_with_conf(gl, 1, 1, &[255; 4], TextureConf {
				filter: FilterMode::Nearest,
				wrap: WrapMode::Repeat,
			})?,

			default_font: font,

			gl: gl.clone(),

		});

	}

	/// draw a [`Drawable`](trait.Drawable.html)
	pub fn draw(&mut self, shape: &impl Drawable) -> Result<()> {
		return shape.draw(self);
	}

	/// draw a [`Drawable`](trait.Drawable.html) with transform
	pub fn draw_t(&mut self, t: Mat4, shape: &impl Drawable) -> Result<()> {
		return self.push_t(t, |ctx| {
			return ctx.draw(shape);
		});
	}

	// TODO: alias this closure type
	/// draw everything inside with transform
	pub fn push_t(
		&mut self,
		t: Mat4,
		f: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {

		let ot = self.transform;

		self.transform = ot * t;
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	// TODO: viewport 2x scaled with no hidpi
	/// draw on a [`Canvas`](struct.Canvas.html)
	pub fn draw_on(
		&mut self,
		canvas: &Canvas,
		action: CanvasAction,
		f: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {

		if self.on_canvas {
			return Err(format!("cannot use canvas inside a canvas"));
		}

		self.flush();

		let t = self.transform;
		let (cw, ch) = (canvas.width(), canvas.height());

		let new_cam = OrthoCam {
			width: cw as f32,
			height: ch as f32,
			near: DEFAULT_NEAR,
			far: DEFAULT_FAR,
		};

		let oproj = self.proj;
		let oview = self.view;

		self.proj = new_cam.proj();
		self.view = new_cam.view();

		self.on_canvas = true;
		self.transform = mat4!();

		canvas.bind();

		unsafe {

			self.gl.viewport(
				0,
				0,
				(cw as f32 * self.dpi) as i32,
				(ch as f32 * self.dpi) as i32,
			);

			if action.color == CanvasOp::Clear {
				self.gl.clear(Surface::Color.as_glow());
			}

			if action.depth == CanvasOp::Clear {
				self.gl.clear(Surface::Depth.as_glow());
			}

			if action.stencil == CanvasOp::Clear {
				self.gl.clear(Surface::Stencil.as_glow());
			}

		}

		f(self)?;
		self.flush();
		canvas.unbind();

		self.on_canvas = false;
		self.transform = t;

		self.proj = oproj;
		self.view = oview;

		unsafe {
			self.gl.viewport(
				0,
				0,
				(self.width as f32 * self.dpi) as i32,
				(self.height as f32 * self.dpi) as i32,
			);
		}

		return Ok(());

	}

	/// draw with a [`Shader`](struct.Shader.html)
	pub fn draw_with<U: UniformLayout>(
		&mut self,
		shader: &Shader<U>,
		uniform: &U,
		f: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {

		let prev_pipeline = self.cur_pipeline.clone();
		let prev_uniform = self.cur_custom_uniform.clone();

		self.flush();
		self.cur_pipeline = Pipeline::clone(&shader.pipeline());
		self.cur_custom_uniform = Some(uniform.data());
		f(self)?;
		self.flush();
		self.cur_pipeline = prev_pipeline;
		self.cur_custom_uniform = prev_uniform;

		return Ok(());

	}

	/// draw with stencil operations
	pub fn draw_masked_ex(
		&mut self,
		// if render the stencil write function or not
		draw_write: bool,
		f1: impl FnOnce(&mut Self) -> Result<()>,
		// draw pixels that're present in the stencil buffer or draw those are not
		draw_equal: bool,
		f2: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {

		unsafe {

			self.flush();
			self.gl.enable(Capability::StencilTest.as_glow());
			self.gl.clear(Surface::Stencil.as_glow());

			// write to stencil buffer
			let c1 = if draw_write { Cmp::Always } else { Cmp::Never };
			self.gl.stencil_op(StencilOp::Replace.as_glow(), StencilOp::Replace.as_glow(), StencilOp::Replace.as_glow());
			self.gl.stencil_func(c1.as_glow(), 1, 0xff);
			f1(self)?;
			self.flush();

			// draw with stencil test
			let c2 = if draw_equal { Cmp::Equal } else { Cmp::NotEqual };
			self.gl.stencil_op(StencilOp::Keep.as_glow(), StencilOp::Keep.as_glow(), StencilOp::Keep.as_glow());
			self.gl.stencil_func(c2.as_glow(), 1, 0xff);
			f2(self)?;
			self.flush();

			self.gl.disable(Capability::StencilTest.as_glow());

		}

		return Ok(());
	}

	/// mask pixels from first call to the second
	pub fn draw_masked(
		&mut self,
		mask: impl FnOnce(&mut Self) -> Result<()>,
		draw: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {
		return self.draw_masked_ex(false, mask, true, draw);
	}

	fn transform_pt(&self, pt: Vec2) -> Vec2 {
		return vec2!(pt.x + self.width as f32 / 2.0, pt.y + self.height as f32 / 2.0) * self.dpi;
	}

	/// draw within a rect
	pub fn draw_within(
		&mut self,
		p1: Vec2,
		p2: Vec2,
		f: impl FnOnce(&mut Self) -> Result<()>
	) -> Result<()> {

		let pt1 = self.transform * p1;
		let pt2 = self.transform * p2;
		let (pt1, pt2) = (
			self.transform_pt(vec2!(f32::min(pt1.x, pt2.x), f32::min(pt1.y, pt2.y))),
			self.transform_pt(vec2!(f32::max(pt1.x, pt2.x), f32::max(pt1.y, pt2.y))),
		);
		let x = pt1.x;
		let y = pt1.y;
		let w = (pt2.x - pt1.x);
		let h = (pt2.y - pt1.y);

		unsafe {

			self.flush();
			self.gl.enable(Capability::ScissorTest.as_glow());
			self.gl.scissor(x as i32, y as i32, w as i32, h as i32);
			self.push_t(mat4!().t2(p1), |gfx| {
				return f(gfx);
			})?;
			self.flush();
			self.gl.disable(Capability::ScissorTest.as_glow());

		}

		return Ok(());

	}

	/// use custom blending
	pub fn use_blend(
		&mut self,
		b: Blend,
		f: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {

		let default = Blend::Alpha.state();
		let custom = b.state();

		unsafe {

			self.flush();

			self.gl.blend_func_separate(
				custom.rgb_src.as_glow(),
				custom.rgb_dest.as_glow(),
				custom.a_src.as_glow(),
				custom.a_dest.as_glow(),
			);

			f(self)?;
			self.flush();

			self.gl.blend_func_separate(
				default.rgb_src.as_glow(),
				default.rgb_dest.as_glow(),
				default.a_src.as_glow(),
				default.a_dest.as_glow(),
			);

		}

		return Ok(());

	}

	/// use a [`Camera`](trait.Camera.html)
	pub fn use_cam(
		&mut self,
		cam: &dyn Camera,
		f: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {

		let oview = self.view;
		let oproj = self.proj;

		self.apply_cam(cam);

		f(self)?;

		self.view = oview;
		self.proj = oproj;

		return Ok(());

	}

	/// use the default camera
	pub fn use_default_cam(
		&mut self,
		f: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {
		let cam = self.default_cam();
		return self.use_cam(&cam, f);
	}

	/// temporarily disable depth write
	pub fn no_depth_write(
		&mut self,
		f: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {

		unsafe {
			self.flush();
			self.gl.depth_mask(false);
			f(self)?;
			self.flush();
			self.gl.depth_mask(true);
		}

		return Ok(());

	}

	/// temporarily disable depth test
	pub fn no_depth_test(
		&mut self,
		f: impl FnOnce(&mut Self) -> Result<()>,
	) -> Result<()> {

		unsafe {
			self.flush();
			self.gl.disable(Capability::DepthTest.as_glow());
			f(self)?;
			self.flush();
			self.gl.enable(Capability::DepthTest.as_glow());
		}

		return Ok(());

	}

	/// get current transform
	pub fn transform(&self) -> Mat4 {
		return self.transform;
	}

	/// get position of a window [`Origin`](struct.Origin.html)
	pub fn coord(&self, orig: gfx::Origin) -> Vec2 {
		return orig.as_pt() / 2.0 * vec2!(self.width, self.height);
	}

	/// transform a point from clip space to screen space
	pub fn clip_to_screen(&self, p: Vec2) -> Vec2 {
		return p * vec2!(self.width, self.height) * 0.5;
	}

	/// transform a point from screen space to clip space
	pub fn screen_to_clip(&self, p: Vec2) -> Vec2 {
		return p / 0.5 / vec2!(self.width, self.height);
	}

	/// get default font
	pub fn default_font(&self) -> &impl Font {
		return &self.default_font;
	}

	pub fn flush(&mut self) {
		self.renderer.flush();
	}

	pub(crate) fn set_dpi(&mut self, dpi: f32) {
		self.dpi = dpi;
	}

	pub(crate) fn resize(&mut self, w: i32, h: i32) {

		self.width = w;
		self.height = h;

		let cam = self.default_cam();

		self.apply_cam(&cam);

	}

	pub(crate) fn apply_cam(&mut self, cam: &dyn Camera) {
		self.proj = cam.proj();
		self.view = cam.view();
	}

	pub(crate) fn default_cam(&mut self) -> impl Camera {
		return OrthoCam {
			width: self.width as f32,
			height: self.height as f32,
			near: DEFAULT_NEAR,
			far: DEFAULT_FAR,
		};
	}

	pub(crate) fn begin_frame(&mut self) {

		self.draw_calls_last = self.draw_calls;
		self.draw_calls = 0;

		unsafe {

			self.gl.clear(Surface::Color.as_glow());
			self.gl.clear(Surface::Depth.as_glow());
			self.gl.clear(Surface::Stencil.as_glow());

			self.gl.viewport(
				0,
				0,
				(self.width as f32 * self.dpi) as i32,
				(self.height as f32 * self.dpi) as i32,
			);
		}

	}

	pub(crate) fn end_frame(&mut self) {
		self.flush();
		self.transform = mat4!();
		self.draw_calls += self.renderer.draw_count();
		self.renderer.clear_draw_count();
	}

	pub fn width(&self) -> i32 {
		return self.width;
	}

	pub fn height(&self) -> i32 {
		return self.height;
	}

	pub fn dpi(&self) -> f32 {
		return self.dpi;
	}

	pub fn draw_calls(&self) -> usize {
		return self.draw_calls_last;
	}

}

pub(self) fn draw<V: VertexLayout, U: UniformLayout>(
	ctx: &impl GLCtx,
	prim: Primitive,
	pip: &Pipeline<V, U>,
	vbuf: &VertexBuffer<V>,
	ibuf: &IndexBuffer,
	count: usize,
	uniform: &U,
) {

	unsafe {

		let gl = ctx.gl();

		pip.bind();
		vbuf.bind();
		bind_attrs::<V>(&gl);
		ibuf.bind();

		let mut tex_slots = vec![];

		for (name, data) in uniform.data() {

			let loc = pip.loc(name);

			if loc.is_some() {
				match data {
					UniformData::Float(f) => gl.uniform_1_f32(loc.as_ref(), f),
					UniformData::Vec2(f) => gl.uniform_2_f32(loc.as_ref(), f.x, f.y),
					UniformData::Vec3(f) => gl.uniform_3_f32(loc.as_ref(), f.x, f.y, f.z),
					UniformData::Vec4(f) => gl.uniform_4_f32(loc.as_ref(), f.x, f.y, f.z, f.w),
					UniformData::Int(i) => gl.uniform_1_i32(loc.as_ref(), i),
					UniformData::Mat4(m) => gl.uniform_matrix_4_f32_slice(loc.as_ref(), false, &m.as_arr()),
					UniformData::Texture(tex) => {
						gl.uniform_1_i32(loc.as_ref(), tex_slots.len() as i32);
						gl.active_texture(glow::TEXTURE0 + tex_slots.len() as u32);
						tex.bind();
						tex_slots.push(tex.clone());
					},
				}
			}

		}

		match prim {
			Primitive::Line(w) => gl.line_width(w),
			_ => {},
		}

		gl.draw_elements(prim.as_glow(), count as i32, glow::UNSIGNED_INT, 0);

		ibuf.unbind();
		vbuf.unbind();
		gl.use_program(None);

		for (i, tex) in tex_slots.into_iter().enumerate() {
			gl.active_texture(glow::TEXTURE0 + i as u32);
			tex.unbind();
		}

	}

}

pub trait Drawable {
	fn draw(&self, ctx: &mut Gfx) -> Result<()>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BlendState {
	rgb_src: BlendFac,
	rgb_dest: BlendFac,
	a_src: BlendFac,
	a_dest: BlendFac,
}

// TODO
#[derive(Clone, Copy, Debug, PartialEq)]
struct GLState {
	blend: BlendState,
	depth_test: Option<Cmp>,
	depth_write: bool,
	stencil_test: Option<Cmp>,
	stencil_write: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Primitive {
	Point(f32),
	Line(f32),
	Triangle,
	LineStrip,
	TriangleFan,
	TriangleStrip,
}

impl Primitive {

	pub(super) fn as_glow(&self) -> u32 {
		return match self {
			Primitive::Point(_) => glow::POINTS,
			Primitive::Line(_) => glow::LINES,
			Primitive::Triangle => glow::TRIANGLES,
			Primitive::LineStrip => glow::LINE_STRIP,
			Primitive::TriangleFan => glow::TRIANGLE_FAN,
			Primitive::TriangleStrip => glow::TRIANGLE_STRIP,
		};
	}

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flip {
	None,
	X,
	Y,
	XY,
}

/// built-in blend modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Blend {
	Alpha,
	Add,
	Replace,
}

impl Blend {
	fn state(&self) -> BlendState {
		return match self {
			Blend::Alpha => BlendState {
				rgb_src: BlendFac::SrcAlpha,
				rgb_dest: BlendFac::OneMinusSrcAlpha,
				a_src: BlendFac::SrcAlpha,
				a_dest: BlendFac::OneMinusSrcAlpha
			},
			Blend::Add => BlendState {
				rgb_src: BlendFac::SrcAlpha,
				rgb_dest: BlendFac::DestAlpha,
				a_src: BlendFac::SrcAlpha,
				a_dest: BlendFac::DestAlpha
			},
			Blend::Replace => BlendState {
				rgb_src: BlendFac::SrcAlpha,
				rgb_dest: BlendFac::Zero,
				a_src: BlendFac::SrcAlpha,
				a_dest: BlendFac::Zero,
			},
		};
	}
}

/// origin point of a view
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Origin {
	TopLeft,
	Top,
	TopRight,
	Left,
	Center,
	Right,
	BottomLeft,
	Bottom,
	BottomRight,
}

impl Origin {

	pub fn as_pt(&self) -> Vec2 {

		use Origin::*;

		return match self {
			TopLeft => vec2!(-1, 1),
			Top => vec2!(0, 1),
			TopRight => vec2!(1, 1),
			Left => vec2!(-1, 0),
			Center => vec2!(0, 0),
			Right => vec2!(1, 0),
			BottomLeft => vec2!(-1, -1),
			Bottom => vec2!(0, -1),
			BottomRight => vec2!(1, -1),
		};

	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CanvasOp {
	Clear,
	Load,
}

/// specifies which buffer to clear or load
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanvasAction {
	pub color: CanvasOp,
	pub depth: CanvasOp,
	pub stencil: CanvasOp,
}

impl CanvasAction {
	pub fn clear() -> Self {
		return Self {
			color: CanvasOp::Clear,
			depth: CanvasOp::Clear,
			stencil: CanvasOp::Clear,
		};
	}
	pub fn load() -> Self {
		return Self {
			color: CanvasOp::Load,
			depth: CanvasOp::Load,
			stencil: CanvasOp::Load,
		};
	}
}

