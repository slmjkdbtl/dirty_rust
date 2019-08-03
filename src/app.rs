// wengwengweng

mod gl;
pub mod gfx;
pub mod input;
pub mod window;
pub mod shapes;

use crate::*;
use crate::math::*;

pub use gfx::Gfx;
pub use input::Input;
pub use window::Window;

use std::rc::Rc;
use std::collections::HashMap;
use std::thread;
use std::time::Instant;
use std::time::Duration;

use glutin::dpi::*;
use glutin::Api;
use glutin::GlRequest;
use glutin::GlProfile;
use gilrs::Gilrs;

use input::ButtonState;
use input::Key;
use input::Mouse;

use window::Pos;

use gfx::Origin;

const MAX_DRAWS: usize = 65536;

#[cfg(feature="gl3")]
const TEMPLATE_2D_VERT: &str = include_str!("res/2d_template_330.vert");
#[cfg(feature="gl3")]
const TEMPLATE_2D_FRAG: &str = include_str!("res/2d_template_330.frag");

#[cfg(feature="gl3")]
const DEFAULT_2D_VERT: &str = include_str!("res/2d_default_330.vert");
#[cfg(feature="gl3")]
const DEFAULT_2D_FRAG: &str = include_str!("res/2d_default_330.frag");

#[cfg(not(feature="gl3"))]
const TEMPLATE_2D_VERT: &str = include_str!("res/2d_template.vert");
#[cfg(not(feature="gl3"))]
const TEMPLATE_2D_FRAG: &str = include_str!("res/2d_template.frag");

#[cfg(not(feature="gl3"))]
const DEFAULT_2D_VERT: &str = include_str!("res/2d_default.vert");
#[cfg(not(feature="gl3"))]
const DEFAULT_2D_FRAG: &str = include_str!("res/2d_default.frag");

const DEFAULT_3D_VERT: &str = include_str!("res/3d.vert");
const DEFAULT_3D_FRAG: &str = include_str!("res/3d.frag");

const DEFAULT_FONT_IMG: &[u8] = include_bytes!("res/CP437.png");
const DEFAULT_FONT_COLS: usize = 32;
const DEFAULT_FONT_ROWS: usize = 8;
const DEFAULT_FONT_CHARS: &str = r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##;

/// Manages Ctx
pub struct Ctx {

	// lifecycle
	pub(self) quit: bool,
	pub(self) dt: f32,
	pub(self) time: f32,
	pub(self) fps_cap: Option<u16>,
	pub(self) fps_counter: FPSCounter,

	// input
	pub(self) key_state: HashMap<Key, ButtonState>,
	pub(self) mouse_state: HashMap<Mouse, ButtonState>,
	pub(self) mouse_pos: Pos,
	pub(self) mouse_delta: Option<Pos>,
	pub(self) scroll_delta: Option<Pos>,
	pub(self) text_input: Option<String>,

	// window
	pub(self) title: String,
	pub(self) fullscreen: bool,
	pub(self) cursor_hidden: bool,
	pub(self) cursor_locked: bool,
	pub(self) width: u32,
	pub(self) height: u32,

	pub(self) windowed_ctx: glutin::WindowedContext<glutin::PossiblyCurrent>,
	pub(self) events_loop: glutin::EventsLoop,
	pub(self) gamepad_ctx: gilrs::Gilrs,

	// gfx
	pub(self) origin: gfx::Origin,
	pub(self) texture_origin: gfx::Origin,

	pub(self) proj_2d: math::Mat4,
	pub(self) proj_3d: math::Mat4,

	pub(self) gl: gl::Device,
	pub(self) batched_renderer: gl::BatchedRenderer<gfx::QuadShape>,

	pub(self) cur_tex: Option<gfx::Texture>,
	pub(self) empty_tex: gfx::Texture,

	pub(self) default_shader_2d: gfx::Shader,
	pub(self) cur_shader_2d: gfx::Shader,

