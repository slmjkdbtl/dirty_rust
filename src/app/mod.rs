// wengwengweng

//! Windowing, Input, and Graphics

pub mod gfx;
pub mod res;
mod texture;
mod shader;
mod canvas;
mod transform;
mod font;
mod camera;
mod model;
mod desc;
mod skybox;

pub mod input;
pub mod window;
pub mod shapes;

mod state;
mod conf;

#[cfg(feature = "gkit")]
pub mod kit;

#[cfg(feature = "imgui")]
mod imgui;

pub use state::*;
pub use conf::*;

use crate::*;
use crate::math::*;

use std::rc::Rc;
use std::collections::HashMap;
use std::thread;
use std::time::Instant;
use std::time::Duration;

use glutin::dpi::*;
use glutin::GlRequest;
use glutin::event_loop::ControlFlow;

use derive_more::*;

use gfx::Camera;

use input::ButtonState;
use input::Key;
use input::Mouse;
use input::GamepadID;
use input::GamepadButton;

const DRAW_COUNT: usize = 65536;
const NEAR: f32 = -4096.0;
const FAR: f32 = 4096.0;

pub struct Ctx {

	pub(self) conf: Conf,

	// lifecycle
	pub(self) quit: bool,
	pub(self) dt: Time,
	pub(self) time: Time,
	pub(self) fps_counter: FPSCounter,
	pub(self) last_frame_time: Instant,

	// input
	pub(self) key_states: HashMap<Key, ButtonState>,
	pub(self) mouse_states: HashMap<Mouse, ButtonState>,
	pub(self) mouse_pos: Vec2,
	pub(self) gamepad_button_states: HashMap<GamepadID, HashMap<GamepadButton, ButtonState>>,
	pub(self) gamepad_axis_pos: HashMap<GamepadID, (Vec2, Vec2)>,
	pub(self) scroll_phase: input::ScrollPhase,

	// window
	pub(self) title: String,
	pub(self) cursor_hidden: bool,
	pub(self) cursor_locked: bool,
	pub(self) width: i32,
	pub(self) height: i32,

	pub(self) clipboard_ctx: clipboard::ClipboardContext,

	pub(self) windowed_ctx: glutin::WindowedContext<glutin::PossiblyCurrent>,
	pub(self) gamepad_ctx: gilrs::Gilrs,
// 	pub(self) imgui: imgui::Imgui,

	// gfx
	pub(self) gl: Rc<gl::Device>,

	pub(self) proj: Mat4,
	pub(self) view: Mat4,

	pub(self) renderer_2d: gl::BatchedMesh<gfx::Vertex2D, gfx::Uniform2D>,
	pub(self) cube_renderer: gl::Mesh<gfx::Vertex3D, gfx::Uniform3D>,
	pub(self) renderer_3d: gl::BatchedMesh<gfx::Vertex3D, gfx::Uniform3D>,
	pub(self) cubemap_renderer: gl::Mesh<gfx::VertexCubemap, gfx::UniformCubemap>,

	pub(self) empty_tex: gfx::Texture,

