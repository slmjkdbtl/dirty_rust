// wengwengweng

//! Windowing, Input, and Graphics

use std::rc::Rc;
use std::collections::HashMap;
use std::thread;
use std::time::Instant;
use std::time::Duration;

use glutin::dpi::*;
use glutin::GlRequest;

use crate::*;
use crate::math::*;
pub use state::*;
pub use conf::*;

use gfx::Camera;

use input::ButtonState;
use input::Key;
use input::Mouse;
use input::GamepadID;
use input::GamepadButton;

pub struct Ctx {

	pub(crate) conf: Conf,

	// lifecycle
	pub(crate) quit: bool,
	pub(crate) dt: Duration,
	pub(crate) time: Duration,
	pub(crate) fps_counter: FPSCounter,

	// input
	pub(crate) key_states: HashMap<Key, ButtonState>,
	pub(crate) mouse_states: HashMap<Mouse, ButtonState>,
	pub(crate) mouse_pos: Vec2,
	pub(crate) gamepad_button_states: HashMap<GamepadID, HashMap<GamepadButton, ButtonState>>,
	pub(crate) gamepad_axis_pos: HashMap<GamepadID, (Vec2, Vec2)>,
	pub(crate) scroll_phase: input::ScrollPhase,

	// window
	pub(crate) title: String,
	pub(crate) cursor_hidden: bool,
	pub(crate) cursor_locked: bool,
	pub(crate) width: i32,
	pub(crate) height: i32,

	pub(crate) clipboard_ctx: clipboard::ClipboardContext,

	pub(crate) windowed_ctx: glutin::WindowedContext<glutin::PossiblyCurrent>,
	pub(crate) gamepad_ctx: gilrs::Gilrs,

	// gfx
	pub(crate) gl: Rc<gl::Device>,

	pub(crate) proj: Mat4,
	pub(crate) view: Mat4,

	pub(crate) renderer_2d: gl::BatchedMesh<gfx::Vertex2D, gfx::Uniform2D>,
	pub(crate) cube_renderer: gl::Mesh<gfx::Vertex3D, gfx::Uniform3D>,
	pub(crate) renderer_3d: gl::BatchedMesh<gfx::Vertex3D, gfx::Uniform3D>,
	pub(crate) cubemap_renderer: gl::Mesh<gfx::VertexCubemap, gfx::UniformCubemap>,

	pub(crate) empty_tex: gfx::Texture,

