// wengwengweng

pub mod gfx;
pub mod input;
pub mod window;
pub mod shapes;

use crate::*;
use crate::math::*;

pub use gfx::Gfx;
pub use window::Window;
pub use input::Input;

#[cfg(feature = "imgui")]
pub use imgui_lib as imgui;

use std::collections::HashMap;
use std::thread;
use std::time::Instant;
use std::time::Duration;

#[cfg(not(web))]
use glutin::dpi::*;
#[cfg(not(web))]
use glutin::Api;
#[cfg(not(web))]
use glutin::GlRequest;
#[cfg(web)]
use glow::RenderLoop;

use derive_more::*;

use input::ButtonState;
use input::Key;
use input::Mouse;
use input::GamepadID;
use input::GamepadButton;

// TODO: make this lighter
/// Manages Ctx
pub struct Ctx {

	pub(self) conf: Conf,

	// lifecycle
	pub(self) quit: bool,
	pub(self) dt: Time,
	pub(self) time: Time,
	pub(self) fps_counter: FPSCounter,

	// input
	pub(self) key_states: HashMap<Key, ButtonState>,
	pub(self) mouse_states: HashMap<Mouse, ButtonState>,
	pub(self) mouse_pos: Vec2,
	pub(self) gamepad_button_states: HashMap<GamepadID, HashMap<GamepadButton, ButtonState>>,
	pub(self) gamepad_axis_pos: HashMap<GamepadID, (Vec2, Vec2)>,

	// window
	pub(self) title: String,
	pub(self) cursor_hidden: bool,
	pub(self) cursor_locked: bool,
	pub(self) width: i32,
	pub(self) height: i32,

	#[cfg(not(web))]
	pub(self) windowed_ctx: glutin::WindowedContext<glutin::PossiblyCurrent>,
	#[cfg(web)]
	pub(self) render_loop: glow::web::RenderLoop,
	#[cfg(all(not(target_os = "ios"), not(target_os = "android"), not(web)))]
	pub(self) gamepad_ctx: gilrs::Gilrs,

	// gfx
	pub(self) gl: gl::Device,

// 	pub(self) backbuffer: gfx::Canvas,

	pub(self) proj_2d: math::Mat4,
	pub(self) proj_3d: math::Mat4,
	pub(self) view_3d: math::Mat4,

	pub(self) renderer_2d: gl::BatchedMesh<gfx::Vertex2D, gfx::Uniform2D>,
	pub(self) cube_renderer: gl::Mesh<gfx::Vertex3D, gfx::Uniform3D>,
	pub(self) renderer_3d: gl::BatchedMesh<gfx::Vertex3D, gfx::Uniform3D>,

	pub(self) empty_tex: gfx::Texture,

	pub(self) default_pipeline_2d: gl::Pipeline<gfx::Vertex2D, gfx::Uniform2D>,
	pub(self) cur_pipeline_2d: gl::Pipeline<gfx::Vertex2D, gfx::Uniform2D>,
	pub(self) cur_custom_uniform_2d: Option<gfx::UniformValues>,

	pub(self) default_pipeline_3d: gl::Pipeline<gfx::Vertex3D, gfx::Uniform3D>,
	pub(self) cur_pipeline_3d: gl::Pipeline<gfx::Vertex3D, gfx::Uniform3D>,
	pub(self) cur_custom_uniform_3d: Option<gfx::UniformValues>,

	pub(self) cur_canvas: Option<gfx::Canvas>,

	pub(self) default_font: gfx::BitmapFont,

	pub(self) draw_calls_last: usize,
	pub(self) draw_calls: usize,

	pub(self) transform: gfx::Transform,

	#[cfg(feature = "imgui")]
	pub(self) imgui_ctx: imgui::Context,
	#[cfg(feature = "imgui")]
	pub(self) imgui_platform: imgui_winit_support::WinitPlatform,
	#[cfg(feature = "imgui")]
	pub(self) imgui_renderer: imgui_opengl_renderer::Renderer,

}