	pub(self) default_pipeline_2d: gl::Pipeline<gfx::Vertex2D, gfx::Uniform2D>,
	pub(self) cur_pipeline_2d: gl::Pipeline<gfx::Vertex2D, gfx::Uniform2D>,
	pub(self) cur_custom_uniform_2d: Option<Vec<(&'static str, gl::UniformValue)>>,

	pub(self) default_pipeline_3d: gl::Pipeline<gfx::Vertex3D, gfx::Uniform3D>,
	pub(self) cur_pipeline_3d: gl::Pipeline<gfx::Vertex3D, gfx::Uniform3D>,
	pub(self) cur_custom_uniform_3d: Option<Vec<(&'static str, gl::UniformValue)>>,

	pub(self) pipeline_cubemap: gl::Pipeline<gfx::VertexCubemap, gfx::UniformCubemap>,

	pub(self) cur_canvas: Option<gfx::Canvas>,

	pub(self) default_font: gfx::BitmapFont,

	pub(self) draw_calls_last: usize,
	pub(self) draw_calls: usize,

	pub(self) transform: Mat4,

}

fn run_with_conf<S: State>(mut conf: Conf) -> Result<()> {

	let (windowed_ctx, mut event_loop, gl) =  {

		let event_loop = glutin::event_loop::EventLoop::new();

		let mut window_builder = glutin::window::WindowBuilder::new()
			.with_title(conf.title.to_owned())
			.with_resizable(conf.resizable)
			.with_transparent(conf.transparent)
			.with_decorations(!conf.borderless)
			.with_always_on_top(conf.always_on_top)
			.with_inner_size(LogicalSize::new(conf.width as f64, conf.height as f64))
			;

		if conf.fullscreen {
			window_builder = window_builder
				.with_fullscreen(Some(glutin::window::Fullscreen::Borderless(event_loop.primary_monitor())));
		}

		#[cfg(target_os = "macos")] {

			use glutin::platform::macos::WindowBuilderExtMacOS;

			window_builder = window_builder
				.with_disallow_hidpi(!conf.hidpi)
				;

		}

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

		(windowed_ctx, event_loop, gl)

	};

	let mut imgui = imgui::Imgui::new(&gl, windowed_ctx.window())?;

	let c = conf.clear_color;

	gl.enable(gl::Capability::Blend);
	gl.enable(gl::Capability::DepthTest);
//	gl.enable(gl::Capability::CullFace);
//	gl.cull_face(gl::Face::Back);
	gl.blend_func(gl::BlendFac::SrcAlpha, gl::BlendFac::OneMinusSrcAlpha);
	gl.depth_func(gl::Cmp::LessOrEqual);
	gl.clear_color(c.r, c.g, c.b, c.a);

	let cam = gfx::OrthoCam::new(conf.width as f32, conf.height as f32, NEAR, FAR);

	let empty_tex = gl::Texture2D::from(&gl, 1, 1, &[255; 4])?;
	let empty_tex = gfx::Texture::from_gl_tex(empty_tex);

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
		dt: Time::new(0.0),
		time: Time::new(0.0),
		fps_counter: FPSCounter::new(),
		last_frame_time: Instant::now(),

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

		renderer_2d: gl::BatchedMesh::<gfx::Vertex2D, gfx::Uniform2D>::new(&gl, DRAW_COUNT, DRAW_COUNT)?,
		renderer_3d: gl::BatchedMesh::<gfx::Vertex3D, gfx::Uniform3D>::new(&gl, DRAW_COUNT, DRAW_COUNT)?,
		cube_renderer: gl::Mesh::from_shape(&gl, gfx::CubeShape)?,
		cubemap_renderer: gl::Mesh::from_shape(&gl, gfx::CubemapShape)?,
// 		imgui: imgui,
		gl: Rc::new(gl),

		proj: cam.projection(),
		view: cam.lookat(),

		empty_tex: empty_tex,

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

		conf: conf,

	};

	if ctx.conf.cursor_hidden {
		ctx.set_cursor_hidden(true);
	}

	if ctx.conf.cursor_locked {
		ctx.set_cursor_locked(true)?;
	}

	ctx.clear();
	ctx.swap_buffers()?;

	let mut s = S::init(&mut ctx)?;

	event_loop.run(move |event, _, flow| {
		match handle_event(&mut ctx, &mut s, &mut imgui, event) {
			Ok(f) => {
				*flow = f;
			},
			Err(err) => {
				eprintln!("{}", err);
				*flow = ControlFlow::Poll;
			},
		}
	});

}