	pub(crate) default_pipeline_2d: gl::Pipeline<gfx::Vertex2D, gfx::Uniform2D>,
	pub(crate) cur_pipeline_2d: gl::Pipeline<gfx::Vertex2D, gfx::Uniform2D>,
	pub(crate) cur_custom_uniform_2d: Option<Vec<(&'static str, gl::UniformValue)>>,

	pub(crate) default_pipeline_3d: gl::Pipeline<gfx::Vertex3D, gfx::Uniform3D>,
	pub(crate) cur_pipeline_3d: gl::Pipeline<gfx::Vertex3D, gfx::Uniform3D>,
	pub(crate) cur_custom_uniform_3d: Option<Vec<(&'static str, gl::UniformValue)>>,

	pub(crate) pipeline_cubemap: gl::Pipeline<gfx::VertexCubemap, gfx::UniformCubemap>,

	pub(crate) cur_canvas: Option<gfx::Canvas>,

	pub(crate) default_font: gfx::BitmapFont,

	pub(crate) draw_calls_last: usize,
	pub(crate) draw_calls: usize,

	pub(crate) transform: Mat4,

	// audio
	pub(crate) audio_device: Option<audio::Device>,

}

fn run_with_conf<S: State>(mut conf: Conf) -> Result<()> {

	let mut event_loop = glutin::EventsLoop::new();

	let mut window_builder = glutin::WindowBuilder::new()
		.with_title(conf.title.to_owned())
		.with_dimensions(LogicalSize::new(conf.width as f64, conf.height as f64))
		.with_resizable(conf.resizable)
		.with_transparency(conf.transparent)
		.with_decorations(!conf.borderless)
		.with_always_on_top(conf.always_on_top)
		.with_fullscreen(conf.fullscreen.then_some(event_loop.get_primary_monitor()))
		;

	let mut ctx_builder = glutin::ContextBuilder::new()
		.with_vsync(conf.vsync)
		.with_gl(GlRequest::Specific(glutin::Api::OpenGl, (2, 1)))
		;

	#[cfg(feature = "gl3")] {
		ctx_builder = ctx_builder
			.with_gl(GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
			.with_gl_profile(glutin::GlProfile::Core)
			;
	}

	let windowed_ctx = unsafe {
		ctx_builder
			.build_windowed(window_builder, &event_loop)
			.map_err(|_| format!("failed to build window"))?
			.make_current()
			.map_err(|_| format!("failed to make opengl context"))?
	};

	let gl = gl::Device::from_loader(|s| {
		windowed_ctx.get_proc_address(s) as *const _
	});

	gl.enable(gl::Capability::Blend);
	gl.enable(gl::Capability::DepthTest);
// 	gl.enable(gl::Capability::CullFace);
// 	gl.cull_face(gl::Face::Back);
// 	gl.front_face(gl::CullMode::CounterClockwise);
	gl.blend_func(gl::BlendFac::SrcAlpha, gl::BlendFac::OneMinusSrcAlpha);
	gl.depth_func(gl::Cmp::LessOrEqual);
	gl.clear_color(0.0, 0.0, 0.0, 0.0);

	let cam = gfx::OrthoCam::new(conf.width as f32, conf.height as f32, gfx::DEFAULT_NEAR, gfx::DEFAULT_FAR);

	use res::shader::*;
	use res::font::*;

	let vert_2d_src = TEMPLATE_2D_VERT.replace("###REPLACE###", DEFAULT_2D_VERT);
	let frag_2d_src = TEMPLATE_2D_FRAG.replace("###REPLACE###", DEFAULT_2D_FRAG);

	let pipeline_2d = gl::Pipeline::new(&gl, &vert_2d_src, &frag_2d_src)?;

	let vert_3d_src = TEMPLATE_3D_VERT.replace("###REPLACE###", DEFAULT_3D_VERT);
	let frag_3d_src = TEMPLATE_3D_FRAG.replace("###REPLACE###", DEFAULT_3D_FRAG);

	let pipeline_3d = gl::Pipeline::new(&gl, &vert_3d_src, &frag_3d_src)?;

	let pipeline_cmap = gl::Pipeline::new(&gl, CUBEMAP_VERT, CUBEMAP_FRAG)?;

	let font_data = conf.default_font
		.take()
		.unwrap_or(CP437);

	let font = gfx::BitmapFont::from_data(&gl, font_data)?;

	let mut ctx = Ctx {

		// app
		quit: false,
		dt: Duration::from_secs(0),
		time: Duration::from_secs(0),
		fps_counter: FPSCounter::new(),

		// input
		key_states: HashMap::new(),
		mouse_states: HashMap::new(),
		mouse_pos: vec2!(),
		gamepad_button_states: HashMap::new(),
		gamepad_axis_pos: HashMap::new(),
		scroll_phase: input::ScrollPhase::Solid,

		// window
		title: conf.title.to_owned(),
		width: conf.width,
		height: conf.height,
		cursor_hidden: conf.cursor_hidden,
		cursor_locked: conf.cursor_locked,

		clipboard_ctx: clipboard::ClipboardProvider::new()
			.map_err(|_| format!("failed to create clipboard context"))?,

		gamepad_ctx: gilrs::Gilrs::new()
			.map_err(|_| format!("failed to create gamepad context"))?,

		windowed_ctx: windowed_ctx,

		renderer_2d: gl::BatchedMesh::<gfx::Vertex2D, gfx::Uniform2D>::new(&gl, gfx::DRAW_COUNT, gfx::DRAW_COUNT)?,
		renderer_3d: gl::BatchedMesh::<gfx::Vertex3D, gfx::Uniform3D>::new(&gl, gfx::DRAW_COUNT, gfx::DRAW_COUNT)?,
		cube_renderer: gl::Mesh::from_shape(&gl, gfx::CubeShape)?,
		cubemap_renderer: gl::Mesh::from_shape(&gl, gfx::CubemapShape)?,

		proj: cam.projection(),
		view: cam.view(),

		empty_tex: gfx::Texture::from_pixels(&gl, 1, 1, &[255; 4])?,

		default_pipeline_2d: pipeline_2d.clone(),
		cur_pipeline_2d: pipeline_2d,
		cur_custom_uniform_2d: None,

		default_pipeline_3d: pipeline_3d.clone(),
		cur_pipeline_3d: pipeline_3d,
		cur_custom_uniform_3d: None,

		pipeline_cubemap: pipeline_cmap,

		cur_canvas: None,

		default_font: font,
		draw_calls: 0,
		draw_calls_last: 0,
		transform: mat4!(),

		gl: Rc::new(gl),

		// audio
		audio_device: audio::default_device(),

		conf: conf,

	};

	if ctx.conf.cursor_hidden {
		ctx.set_cursor_hidden(true);
	}

	if ctx.conf.cursor_locked {
		ctx.set_cursor_locked(true)?;
	}

	#[cfg(feature = "imgui")]
	let mut imgui = imgui::Imgui::new(&ctx)?;

	ctx.clear();
	ctx.swap_buffers()?;

	let mut s = S::init(&mut ctx)?;
	let mut last_frame_time = Instant::now();

	'run: loop {

		let start_time = Instant::now();

		for e in input::poll(&mut ctx, &mut event_loop)? {
			s.event(&mut ctx, &e)?;
		}

		s.update(&mut ctx)?;
		ctx.begin_frame();
		s.draw(&mut ctx)?;
		ctx.end_frame();

		#[cfg(feature = "imgui")]
		imgui.render(&mut ctx, |ui| {
			return s.imgui(ui);
		})?;

		ctx.swap_buffers()?;

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

		ctx.dt = start_time.elapsed();
		ctx.time += ctx.dt;
		ctx.fps_counter.tick(ctx.dt);

	}

	s.quit(&mut ctx)?;

	return Ok(());

}

impl Ctx {

