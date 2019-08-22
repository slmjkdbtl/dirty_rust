// wengwengweng

pub mod gfx;
pub mod input;
pub mod window;
pub mod shapes;

use crate::*;
use crate::math::*;

pub use gfx::Gfx;
pub use input::Input;
pub use window::Window;

use std::collections::HashMap;
use std::thread;
use std::time::Instant;
use std::time::Duration;

use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::window::Fullscreen;
use glutin::dpi::*;
use glutin::Api;
use glutin::GlRequest;
use gilrs::Gilrs;

use input::ButtonState;
use input::Key;
use input::Mouse;

use window::Pos;

use gfx::Origin;

type WindowedCtx = glutin::WindowedContext<glutin::PossiblyCurrent>;
type EventLoop = glutin::event_loop::EventLoop<()>;

const MAX_DRAWS: usize = 65536;

include!("../res/resources.rs");

// TODO: make this lighter
/// Manages Ctx
pub struct Ctx {

	pub(self) conf: Conf,

	// lifecycle
	pub(self) quit: bool,
	pub(self) dt: f32,
	pub(self) time: f32,
	pub(self) fps_counter: FPSCounter,

	// input
	pub(self) key_state: HashMap<Key, ButtonState>,
	pub(self) rpressed_key: Option<Key>,
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
	pub(self) width: i32,
	pub(self) height: i32,

	pub(self) windowed_ctx: WindowedCtx,
	pub(self) gamepad_ctx: gilrs::Gilrs,

	// gfx
	pub(self) proj_2d: math::Mat4,
	pub(self) proj_3d: math::Mat4,
	pub(self) cam_3d: gfx::Camera,

	pub(self) gl: gl::Device,
	pub(self) quad_renderer: gl::BatchedRenderer<gfx::QuadShape>,
	pub(self) cube_renderer: gl::Renderer<gfx::Vertex3D>,

	pub(self) empty_tex: gfx::Tex2D,

	pub(self) default_shader_2d: gfx::Shader,
	pub(self) cur_shader_2d: gfx::Shader,

	pub(self) default_shader_3d: gfx::Shader,
	pub(self) cur_shader_3d: gfx::Shader,

	pub(self) default_font: gfx::Font,

	pub(self) draw_calls_last: usize,
	pub(self) draw_calls: usize,

	pub(self) transform: Mat4,
	pub(self) transform_stack: Vec<Mat4>,

}

impl Ctx {

	pub(super) fn new(conf: app::Conf) -> Result<(Self, EventLoop)> {

		let event_loop = EventLoop::new();

		let mut window_builder = WindowBuilder::new()
			.with_title(conf.title.to_owned())
			.with_resizable(conf.resizable)
			.with_transparent(conf.transparent)
			.with_decorations(!conf.borderless)
			.with_always_on_top(conf.always_on_top)
			.with_inner_size(LogicalSize::new(conf.width as f64, conf.height as f64))
			;

		if conf.fullscreen {
			window_builder = window_builder
				.with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())));
		}

		#[cfg(target_os = "macos")] {

			use glutin::platform::macos::WindowBuilderExtMacOS;

			window_builder = window_builder
				.with_titlebar_buttons_hidden(conf.hide_titlebar_buttons)
				.with_title_hidden(conf.hide_title)
				.with_titlebar_transparent(conf.titlebar_transparent)
				.with_fullsize_content_view(conf.fullsize_content)
				.with_disallow_hidpi(!conf.hidpi)
				;

		}

		let mut ctx_builder = glutin::ContextBuilder::new()
			.with_vsync(conf.vsync)
			.with_gl(GlRequest::Specific(Api::OpenGl, (2, 1)))
			;

		#[cfg(feature = "gl3")] {
			ctx_builder = ctx_builder
				.with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
				.with_gl_profile(glutin::GlProfile::Core)
				;
		}

		let windowed_ctx = unsafe {
			ctx_builder.build_windowed(window_builder, &event_loop)?.make_current()?
		};

		let gl = gl::Device::from_loader(|s| {
			windowed_ctx.get_proc_address(s) as *const _
		});

		gl.enable(gl::Capability::Blend);
		gl.enable(gl::Capability::DepthTest);
