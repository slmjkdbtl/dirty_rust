// wengwengweng

mod desc;
mod texture;
mod canvas;
mod model;
mod transform;
mod camera;
mod shader;
mod font;
pub mod shapes;

use std::rc::Rc;

use crate::*;
use math::*;
use window::*;

pub use desc::*;
pub use shader::*;
pub use camera::*;
pub use texture::*;
pub use canvas::*;
pub use model::*;
pub use transform::*;
pub use font::*;

pub use gl::UniformValues;
pub use gl::IntoUniformValue;
pub use gl::FilterMode;
pub use gl::Surface;
pub use gl::Cmp;
pub use gl::Primitive;

const DRAW_COUNT: usize = 65536;
const DEFAULT_NEAR: f32 = -4096.0;
const DEFAULT_FAR: f32 = 4096.0;

pub struct Gfx {

	gl: Rc<gl::Device>,

	width: i32,
	height: i32,
	dpi: f32,

	proj: Mat4,
	view: Mat4,
	transform: Mat4,

	renderer: gl::BatchedMesh<Vertex, Uniform>,

	empty_tex: gfx::Texture,

	default_pipeline: gl::Pipeline<gfx::Vertex, gfx::Uniform>,
	cur_pipeline: gl::Pipeline<gfx::Vertex, gfx::Uniform>,
	cur_custom_uniform: Option<Vec<(&'static str, gl::UniformValue)>>,

	cur_canvas: Option<Canvas>,

	default_font: gfx::BitmapFont,

	draw_calls_last: usize,
	draw_calls: usize,

}

pub trait HasGLDevice {
	fn device(&self) -> &gl::Device;
}

impl HasGLDevice for Gfx {
	fn device(&self) -> &gl::Device {
		return &self.gl;
	}
}

impl HasGLDevice for gl::Device {
	fn device(&self) -> &gl::Device {
		return &self;
	}
}

impl Gfx {

	pub(crate) fn new(window: &Window, conf: &Conf) -> Result<Self> {

		let gl = window.get_gl_ctx()?;

		gl.enable(gl::Capability::Blend);
		gl.enable(gl::Capability::DepthTest);
		gl.blend_func(gl::BlendFac::SrcAlpha, gl::BlendFac::OneMinusSrcAlpha);
		gl.depth_func(gl::Cmp::LessOrEqual);

		if conf.cull_face {
			gl.enable(gl::Capability::CullFace);
			gl.cull_face(gl::Face::Back);
			gl.front_face(gl::CullMode::CounterClockwise);
		}

		gl.clear_color(0.0, 0.0, 0.0, 1.0);
		gl.clear(gl::Surface::Color);
		gl.clear(gl::Surface::Depth);
		gl.clear(gl::Surface::Stencil);

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

		let pipeline = gl::Pipeline::new(&gl, &vert_src, &frag_src)?;

		// TODO: don't clone here
		let font_data = conf.default_font
			.clone()
			.take()
			.unwrap_or(res::font::UNSCII);

		let font = gfx::BitmapFont::from_data(&gl, font_data)?;

		return Ok(Self {

			width: window.width(),
			height: window.height(),
			dpi: window.dpi(),

			renderer: gl::BatchedMesh::<Vertex, Uniform>::new(&gl, DRAW_COUNT, DRAW_COUNT)?,

			view: cam.view(),
			proj: cam.proj(),
			transform: mat4!(),

			default_pipeline: pipeline.clone(),
			cur_pipeline: pipeline,
			cur_custom_uniform: None,

			cur_canvas: None,

			draw_calls_last: 0,
			draw_calls: 0,

			empty_tex: gfx::Texture::from_pixels(&gl, 1, 1, &[255; 4])?,

			default_font: font,

			gl: Rc::new(gl),

		});

	}

	pub fn clear(&mut self) {

		self.flush();
		self.gl.clear(Surface::Color);
		self.gl.clear(Surface::Depth);
		self.gl.clear(Surface::Stencil);

	}

	pub fn clear_ex(&mut self, s: Surface) {

		self.flush();
		self.gl.clear(s);

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

		self.cur_canvas = Some(canvas.clone());
		self.transform = mat4!();

		self.gl.viewport(
			0,
			0,
			(cw as f32 * self.dpi) as i32,
			(ch as f32 * self.dpi) as i32,
		);

		canvas.gl_fbuf().with(|| -> Result<()> {
			f(self)?;
			self.flush();
			return Ok(());
		})?;

		self.cur_canvas = None;
		self.transform = t;

		self.gl.viewport(
			0,
			0,
			(self.width as f32 * self.dpi) as i32,
			(self.height as f32 * self.dpi) as i32,
		);

		return Ok(());

	}

	pub fn draw_with<U: CustomUniform>(&mut self, shader: &Shader<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let uniforms = uniform.values()
			.into_iter()
			.map(|(n, v)| (n, v.into_uniform()))
			.collect::<Vec<(&'static str, gl::UniformValue)>>();

		let prev_pipeline = self.cur_pipeline.clone();
		let prev_uniform = self.cur_custom_uniform.clone();

		self.flush();
		self.cur_pipeline = gl::Pipeline::clone(&shader.gl_pipeline());
		self.cur_custom_uniform = Some(uniforms);
		f(self)?;
		self.flush();
		self.cur_pipeline = prev_pipeline;
		self.cur_custom_uniform = prev_uniform;

		return Ok(());

	}

	pub fn no_depth_test(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		self.flush();
		self.gl.disable(gl::Capability::DepthTest);
		f(self)?;
		self.flush();
		self.gl.enable(gl::Capability::DepthTest);

		return Ok(());

	}

	pub fn no_depth_write(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		self.flush();
		self.gl.depth_mask(false);
		f(self)?;
		self.flush();
		self.gl.depth_mask(true);

		return Ok(());

	}

	pub fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let gl = self.gl.clone();

		self.flush();
		gl.enable(gl::Capability::StencilTest);
		gl.clear(Surface::Stencil);

		gl.stencil(gl::StencilFunc {
			cmp: Cmp::Never,
			rf: 1,
			mask: 0xff,
		}, gl::StencilOps {
			sfail: gl::StencilOp::Replace,
			dpfail: gl::StencilOp::Replace,
			dppass: gl::StencilOp::Replace,
		}, || {
			return mask(self);
		})?;

		self.flush();

		gl.stencil(gl::StencilFunc {
			cmp: Cmp::Equal,
			rf: 1,
			mask: 0xff,
		}, gl::StencilOps {
			sfail: gl::StencilOp::Keep,
			dpfail: gl::StencilOp::Keep,
			dppass: gl::StencilOp::Keep,
		}, || {
			return draw(self);
		})?;

		self.flush();
		gl.disable(gl::Capability::StencilTest);

		return Ok(());

	}

	pub fn use_blend(&mut self, b: Blend, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let default = Blend::Alpha.to_gl();
		let b = b.to_gl();

		self.flush();
		self.gl.blend_func(b.0, b.1);
		f(self)?;
		self.flush();
		self.gl.blend_func(default.0, default.1);

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
	fn to_gl(&self) -> (gl::BlendFac, gl::BlendFac) {
		return match self {
			Blend::Alpha => (gl::BlendFac::SrcAlpha, gl::BlendFac::OneMinusSrcAlpha),
			Blend::Add => (gl::BlendFac::SrcAlpha, gl::BlendFac::DestAlpha),
			Blend::Replace => (gl::BlendFac::SrcAlpha, gl::BlendFac::Zero),
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