fn handle_event(mut ctx: &mut Ctx, s: &mut impl State, imgui: &mut imgui::Imgui, event: glutin::event::Event<()>) -> Result<ControlFlow> {

	use glutin::event::WindowEvent as WEvent;
	use glutin::event::DeviceEvent as DEvent;
	use glutin::event::TouchPhase;
	use glutin::event::ElementState;
	use input::*;

	imgui.handle_event(&ctx, &event);

	match event {

		glutin::event::Event::LoopDestroyed => return Ok(ControlFlow::Exit),

		glutin::event::Event::WindowEvent { ref event, .. } => match event {
			WEvent::CloseRequested => {
				return Ok(ControlFlow::Exit);
			},
			WEvent::KeyboardInput { input, .. } => {

				if let Some(kc) = input.virtual_keycode {

					if let Some(key) = Key::from_extern(kc) {

						match input.state {

							ElementState::Pressed => {

								s.event(&mut ctx, &Event::KeyPressRepeat(key))?;

								if ctx.key_up(key) || ctx.key_released(key) {
									ctx.key_states.insert(key, ButtonState::Pressed);
									s.event(&mut ctx, &Event::KeyPress(key))?;
								}

							},

							ElementState::Released => {
								if ctx.key_down(key) || ctx.key_pressed(key) {
									ctx.key_states.insert(key, ButtonState::Released);
									s.event(&mut ctx, &Event::KeyRelease(key))?;
								}
							},

						}

					}

				}

			},

			WEvent::MouseInput { button, state, .. } => {

				if let Some(button) = Mouse::from_extern(*button) {

					match state {

						ElementState::Pressed => {
							if ctx.mouse_up(button) || ctx.mouse_released(button) {
								ctx.mouse_states.insert(button, ButtonState::Pressed);
								s.event(&mut ctx, &Event::MousePress(button))?;
							}
						},
						ElementState::Released => {
							if ctx.mouse_down(button) || ctx.mouse_pressed(button) {
								ctx.mouse_states.insert(button, ButtonState::Released);
								s.event(&mut ctx, &Event::MouseRelease(button))?;
							}
						},

					}

				}

			},

			WEvent::CursorMoved { position, .. } => {

				let mpos: Vec2 = (*position).into();
				let (w, h) = (ctx.width as f32, ctx.height as f32);
				let mpos = vec2!(mpos.x - w / 2.0, h / 2.0 - mpos.y);

				ctx.mouse_pos = mpos;

			},

			WEvent::MouseWheel { delta, phase, .. } => {

				match phase {
					TouchPhase::Started => {
						ctx.scroll_phase = ScrollPhase::Solid;
					},
					TouchPhase::Ended => {
						ctx.scroll_phase = ScrollPhase::Trailing;
					},
					_ => {},
				}

				let p = ctx.scroll_phase;
				s.event(&mut ctx, &Event::Scroll((*delta).into(), p))?;

			},

			WEvent::ReceivedCharacter(ch) => {
				if !INVALID_CHARS.contains(&ch) && !ch.is_control() {
					s.event(&mut ctx, &Event::CharInput(*ch))?;
				}
			},

			WEvent::Resized(size) => {

				let dpi = ctx.dpi() as f64;
				let lsize: LogicalSize<f64> = size.to_logical(dpi);
				let w = lsize.width as i32;
				let h = lsize.height as i32;

				ctx.width = w;
				ctx.height = h;
				ctx.reset_default_cam();
				ctx.windowed_ctx.resize(*size);

				s.event(&mut ctx, &Event::Resize(w, h))?;

			},

			WEvent::Touch(touch) => {
				s.event(&mut ctx, &Event::Touch(touch.id, touch.location.into()))?;
			},

			WEvent::HoveredFile(path) => {
				s.event(&mut ctx, &Event::FileHover(path.to_path_buf()))?;
			},

			WEvent::HoveredFileCancelled => {
				s.event(&mut ctx, &Event::FileHoverCancel)?;
			},

			WEvent::DroppedFile(path) => {
				s.event(&mut ctx, &Event::FileDrop(path.to_path_buf()))?;
			},

			WEvent::Focused(b) => {
				s.event(&mut ctx, &Event::Focus(*b))?;
			},

			WEvent::CursorEntered { .. } => {
				s.event(&mut ctx, &Event::CursorEnter)?;
			},

			WEvent::CursorLeft { .. } => {
				s.event(&mut ctx, &Event::CursorLeave)?;
			},
			_ => (),
		},

		glutin::event::Event::DeviceEvent { event, .. } => match event {
			DEvent::MouseMotion { delta } => {
				s.event(&mut ctx, &Event::MouseMove(vec2!(delta.0, delta.1)))?;
			},
			_ => (),
		},

		glutin::event::Event::RedrawRequested(_) => {

			if let Some(fps_cap) = ctx.conf.fps_cap {

				let real_dt = ctx.last_frame_time.elapsed().as_millis();
				let expected_dt = (1000.0 / fps_cap as f32) as u128;

				if real_dt < expected_dt {
					thread::sleep(Duration::from_millis((expected_dt - real_dt) as u64));
				}

			}

			ctx.dt.set_inner(ctx.last_frame_time.elapsed());
			ctx.time += ctx.dt;
			ctx.fps_counter.tick(ctx.dt);

			ctx.last_frame_time = Instant::now();

			for state in ctx.key_states.values_mut() {
				if state == &ButtonState::Pressed {
					*state = ButtonState::Down;
				} else if state == &ButtonState::Released {
					*state = ButtonState::Up;
				}
			}

			for state in ctx.mouse_states.values_mut() {
				if state == &ButtonState::Pressed {
					*state = ButtonState::Down;
				} else if state == &ButtonState::Released {
					*state = ButtonState::Up;
				}
			}

			for states in ctx.gamepad_button_states.values_mut() {
				for state in states.values_mut() {
					if state == &ButtonState::Pressed {
						*state = ButtonState::Down;
					} else if state == &ButtonState::Released {
						*state = ButtonState::Up;
					}
				}
			}

			s.update(&mut ctx)?;
			ctx.begin_frame();
			s.draw(&mut ctx)?;
			imgui.render(ctx, |_| {
				// ..
			})?;
			ctx.end_frame();
			ctx.swap_buffers()?;

			if ctx.quit {
				return Ok(ControlFlow::Exit);
			}

		},

		glutin::event::Event::MainEventsCleared => {

			ctx.windowed_ctx
				.window()
				.request_redraw();

			while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad_ctx.next_event() {

				use gilrs::ev::EventType::*;

				match event {

					ButtonPressed(button, ..) => {

						if let Some(button) = GamepadButton::from_extern(button) {

							if ctx.gamepad_up(id, button) || ctx.gamepad_released(id, button) {

								ctx
									.gamepad_button_states
									.entry(id)
									.or_insert(hmap![])
									.insert(button, ButtonState::Pressed);

								s.event(&mut ctx, &Event::GamepadPress(id, button))?;

							}

						}

					},

					ButtonRepeated(button, ..) => {
						if let Some(button) = GamepadButton::from_extern(button) {
							s.event(&mut ctx, &Event::GamepadPressRepeat(id, button))?;
						}
					},

					ButtonReleased(button, ..) => {

						if let Some(button) = GamepadButton::from_extern(button) {

							if ctx.gamepad_down(id, button) || ctx.gamepad_pressed(id, button) {

								ctx
									.gamepad_button_states
									.entry(id)
									.or_insert(hmap![])
									.insert(button, ButtonState::Released);

								s.event(&mut ctx, &Event::GamepadRelease(id, button))?;

							}

						}

					},

					AxisChanged(axis, val, ..) => {

						let mut pos = ctx.gamepad_axis_pos
							.entry(id)
							.or_insert((vec2!(), vec2!()))
							.clone()
							;

						match axis {
							gilrs::ev::Axis::LeftStickX => {
								pos.0.x = val;
								s.event(&mut ctx, &Event::GamepadAxis(id, GamepadAxis::LStick, pos.0))?;
							},
							gilrs::ev::Axis::LeftStickY => {
								pos.0.y = val;
								s.event(&mut ctx, &Event::GamepadAxis(id, GamepadAxis::LStick, pos.0))?;
							},
							gilrs::ev::Axis::RightStickX => {
								pos.1.x = val;
								s.event(&mut ctx, &Event::GamepadAxis(id, GamepadAxis::RStick, pos.1))?;
							},
							gilrs::ev::Axis::RightStickY => {
								pos.1.y = val;
								s.event(&mut ctx, &Event::GamepadAxis(id, GamepadAxis::RStick, pos.1))?;
							},
							_ => {},

						}

						ctx.gamepad_axis_pos.insert(id, pos);

					},

					Connected => {
						s.event(&mut ctx, &Event::GamepadConnect(id))?;
					},

					Disconnected => {
						s.event(&mut ctx, &Event::GamepadDisconnect(id))?;
					},

					_ => {},

				}

			}

		},

		_ => (),

	}

	return Ok(ControlFlow::Poll);

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

impl Ctx {

	pub fn quit(&mut self) {
		self.quit = true;
	}

	pub fn dt(&self) -> f32 {
		return self.dt.as_secs();
	}

	pub fn time(&self) -> f32 {
		return self.time.as_secs();
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

	pub fn texture_filter(mut self, f: gfx::FilterMode) -> Self {
		self.conf.texture_filter = f;
		return self;
	}

	pub fn default_font(mut self, f: gfx::BitmapFontData) -> Self {
		self.conf.default_font = Some(f);
		return self;
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

pub trait GfxCtx {
	fn gl_ctx(&self) -> &gl::Device;
}

impl GfxCtx for Ctx {
	fn gl_ctx(&self) -> &gl::Device {
		return &self.gl;
	}
}

impl GfxCtx for gl::Device {
	fn gl_ctx(&self) -> &gl::Device {
		return self;
	}
}