// 		gl.enable(gl::Capability::CullFace);
// 		gl.cull_face(gl::Face::Back);
		gl.blend_func(gl::BlendFac::SrcAlpha, gl::BlendFac::OneMinusSrcAlpha);
		gl.depth_func(gl::Cmp::LessOrEqual);
		gl.clear_color(color!(0, 0, 0, 1));

		let empty_tex = gl::Texture::init(&gl, 1, 1, &[255; 4])?;
		let empty_tex = gfx::Tex2D::from_handle(empty_tex, 1, 1);

		let vert_2d_src = TEMPLATE_2D_VERT.replace("###REPLACE###", DEFAULT_2D_VERT);
		let frag_2d_src = TEMPLATE_2D_FRAG.replace("###REPLACE###", DEFAULT_2D_FRAG);

		let shader_2d = gfx::Shader::from_handle(gl::Program::new(&gl, &vert_2d_src, &frag_2d_src)?);
		let proj_2d = conf.origin.to_ortho(conf.width, conf.height);

		shader_2d.send("proj", proj_2d.clone());

		let vert_3d_src = TEMPLATE_3D_VERT.replace("###REPLACE###", DEFAULT_3D_VERT);
		let frag_3d_src = TEMPLATE_3D_FRAG.replace("###REPLACE###", DEFAULT_3D_FRAG);

		let shader_3d = gfx::Shader::from_handle(gl::Program::new(&gl, &vert_3d_src, &frag_3d_src)?);
		let proj_3d = math::perspective(60f32.to_radians(), conf.width as f32 / conf.height as f32, 0.1, 1024.0);
		let cam_3d = gfx::Camera::new(vec3!(), 0.0, 0.0);

		shader_3d.send("proj", proj_3d.clone());
		shader_3d.send("view", cam_3d.get_lookat_matrix());

		let font_img = img::Image::from_bytes(DEFAULT_FONT_IMG)?;
		let font_width = font_img.width();
		let font_height = font_img.height();
		let font_tex = gl::Texture::init(&gl, font_width, font_height, &font_img.into_raw())?;
		let font_tex = gfx::Tex2D::from_handle(font_tex, font_width, font_height);

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
			fps_counter: FPSCounter::new(),

			key_state: HashMap::new(),
			rpressed_key: None,
			mouse_state: HashMap::new(),
			mouse_pos: Pos::new(0, 0),
			mouse_delta: None,
			scroll_delta: None,
			text_input: None,

			title: conf.title.to_owned(),
			width: conf.width,
			height: conf.height,
			fullscreen: conf.fullscreen,
			cursor_hidden: conf.cursor_hidden,
			cursor_locked: conf.cursor_locked,

			windowed_ctx: windowed_ctx,
			gamepad_ctx: Gilrs::new()?,

			quad_renderer: gl::BatchedRenderer::<gfx::QuadShape>::new(&gl, MAX_DRAWS)?,
			cube_renderer: gl::Renderer::from_shape(&gl, gfx::CubeShape)?,
			gl: gl,

			proj_2d: proj_2d,
			proj_3d: proj_3d,
			cam_3d: cam_3d,

			empty_tex: empty_tex,

			default_shader_2d: shader_2d.clone(),
			cur_shader_2d: shader_2d,

			default_shader_3d: shader_3d.clone(),
			cur_shader_3d: shader_3d,

			default_font: font,
			draw_calls: 0,
			draw_calls_last: 0,

			transform: Mat4::identity(),
			transform_stack: Vec::with_capacity(4),

			conf: conf,

		};

		if ctx.conf.cursor_hidden {
// 			ctx.set_cursor_hidden(true);
		}

		if ctx.conf.cursor_locked {
// 			ctx.set_cursor_locked(true)?;
		}

		ctx.clear();
		window::swap(&ctx)?;

		return Ok((ctx, event_loop));

	}

	pub(super) fn run(mut self, event_loop: EventLoop, mut f: impl FnMut(&mut Self) -> Result<()> + 'static) -> ! {

		event_loop.run(move |event, target, control_flow| {

			let ctx = &mut self;
			let start_time = Instant::now();

			input::poll(ctx, event);
			gfx::begin(ctx);
			f(ctx);
			gfx::end(ctx);
			window::swap(ctx);

			if ctx.quit {
				*control_flow = ControlFlow::Exit;
			}

			if let Some(fps_cap) = ctx.conf.fps_cap {

				let real_dt = start_time.elapsed().as_millis();
				let expected_dt = (1000.0 / fps_cap as f32) as u128;

				if real_dt < expected_dt {
					thread::sleep(Duration::from_millis((expected_dt - real_dt) as u64));
				}

			}

			ctx.dt = start_time.elapsed().as_millis() as f32 / 1000.0;
			ctx.time += ctx.dt;
			ctx.fps_counter.tick(ctx.dt);

		});

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

pub fn run<S: State + 'static>() -> Result<()> {
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

	pub fn run<S: State + 'static>(self) -> Result<()> {

		let (mut ctx, event_loop) = Ctx::new(self.conf)?;
		let mut s = S::init(&mut ctx)?;

		ctx.run(event_loop, move |c| {
			return s.run(c);
		});

	}

	pub fn conf(mut self, c: Conf) -> Self {
		self.conf = c;
		return self;
	}

	pub fn size(mut self, w: i32, h: i32) -> Self {
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

	pub fn fps_cap(mut self, f: Option<u16>) -> Self {
		self.conf.fps_cap = f;
		return self;
	}

	pub fn origin(mut self, o: Origin) -> Self {
		self.conf.origin = o;
		return self;
	}

	pub fn quad_origin(mut self, o: Origin) -> Self {
		self.conf.quad_origin = o;
		return self;
	}

	pub fn texture_filter(mut self, f: gfx::FilterMode) -> Self {
		self.conf.texture_filter = f;
		return self;
	}

}

#[derive(Clone, Debug)]
pub struct Conf {
	pub width: i32,
	pub height: i32,
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
	pub fps_cap: Option<u16>,
	pub origin: Origin,
	pub quad_origin: Origin,
	pub texture_filter: gfx::FilterMode,
}

impl Conf {

	pub fn basic(title: &str, width: i32, height: i32) -> Self {
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
			fps_cap: Some(60),
			origin: Origin::Center,
			quad_origin: Origin::Center,
			texture_filter: gfx::FilterMode::Nearest,
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Platform {
	Mobile,
	Desktop,
	Web,
	Unknown,
}

pub fn platform() -> Platform {

	#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
	return Platform::Desktop;
	#[cfg(any(target_os = "ios", target_os = "android"))]
	return Platform::Mobile;
	#[cfg(target_arch = "wasm32")]
	return Platform::Web;

	return Platform::Unknown;

}

