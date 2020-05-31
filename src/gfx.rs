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
//! ## Custom Shader
//!
//! Use [`Shader`](struct.Shader.html) to create custom shaders. It requires a type that implements [`CustomUniform`](trait.CustomUniform.html), a minimal example:
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
//! impl CustomUniform for BlueUniform {
//!     fn values(&self) -> UniformValues {
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
//!
//! ## Camera
//!
//! ## Memory Management
//!
//! OpenGL uses its own heap memory allocation, so you'll have to free memory yourself when you're done with them. Resource types [`Texture`](struct.Texture.html), [`Model`](struct.Model.html), [`Shader`](struct.Shader.html), [`Canvas`](struct.Canvas.html) and fonts all have a `free(self)` method that frees the memory.

// TODO: major cleaning

mod gltypes;
import!(vbuf);
import!(ibuf);
import!(pipeline);
import!(renderer);
import!(mesh);

export!(desc);
export!(texture);
export!(canvas);
export!(shader);
export!(transform);
export!(camera);
export!(font);
export!(uniform);
export!(model);

pub mod shapes;

use std::rc::Rc;

use glow::HasContext;

use crate::*;
use math::*;
use window::*;

pub(self) type BufferID = <glow::Context as HasContext>::Buffer;
pub(self) type ProgramID = <glow::Context as HasContext>::Program;
pub(self) type TextureID = <glow::Context as HasContext>::Texture;
pub(self) type FramebufferID = <glow::Context as HasContext>::Framebuffer;
pub(self) type RenderbufferID = <glow::Context as HasContext>::Renderbuffer;

use gltypes::*;

pub use gltypes::Surface;
pub use gltypes::Primitive;

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

	renderer: BatchedMesh<Vertex, Uniform>,

	empty_tex: gfx::Texture,

	default_pipeline: Pipeline<gfx::Vertex, gfx::Uniform>,
	cur_pipeline: Pipeline<gfx::Vertex, gfx::Uniform>,
	cur_custom_uniform: Option<Vec<(&'static str, UniformValue)>>,

	cur_canvas: Option<Canvas>,

	default_font: gfx::BitmapFont,

	draw_calls_last: usize,
	draw_calls: usize,

}

pub trait HasGL {
	fn gl(&self) -> &Rc<glow::Context>;
}

impl HasGL for Gfx {
	fn gl(&self) -> &Rc<glow::Context> {
		return &self.gl;
	}
}

impl HasGL for &Rc<glow::Context> {
	fn gl(&self) -> &Rc<glow::Context> {
		return &self;
	}
}

impl Gfx {

	pub(crate) fn new(window: &Window, conf: &conf::Conf) -> Result<Self> {

		let gl = window.gl();

		use gltypes::*;

		unsafe {

			gl.enable(Capability::Blend.into());
			gl.enable(Capability::DepthTest.into());
			gl.blend_func(BlendFac::SrcAlpha.into(), BlendFac::OneMinusSrcAlpha.into());
			gl.depth_func(Cmp::LessOrEqual.into());

			if conf.cull_face {
				gl.enable(Capability::CullFace.into());
				gl.cull_face(Face::Back.into());
				gl.front_face(CullMode::CounterClockwise.into());
			}

			gl.clear_color(0.0, 0.0, 0.0, 1.0);
			gl.clear(Surface::Color.into());
			gl.clear(Surface::Depth.into());
			gl.clear(Surface::Stencil.into());

		}

		let cam = OrthoCam {
			width: conf.width as f32,
			height: conf.height as f32,
			near: DEFAULT_NEAR,
			far: DEFAULT_FAR,
		};

		let vert_src = res::shader::TEMPLATE_VERT.replace("{{user}}", res::shader::DEFAULT_VERT);
		let frag_src = res::shader::TEMPLATE_FRAG.replace("{{user}}", res::shader::DEFAULT_FRAG);
		#[cfg(web)]
		let frag_src = format!("{}{}", "precision mediump float;", frag_src);

		let pipeline = Pipeline::new(&gl, &vert_src, &frag_src)?;

		let font_data = conf.default_font
			.clone()
			.take()
			.unwrap_or(res::font::UNSCII);

		let font = gfx::BitmapFont::from_data(&gl, font_data)?;

		return Ok(Self {

			width: window.width(),
			height: window.height(),
			dpi: window.dpi(),

			renderer: BatchedMesh::<Vertex, Uniform>::new(&gl, DRAW_COUNT, DRAW_COUNT)?,

			view: cam.view(),
			proj: cam.proj(),
			transform: mat4!(),

			default_pipeline: pipeline.clone(),
			cur_pipeline: pipeline,
			cur_custom_uniform: None,

			cur_canvas: None,

			draw_calls_last: 0,
			draw_calls: 0,

			empty_tex: Texture::from_raw(&gl, 1, 1, &[255; 4])?,

			default_font: font,

			gl: gl.clone(),

		});

	}