	pub(self) default_shader_3d: gfx::Shader,
	pub(self) cur_shader_3d: gfx::Shader,

	pub(self) default_font: gfx::Font,

	pub(self) draw_calls_last: usize,
	pub(self) draw_calls: usize,

	pub(self) state: gfx::State,
	pub(self) state_stack: Vec<gfx::State>,

}

unsafe impl Send for Ctx {}

impl Ctx {

	pub(super) fn new(conf: app::Conf) -> Result<Self> {

		let events_loop = glutin::EventsLoop::new();

		let mut window_builder = glutin::WindowBuilder::new()
			.with_title(conf.title.to_owned())
			.with_resizable(conf.resizable)
			.with_transparency(conf.transparent)
			.with_decorations(!conf.borderless)
			.with_always_on_top(conf.always_on_top)
			.with_dimensions(LogicalSize::new(conf.width as f64, conf.height as f64))
			.with_multitouch();

		if conf.fullscreen {
			window_builder = window_builder
				.with_fullscreen(Some(events_loop.get_primary_monitor()));
		}

		#[cfg(target_os = "macos")] {

			use glutin::os::macos::WindowBuilderExt;

			window_builder = window_builder
				.with_titlebar_buttons_hidden(conf.hide_titlebar_buttons)
				.with_title_hidden(conf.hide_title)
				.with_titlebar_transparent(conf.titlebar_transparent)
				.with_fullsize_content_view(conf.fullsize_content);
// 				.with_disallow_hidpi(!conf.hidpi);

		}

		let mut ctx_builder = glutin::ContextBuilder::new()
			.with_vsync(conf.vsync)
			.with_gl(GlRequest::Specific(Api::OpenGl, (2, 1)));