fn run_with_conf<S: State>(conf: Conf) -> Result<()> {

	#[cfg(not(web))]
	let (windowed_ctx, mut events_loop, gl) =  {

		let events_loop = glutin::EventsLoop::new();

		let mut window_builder = glutin::WindowBuilder::new()
			.with_title(conf.title.to_owned())
			.with_resizable(conf.resizable)
			.with_transparency(conf.transparent)
			.with_decorations(!conf.borderless)
			.with_always_on_top(conf.always_on_top)
			.with_dimensions(LogicalSize::new(conf.width as f64, conf.height as f64))
			.with_multitouch()
			;

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
				.with_fullsize_content_view(conf.titlebar_transparent)
// 				.with_disallow_hidpi(!conf.hidpi)
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
			ctx_builder.build_windowed(window_builder, &events_loop)?.make_current()?
		};

		let gl = gl::Device::from_loader(|s| {
			windowed_ctx.get_proc_address(s) as *const _
		});

		(windowed_ctx, events_loop, gl)

	};

	// TODO: wait till glow supports stdweb
	#[cfg(web)]
	let (gl, render_loop) = {

		use stdweb::web;
		use web::IElement;
		use web::INode;
		use web::html_element::CanvasElement;
		use stdweb::unstable::TryInto;

		let document = web::document();

		document.set_title(&conf.title);

		let canvas: CanvasElement = document
			.create_element("canvas")?
			.try_into()
			.map_err(|_| Error::Web(format!("failed to create canvas")))?;

		let body = document
			.body()
			.ok_or(Error::Web(format!("failed to get document body")))?;

		body.append_child(&canvas);
		canvas.set_width(conf.width as u32);
		canvas.set_height(conf.height as u32);

		let gl_ctx = canvas.get_context()?;
		let render_loop = glow::web::RenderLoop::from_request_animation_frame();

		((), render_loop)

	};

	#[cfg(feature = "imgui")]
	let (imgui_ctx, imgui_platform, imgui_renderer) = {

		use imgui_winit_support::{HiDpiMode, WinitPlatform};
		use imgui_opengl_renderer::Renderer;

		let mut ctx = imgui::Context::create();
		let mut platform = WinitPlatform::init(&mut ctx);

		ctx.set_ini_filename(None);
		platform.attach_window(ctx.io_mut(), &windowed_ctx.window(), HiDpiMode::Locked(1.0));

		let renderer = Renderer::new(&mut ctx, |s| windowed_ctx.get_proc_address(s) as *const _);

		(ctx, platform, renderer)

	};

	gl.enable(gl::Capability::Blend);
	gl.enable(gl::Capability::DepthTest);