	pub fn quit(&mut self) {
		self.quit = true;
	}

	pub fn dt(&self) -> f32 {
		return self.dt.as_secs_f32();
	}

	pub fn time(&self) -> f32 {
		return self.time.as_secs_f32();
	}

	pub fn fps(&self) -> u16 {
		return self.fps_counter.fps();
	}

	pub fn conf(&self) -> &Conf {
		return &self.conf;
	}

}

pub fn run<S: State>() -> Result<()> {
	return launcher().run::<S>();
}

pub fn launcher() -> Launcher {
	return Launcher::default();
}

impl Launcher {
	pub fn run<S: State>(self) -> Result<()> {
		return run_with_conf::<S>(self.conf);
	}
}

pub(crate) struct FPSCounter {
	frames: usize,
	timer: Duration,
	fps: u16,
}

impl FPSCounter {

	fn new() -> Self {
		return Self {
			frames: 0,
			timer: Duration::from_secs(0),
			fps: 0,
		}
	}

	fn tick(&mut self, dt: Duration) {

		self.frames += 1;
		self.timer += dt;

		if self.timer.as_secs_f32() >= 1.0 {
			self.fps = self.frames as u16;
			self.timer = Duration::from_secs(0);
			self.frames = 0;
		}

	}

	fn fps(&self) -> u16 {
		return self.fps;
	}

}