		#[cfg(feature = "gl3")] {
			ctx_builder = ctx_builder
				.with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
				.with_gl_profile(glutin::GlProfile::Core);
		}

		let windowed_ctx = unsafe {
			ctx_builder.build_windowed(window_builder, &events_loop)?.make_current()?
		};

		let gl = gl::Device::from_loader(|s| {
			windowed_ctx.get_proc_address(s) as *const _
		});

		gl.enable(gl::Capability::Blend);
		gl.blend_func_sep(gl::BlendFac::SourceAlpha, gl::BlendFac::OneMinusSourceAlpha, gl::BlendFac::One, gl::BlendFac::OneMinusSourceAlpha);
		gl.clear_color(conf.clear_color);
		gl.clear();

		let batched_renderer = gl::BatchedRenderer::<gfx::QuadShape>::new(&gl, MAX_DRAWS)?;

		let empty_tex = gl::Texture::new(&gl, 1, 1)?;
		empty_tex.data(&[255, 255, 255, 255]);
		let empty_tex = gfx::Texture::from_handle(empty_tex);

		let vert_2d_src = TEMPLATE_2D_VERT.replace("###REPLACE###", DEFAULT_2D_VERT);
		let frag_2d_src = TEMPLATE_2D_FRAG.replace("###REPLACE###", DEFAULT_2D_FRAG);

		let shader_2d = gfx::Shader::from_handle(gl::Program::new(&gl, &vert_2d_src, &frag_2d_src)?);
		let proj_2d = conf.origin.to_ortho(conf.width, conf.height);

		shader_2d.send("proj", proj_2d.clone());

		let shader_3d = gfx::Shader::from_handle(gl::Program::new(&gl, DEFAULT_3D_VERT, DEFAULT_3D_FRAG)?);
		let proj_3d = conf.origin.to_ortho(conf.width, conf.height);

		shader_3d.send("proj", proj_3d.clone());

		let font_img = img::Image::from_bytes(DEFAULT_FONT_IMG)?;
		let font_tex = gl::Texture::new(&gl, font_img.width() as i32, font_img.height() as i32)?;
		font_tex.data(&font_img.into_raw());
		let font_tex = gfx::Texture::from_handle(font_tex);

		let font = gfx::Font::from_tex(
			font_tex,
			DEFAULT_FONT_COLS,
			DEFAULT_FONT_ROWS,
			DEFAULT_FONT_CHARS,
		)?;

		let mut ctx = Self {

			quit: false,
			dt: 0.0,
			time: 0.0,
			fps_cap: conf.fps_cap,
			fps_counter: FPSCounter::new(),

			key_state: HashMap::new(),
			mouse_state: HashMap::new(),
			mouse_pos: Pos::new(0, 0),
			mouse_delta: None,
			scroll_delta: None,
			text_input: None,
			fullscreen: conf.fullscreen,
			cursor_hidden: conf.cursor_hidden,
			cursor_locked: conf.cursor_locked,
			title: conf.title.to_owned(),
			width: conf.width,
			height: conf.height,

			events_loop: events_loop,
			windowed_ctx: windowed_ctx,
			gamepad_ctx: Gilrs::new()?,

			gl: gl,
			origin: conf.origin,
			texture_origin: conf.texture_origin,
			batched_renderer: batched_renderer,

			proj_2d: proj_2d,
			proj_3d: proj_3d,

			cur_tex: None,
			empty_tex: empty_tex,

			default_shader_2d: shader_2d.clone(),
			cur_shader_2d: shader_2d,

			default_shader_3d: shader_3d.clone(),
			cur_shader_3d: shader_3d,

			default_font: font,
			draw_calls: 0,
			draw_calls_last: 0,
			state: gfx::State::default(),
			state_stack: Vec::with_capacity(16),

		};

		if conf.cursor_hidden {
			ctx.set_cursor_hidden(true);
		}

		if conf.cursor_locked {
			ctx.set_cursor_locked(true)?;
		}

		window::swap(&ctx)?;

		return Ok(ctx);

	}

	pub(super) fn run(&mut self, mut f: impl FnMut(&mut Self) -> Result<()>) -> Result<()> {

		'run: loop {

			let start_time = Instant::now();

			input::poll(self)?;

			gfx::begin(self);
			f(self)?;
			gfx::end(self);
			window::swap(self)?;

			if self.quit {
				break 'run;
			}

			if let Some(fps_cap) = self.fps_cap {

				let real_dt = start_time.elapsed().as_millis();
				let expected_dt = (1000.0 / fps_cap as f32) as u128;

				if real_dt < expected_dt {
					thread::sleep(Duration::from_millis((expected_dt - real_dt) as u64));
				}

			}

			self.dt = start_time.elapsed().as_millis() as f32 / 1000.0;
			self.time += self.dt;
			self.fps_counter.tick(self.dt);

		}

		return Ok(());

	}

}

pub trait App {
	fn quit(&mut self);
	fn dt(&self) -> f32;
	fn fps(&self) -> u16;
	fn time(&self) -> f32;
}

impl App for Ctx {

	fn quit(&mut self) {
		self.quit = true;
	}

	fn dt(&self) -> f32 {
		return self.dt;
	}

	fn fps(&self) -> u16 {
		return self.fps_counter.fps();
	}

	fn time(&self) -> f32 {
		return self.time;
	}

}

pub fn run<S: State>() -> Result<()> {
	return launcher().run::<S>();
}

pub fn launcher() -> Launcher {
	return Launcher::default();
}

#[derive(Default)]
pub struct Launcher {
	conf: Conf,
}

impl Launcher {

	pub fn run<S: State>(self) -> Result<()> {

		let mut ctx = Ctx::new(self.conf)?;
		let mut s = S::init(&mut ctx)?;

		return ctx.run(|c| {
			return s.run(c);
		});

	}

	pub fn conf(mut self, c: Conf) -> Self {
		self.conf = c;
		return self;
	}

	pub fn size(mut self, w: u32, h: u32) -> Self {
		self.conf.width = w;
		self.conf.height = h;
		return self;
	}

	pub fn title(mut self, t: &str) -> Self {
		self.conf.title = t.to_owned();
		return self;
	}