// 	gl.enable(gl::Capability::CullFace);
// 	gl.cull_face(gl::Face::Back);
	gl.blend_func(gl::BlendFac::SrcAlpha, gl::BlendFac::OneMinusSrcAlpha);
	gl.depth_func(gl::Cmp::LessOrEqual);
	gl.clear_color(conf.clear_color);

	let empty_tex = gl::Texture::from(&gl, 1, 1, &[255; 4])?;
	let empty_tex = gfx::Texture::from_handle(empty_tex, 1, 1);

	let vert_2d_src = res::TEMPLATE_2D_VERT.replace("###REPLACE###", res::DEFAULT_2D_VERT);
	let frag_2d_src = res::TEMPLATE_2D_FRAG.replace("###REPLACE###", res::DEFAULT_2D_FRAG);

	let pipeline_2d = gl::Pipeline::new(&gl, &vert_2d_src, &frag_2d_src)?;
	let proj_2d = conf.origin.to_ortho(conf.width, conf.height);

	let vert_3d_src = res::TEMPLATE_3D_VERT.replace("###REPLACE###", res::DEFAULT_3D_VERT);
	let frag_3d_src = res::TEMPLATE_3D_FRAG.replace("###REPLACE###", res::DEFAULT_3D_FRAG);

	let pipeline_3d = gl::Pipeline::new(&gl, &vert_3d_src, &frag_3d_src)?;

	use gfx::Camera;

	let cam_3d = gfx::PerspectiveCam::new(60.0, conf.width as f32 / conf.height as f32, 0.1, 1024.0, vec3!(), 0.0, 0.0);

	let font_img = img::Image::from_bytes(res::CP437_IMG)?;
	let font_width = font_img.width();
	let font_height = font_img.height();
	let font_tex = gl::Texture::from(&gl, font_width, font_height, &font_img.into_raw())?;
	let font_tex = gfx::Texture::from_handle(font_tex, font_width, font_height);

	let font = gfx::BitmapFont::from_tex(
		font_tex,
		res::CP437_COLS,
		res::CP437_ROWS,
		res::CP437_CHARS,
	)?;

	let mut ctx = Ctx {

		// app
		quit: false,
		dt: Time::new(0.0),
		time: Time::new(0.0),
		fps_counter: FPSCounter::new(),

		// input
		key_states: HashMap::new(),
		mouse_states: HashMap::new(),
		mouse_pos: vec2!(),
		gamepad_button_states: HashMap::new(),
		gamepad_axis_pos: HashMap::new(),

		// window
		title: conf.title.to_owned(),
		width: conf.width,
		height: conf.height,
		cursor_hidden: conf.cursor_hidden,
		cursor_locked: conf.cursor_locked,

		#[cfg(not(web))]
		windowed_ctx: windowed_ctx,
		#[cfg(web)]
		render_loop: render_loop,
		#[cfg(all(not(mobile), not(web)))]
		gamepad_ctx: gilrs::Gilrs::new()?,

// 		backbuffer: gfx::Canvas,

		renderer_2d: gl::BatchedMesh::<gfx::Vertex2D, gfx::Uniform2D>::new(&gl, 65536, 65536)?,
		renderer_3d: gl::BatchedMesh::<gfx::Vertex3D, gfx::Uniform3D>::new(&gl, 65536, 65536)?,
		cube_renderer: gl::Mesh::from_shape(&gl, gfx::CubeShape)?,
		gl: gl,

		proj_2d: proj_2d,
		view_3d: cam_3d.lookat(),
		proj_3d: cam_3d.projection(),

		empty_tex: empty_tex,

		default_pipeline_2d: pipeline_2d.clone(),
		cur_pipeline_2d: pipeline_2d,
		cur_custom_uniform_2d: None,

		default_pipeline_3d: pipeline_3d.clone(),
		cur_pipeline_3d: pipeline_3d,
		cur_custom_uniform_3d: None,

		cur_canvas: None,

		default_font: font,
		draw_calls: 0,
		draw_calls_last: 0,

		transform: gfx::Transform::new(),

		#[cfg(feature = "imgui")]
		imgui_ctx: imgui_ctx,
		#[cfg(feature = "imgui")]
		imgui_platform: imgui_platform,
		#[cfg(feature = "imgui")]
		imgui_renderer: imgui_renderer,

		conf: conf,

	};

	if ctx.conf.cursor_hidden {
		ctx.set_cursor_hidden(true);
	}

	if ctx.conf.cursor_locked {
		ctx.set_cursor_locked(true)?;
	}

	ctx.clear();
	window::swap(&ctx)?;

	let mut s = S::init(&mut ctx)?;

	'run: loop {

		let start_time = Instant::now();

		input::poll(&mut ctx, &mut events_loop, &mut s)?;
		gfx::begin(&mut ctx);
		s.run(&mut ctx)?;
		gfx::end(&mut ctx);

		#[cfg(feature = "imgui")] {

			let io = ctx.imgui_ctx.io_mut();

			ctx.imgui_platform.prepare_frame(io, &ctx.windowed_ctx.window())?;
			io.update_delta_time(start_time);

			let ui = ctx.imgui_ctx.frame();

			s.imgui(&ui)?;

			ctx.imgui_platform.prepare_render(&ui, &ctx.windowed_ctx.window());
			ctx.imgui_renderer.render(ui);

		}

		window::swap(&ctx)?;

		if ctx.quit {
			break 'run;
		}

		if let Some(fps_cap) = ctx.conf.fps_cap {

			let real_dt = start_time.elapsed().as_millis();
			let expected_dt = (1000.0 / fps_cap as f32) as u128;

			if real_dt < expected_dt {
				thread::sleep(Duration::from_millis((expected_dt - real_dt) as u64));
			}

		}

		ctx.dt.set_inner(start_time.elapsed());
		ctx.time += ctx.dt;
		ctx.fps_counter.tick(ctx.dt);

	}

	s.quit(&mut ctx)?;

	return Ok(());

}