	pub fn clear(&mut self) {

		self.flush();

		unsafe {
			self.gl.clear(Surface::Color.into());
			self.gl.clear(Surface::Depth.into());
			self.gl.clear(Surface::Stencil.into());
		}

	}

	pub fn clear_ex(&mut self, s: Surface) {

		self.flush();

		unsafe {
			self.gl.clear(s.into());
		}

	}

	pub fn coord(&self, orig: gfx::Origin) -> Vec2 {
		return orig.as_pt() / 2.0 * vec2!(self.width, self.height);
	}

	pub fn clip_to_screen(&self, p: Vec2) -> Vec2 {
		return p * vec2!(self.width, self.height) * 0.5;
	}

	pub fn screen_to_clip(&self, p: Vec2) -> Vec2 {
		return p / 0.5 / vec2!(self.width, self.height);
	}

	pub fn push_t(&mut self, t: Mat4, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let ot = self.transform;

		self.transform = ot * t;
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	pub fn reset_t(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let ot = self.transform;

		self.transform = mat4!();
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	pub fn draw(&mut self, shape: &impl Drawable) -> Result<()> {
		return shape.draw(self);
	}

	pub fn draw_t(&mut self, t: Mat4, shape: &impl Drawable) -> Result<()> {
		return self.push_t(t, |ctx| {
			return ctx.draw(shape);
		});
	}

	// TODO: viewport 2x scaled with no hidpi
	pub fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		if self.cur_canvas.is_some() {
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

		self.cur_canvas = Some(canvas.clone());
		self.transform = mat4!();

		unsafe {
			self.gl.viewport(
				0,
				0,
				(cw as f32 * self.dpi) as i32,
				(ch as f32 * self.dpi) as i32,
			);
		}

		canvas.bind();
		f(self)?;
		self.flush();
		canvas.unbind();

		self.cur_canvas = None;
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

	pub fn draw_with<U: CustomUniform>(&mut self, shader: &Shader<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let uniforms = uniform.values()
			.into_iter()
			.map(|(n, v)| (n, v.into_uniform()))
			.collect::<Vec<(&'static str, UniformValue)>>();

		let prev_pipeline = self.cur_pipeline.clone();
		let prev_uniform = self.cur_custom_uniform.clone();

		self.flush();
		self.cur_pipeline = Pipeline::clone(&shader.pipeline());
		self.cur_custom_uniform = Some(uniforms);
		f(self)?;
		self.flush();
		self.cur_pipeline = prev_pipeline;
		self.cur_custom_uniform = prev_uniform;

		return Ok(());

	}

	// TODO: learn more about stencil
	pub fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		self.flush();

		unsafe {

			self.gl.enable(Capability::StencilTest.into());
			self.gl.clear(Surface::Stencil.into());

			self.gl.stencil_func(Cmp::Never.into(), 1, 0xff);
			self.gl.stencil_op(StencilOp::Replace.into(), StencilOp::Replace.into(), StencilOp::Replace.into());

			mask(self)?;
			self.flush();

			self.gl.stencil_func(Cmp::Equal.into(), 1, 0xff);
			self.gl.stencil_op(StencilOp::Keep.into(), StencilOp::Keep.into(), StencilOp::Keep.into());

			draw(self)?;
			self.flush();

			self.gl.disable(Capability::StencilTest.into());

		}

		return Ok(());

	}

	pub fn use_blend(&mut self, b: Blend, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let (dsrc, ddest) = Blend::Alpha.to_gl();
		let (src, dest) = b.to_gl();

		unsafe {
			self.flush();
			self.gl.blend_func(src.into(), dest.into());
			f(self)?;
			self.flush();
			self.gl.blend_func(dsrc.into(), ddest.into());
		}

		return Ok(());

	}

	pub fn use_cam(&mut self, cam: &dyn Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let oview = self.view;
		let oproj = self.proj;

		self.apply_cam(cam);

		f(self)?;

		self.view = oview;
		self.proj = oproj;

		return Ok(());

	}

	pub fn use_default_cam(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
		let cam = self.default_cam();
		return self.use_cam(&cam, f);
	}

	pub fn transform(&self) -> Mat4 {
		return self.transform;
	}

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
		self.clear();
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

#[derive(Debug, Clone, Copy)]
pub enum Flip {
	None,
	X,
	Y,
	XY,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Blend {
	Alpha,
	Add,
	Replace,
}

impl Blend {
	fn to_gl(&self) -> (BlendFac, BlendFac) {
		return match self {
			Blend::Alpha => (BlendFac::SrcAlpha, BlendFac::OneMinusSrcAlpha),
			Blend::Add => (BlendFac::SrcAlpha, BlendFac::DestAlpha),
			Blend::Replace => (BlendFac::SrcAlpha, BlendFac::Zero),
		};
	}
}

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

pub trait Drawable {
	fn draw(&self, ctx: &mut Gfx) -> Result<()>;
}