	pub fn hidpi(mut self, b: bool) -> Self {
		self.conf.hidpi = b;
		return self;
	}

	pub fn resizable(mut self, b: bool) -> Self {
		self.conf.resizable = b;
		return self;
	}

	pub fn fullscreen(mut self, b: bool) -> Self {
		self.conf.fullscreen = b;
		return self;
	}

	pub fn vsync(mut self, b: bool) -> Self {
		self.conf.vsync = b;
		return self;
	}

	pub fn cursor_hidden(mut self, b: bool) -> Self {
		self.conf.cursor_hidden = b;
		return self;
	}

	pub fn cursor_locked(mut self, b: bool) -> Self {
		self.conf.cursor_locked = b;
		return self;
	}

	pub fn hide_title(mut self, b: bool) -> Self {
		self.conf.hide_title = b;
		return self;
	}

	pub fn hide_titlebar_buttons(mut self, b: bool) -> Self {
		self.conf.hide_titlebar_buttons = b;
		return self;
	}

	pub fn transparent(mut self, b: bool) -> Self {
		self.conf.transparent = b;
		return self;
	}

	pub fn always_on_top(mut self, b: bool) -> Self {
		self.conf.always_on_top = b;
		return self;
	}

	pub fn clear_color(mut self, c: Color) -> Self {
		self.conf.clear_color = c;
		return self;
	}

	pub fn origin(mut self, o: Origin) -> Self {
		self.conf.origin = o;
		return self;
	}

	pub fn texture_origin(mut self, o: Origin) -> Self {
		self.conf.texture_origin = o;
		return self;
	}

	pub fn fps_cap(mut self, f: Option<u16>) -> Self {
		self.conf.fps_cap = f;
		return self;
	}

}

#[derive(Clone, Debug)]
pub struct Conf {
	pub width: u32,
	pub height: u32,
	pub title: String,
	pub hidpi: bool,
	pub resizable: bool,
	pub fullscreen: bool,
	pub always_on_top: bool,
	pub borderless: bool,
	pub transparent: bool,
	pub vsync: bool,
	pub hide_title: bool,
	pub hide_titlebar_buttons: bool,
	pub fullsize_content: bool,
	pub titlebar_transparent: bool,
	pub cursor_hidden: bool,
	pub cursor_locked: bool,
	pub clear_color: Color,
	pub origin: Origin,
	pub texture_origin: Origin,
	pub fps_cap: Option<u16>,
}

impl Conf {

	pub fn basic(title: &str, width: u32, height: u32) -> Self {
		return Self {
			title: String::from(title),
			width: width,
			height: height,
			..Default::default()
		};
	}

}

impl Default for Conf {

	fn default() -> Self {
		return Self {
			width: 640,
			height: 480,
			title: String::new(),
			hidpi: true,
			resizable: false,
			fullscreen: false,
			always_on_top: false,
			borderless: false,
			transparent: false,
			vsync: true,
			fullsize_content: false,
			hide_title: false,
			hide_titlebar_buttons: false,
			titlebar_transparent: false,
			cursor_hidden: false,
			cursor_locked: false,
			clear_color: color!(0, 0, 0, 1),
			origin: Origin::Center,
			texture_origin: Origin::Center,
			fps_cap: Some(60),
		};
	}

}

pub trait State {

	fn init(_: &mut Ctx) -> Result<Self> where Self: Sized;

	fn run(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

	fn quit(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

}

pub(super) struct FPSCounter {
	frames: usize,
	timer: f32,
	fps: u16,
}

impl FPSCounter {

	fn new() -> Self {
		return Self {
			frames: 0,
			timer: 0.0,
			fps: 0,
		}
	}

	fn tick(&mut self, dt: f32) {

		self.frames += 1;
		self.timer += dt;

		if self.timer >= 1.0 {
			self.fps = self.frames as u16;
			self.timer = 0.0;
			self.frames = 0;
		}

	}

	fn fps(&self) -> u16 {
		return self.fps;
	}

}