#[derive(Copy, Clone, PartialEq, Add, AddAssign, Sub, SubAssign)]
pub struct Time {
	time: Duration,
}

impl Time {
	pub fn new(s: f32) -> Self {
		return Self {
			time: Duration::from_millis((s * 1000.0) as u64),
		};
	}
	pub fn from_millis(m: u64) -> Self {
		return Self {
			time: Duration::from_millis(m),
		};
	}
	fn set(&mut self, s: f32) {
		self.set_inner(Duration::from_millis((s * 1000.0) as u64));
	}
	fn set_inner(&mut self, d: Duration) {
		self.time = d;
	}
	fn as_secs(&self) -> f32 {
		return self.time.as_secs_f32();
	}
}

pub trait App {
	fn quit(&mut self);
	fn dt(&self) -> f32;
	fn time(&self) -> f32;
	fn fps(&self) -> u16;
	fn conf(&self) -> &Conf;
}

impl App for Ctx {

	fn quit(&mut self) {
		self.quit = true;
	}

	fn dt(&self) -> f32 {
		return self.dt.as_secs();
	}

	fn time(&self) -> f32 {
		return self.time.as_secs();
	}

	fn fps(&self) -> u16 {
		return self.fps_counter.fps();
	}

	fn conf(&self) -> &Conf {
		return &self.conf;
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
		return run_with_conf::<S>(self.conf);
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

	pub fn titlebar_transparent(mut self, b: bool) -> Self {
		self.conf.titlebar_transparent = b;
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

	pub fn clear_color(mut self, c: Color) -> Self {
		self.conf.clear_color = c;
		return self;
	}

	pub fn origin(mut self, o: gfx::Origin) -> Self {
		self.conf.origin = o;
		return self;
	}

	pub fn texture_filter(mut self, f: gfx::FilterMode) -> Self {
		self.conf.texture_filter = f;
		return self;
	}

	pub fn scale_mode(mut self, m: gfx::ScaleMode) -> Self {
		self.conf.scale_mode = m;
		return self;
	}

	pub fn scale(mut self, s: u8) -> Self {
		self.conf.scale = s;
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
	pub titlebar_transparent: bool,
	pub cursor_hidden: bool,
	pub cursor_locked: bool,
	pub fps_cap: Option<u16>,
	pub clear_color: Color,
	pub origin: gfx::Origin,
	pub texture_filter: gfx::FilterMode,
	pub scale_mode: gfx::ScaleMode,
	pub scale: u8,
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
			hide_title: false,
			hide_titlebar_buttons: false,
			titlebar_transparent: false,
			cursor_hidden: false,
			cursor_locked: false,
			fps_cap: Some(60),
			clear_color: color!(0, 0, 0, 1),
			origin: gfx::Origin::Center,
			texture_filter: gfx::FilterMode::Nearest,
			scale_mode: gfx::ScaleMode::Letterbox,
			scale: 1,
		};
	}

}

pub trait State: Sized {

	fn init(_: &mut Ctx) -> Result<Self>;

	fn event(&mut self, _: &mut Ctx, _: input::Event) -> Result<()> {
		return Ok(());
	}

	fn run(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

	#[cfg(feature = "imgui")]
	fn imgui(&self, _: &imgui::Ui) -> Result<()> {
		return Ok(());
	}

	fn quit(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

}

pub(super) struct FPSCounter {
	frames: usize,
	timer: Time,
	fps: u16,
}

impl FPSCounter {

	fn new() -> Self {
		return Self {
			frames: 0,
			timer: Time::new(0.0),
			fps: 0,
		}
	}

	fn tick(&mut self, dt: Time) {

		self.frames += 1;
		self.timer += dt;

		if self.timer.as_secs() >= 1.0 {
			self.fps = self.frames as u16;
			self.timer.set(0.0);
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

#[allow(unreachable_code)]
pub fn platform() -> Platform {

	#[cfg(desktop)]
	return Platform::Desktop;
	#[cfg(mobile)]
	return Platform::Mobile;
	#[cfg(web)]
	return Platform::Web;

	return Platform::Unknown;

}
