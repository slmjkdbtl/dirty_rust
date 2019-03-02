// wengwengweng

//! General Rendering

use std::rc::Rc;

use gctx::ctx;

use crate::*;
use crate::math::*;
use crate::ggl;

include!("../res/resources.rs");

// context
ctx!(GFX: GfxCtx);

struct GfxCtx {
	current_canvas: Option<Canvas>,
}

pub(super) fn init() {

	ctx_init(GfxCtx {
		current_canvas: None,
	});

	g3d::init();
	g2d::init();
	ggl::set_blend(ggl::BlendFac::SourceAlpha, ggl::BlendFac::OneMinusSourceAlpha);
	ggl::set_depth(ggl::DepthFunc::LessOrEqual);
	ggl::clear_color(0.0, 0.0, 0.0, 1.0);
	clear();
	window::swap();

}

/// check if gfx is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

pub(crate) fn current_canvas() -> &'static Option<Canvas> {
	return &ctx_get().current_canvas;
}

/// set active canvas
pub fn drawon(c: &Canvas) {

	let gfx_mut = ctx_mut();

	gfx_mut.current_canvas = Some(c.clone());
	g2d::flush();
	g2d::flip_projection();
	ggl::set_framebuffer(&*c.handle);

}

/// stop active canvas
pub fn stop_drawon() {

	let gfx_mut = ctx_mut();

	gfx_mut.current_canvas = None;
	g2d::flush();
	g2d::unflip_projection();
	ggl::clear_framebuffer();

}

/// clear view
pub fn clear() {
	ggl::clear(true, true, false);
}

/// save a canvas into a png file
pub fn capture(canvas: &Canvas, fname: &str) {

	let tex = &canvas.tex;
	let buffer = tex.handle.get_data();

	image::save_buffer(
		fname,
		&buffer,
		tex.width(),
		tex.height(),
		image::ColorType::RGBA(8),
	).expect("failed to save png");

}

pub(super) fn begin() {

	clear();
	g2d::begin();
	g3d::begin();

}

pub(super) fn end() {

	let gfx = ctx_get();

	g2d::end();
	g3d::end();
	ggl::clear_framebuffer();

}

/// texture
#[derive(PartialEq, Clone)]
pub struct Texture {
	pub(super) handle: Rc<ggl::Texture>,
}

impl Texture {

	/// create an empty texture with width and height
	pub fn new(width: u32, height: u32) -> Self {
		return Self {
			handle: Rc::new(ggl::Texture::new(width, height)),
		};
	}

	/// create texture with raw data
	pub fn from_bytes(data: &[u8]) -> Self {

		let img = image::load_from_memory(data)
			.expect("failed to load image")
			.to_rgba();

		let width = img.width();
		let height = img.height();
		let pixels = img.into_raw();

		return Self::from_raw(&pixels, width, height);

	}

	/// create texture from pixel data, width and height
	pub fn from_raw(pixels: &[u8], width: u32, height: u32) -> Self {

		let tex = Self::new(width, height);

		tex.handle.data(pixels);

		return tex;

	}

	/// create texture from a file
	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

	/// create texture from a color block
	pub fn from_color(c: Color, width: u32, height: u32) -> Self {
		return Self::from_raw(&c.to_rgba(), width, height);
	}

	/// get texture width
	pub fn width(&self) -> u32 {
		return self.handle.width;
	}

	/// get texture height
	pub fn height(&self) -> u32 {
		return self.handle.height;
	}

}

/// mesh
#[derive(Clone)]
pub struct Mesh {
	vbuf: Rc<ggl::VertexBuffer>,
	ibuf: Rc<ggl::IndexBuffer>,
	texture: Rc<ggl::Texture>,
}

impl Mesh {
	pub fn new(vertices: Vec<f32>, indices: Vec<f32>) -> Self {
		unimplemented!();
	}
	pub fn from_obj() -> Self {
		unimplemented!();
	}
}

/// offscreen framebuffer
#[derive(PartialEq, Clone)]
pub struct Canvas {

	pub(super) handle: Rc<ggl::Framebuffer>,
	pub tex: Texture,
	pub width: u32,
	pub height: u32,

}

impl Canvas {

	/// create new canvas
	pub fn new(width: u32, height: u32) -> Self {

		let handle = ggl::Framebuffer::new();
		let pixels = vec![0.0 as u8; (width * height * 4) as usize];
		let tex = Texture::from_raw(&pixels, width, height);

		handle.attach(&*tex.handle);

		return Self {
			handle: Rc::new(handle),
			tex: tex,
			width: width,
			height: height,
		}

	}

}

macro_rules! gen_templated_shader {

	($name:ident, $vert_template:expr, $frag_template:expr, $vert_default:expr, $frag_default:expr) => {

		use std::rc::Rc;

		/// shader effect
		#[derive(PartialEq, Clone)]
		pub struct $name {
			program: Rc<ggl::Program>,
		}

		impl Shader {

			pub fn from_code(vert: &str, frag: &str) -> Self {

				let vert = $vert_template.replace("###REPLACE###", vert);
				let frag = $frag_template.replace("###REPLACE###", frag);
				let program = ggl::Program::new(&vert, &frag);

				return Self {
					program: Rc::new(program),
				};

			}

			pub fn from_code_vert(vert: &str) -> Self {
				return Self::from_code(vert, $frag_default);
			}

			pub fn from_code_frag(frag: &str) -> Self {
				return Self::from_code($vert_default, frag);
			}

			pub fn from_file(vertf: &str, fragf: &str) -> Self {
				return Self::from_code(&fs::read_str(vertf), &fs::read_str(fragf));
			}

			pub fn from_file_vert(vertf: &str) -> Self {
				return Self::from_code(&fs::read_str(vertf), $frag_default);
			}

			pub fn from_file_frag(fragf: &str) -> Self {
				return Self::from_code($vert_default, &fs::read_str(fragf));
			}

			pub fn send_float(&self, name: &str, f: f32) -> &Self {
				self.program.uniform_f1(name, f);
				return self;
			}

			pub fn send_vec2(&self, name: &str, v: Vec2) -> &Self {
				self.program.uniform_f2(name, v.x, v.y);
				return self;
			}

			pub fn send_vec3(&self, name: &str, v: Vec3) -> &Self {
				self.program.uniform_f3(name, v.x, v.y, v.z);
				return self;
			}

			pub fn send_vec4(&self, name: &str, v: Vec4) -> &Self {
				self.program.uniform_f4(name, v.x, v.y, v.z, v.w);
				return self;
			}

			pub fn send_mat4(&self, name: &str, v: Mat4) -> &Self {
				self.program.uniform_mat4(name, v.as_arr());
				return self;
			}

			pub fn send_color(&self, name: &str, c: Color) -> &Self {
				self.program.uniform_f4(name, c.r, c.g, c.b, c.a);
				return self;
			}

			pub fn send_rect(&self, name: &str, r: Rect) -> &Self {
				self.program.uniform_f4(name, r.x, r.y, r.w, r.h);
				return self;
			}

		}

	}

}

