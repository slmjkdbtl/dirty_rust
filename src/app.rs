// wengwengweng

//! Windowing, Input, and Graphics

use std::rc::Rc;
use std::collections::HashMap;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Instant;
use std::time::Duration;

use glutin::dpi::*;
use glutin::GlRequest;
use glutin::event_loop::ControlFlow;

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

	let (windowed_ctx, event_loop, gl) =  {

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
		.unwrap_or(UNSCII);

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

		proj: cam.proj(),
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

	#[cfg(feature = "midi")]
	let midi_buf = {

		let buf = Arc::new(Mutex::new(vec![]));
		let buf_in = buf.clone();

		// TODO: why does this still block the main thread sometime??
		thread::spawn(move || {

			// TODO: extremely slow
			if let Ok(midi_in) = midir::MidiInput::new("DIRTY") {

				let port_id = midi_in.port_count() - 1;
				let port_name = midi_in.port_name(port_id).unwrap_or(format!("unknown"));

				let _conn = midi_in.connect(port_id, &format!("DIRTY ({})", port_name), move |_, msg, buf| {
					if let Ok(mut buf) = buf.lock() {
						buf.push(midi::Msg::from(&msg));
					}
				}, buf_in).map_err(|_| format!("failed to read midi input"));

				loop {}

			} else {
				eprintln!("failed to init midi reader")
			}

		});

		buf

	};

	#[cfg(feature = "imgui")]
	let mut imgui = imgui::Imgui::new(&ctx)?;

	ctx.clear();
	ctx.swap_buffers()?;

	let mut s = S::init(&mut ctx)?;
	let mut last_frame_time = Instant::now();
	let mut update = false;

	event_loop.run(move |e, _, flow| {

		*flow = ControlFlow::Poll;

		let event_result: Result<()> = try {

			use glutin::event::WindowEvent as WEvent;
			use glutin::event::DeviceEvent as DEvent;
			use glutin::event::TouchPhase;
			use glutin::event::ElementState;
			use input::*;

			let mut events = vec![];

			#[cfg(feature = "midi")]
			if let Ok(mut buf) = midi_buf.lock() {
				for msg in std::mem::replace(&mut *buf, vec![]) {
					events.push(Event::MIDI(msg.clone()));
				}
			}

			match e {

				glutin::event::Event::LoopDestroyed => *flow = ControlFlow::Exit,

				glutin::event::Event::WindowEvent { ref event, .. } => match event {
					WEvent::CloseRequested => {
						*flow = ControlFlow::Exit;
					},
					WEvent::KeyboardInput { input, .. } => {

						if let Some(kc) = input.virtual_keycode {

							if let Some(key) = Key::from_extern(kc) {

								match input.state {

									ElementState::Pressed => {

										events.push(Event::KeyPressRepeat(key));

										if ctx.key_up(key) || ctx.key_released(key) {
											ctx.key_states.insert(key, ButtonState::Pressed);
											events.push(Event::KeyPress(key));
										}

									},

									ElementState::Released => {
										if ctx.key_down(key) || ctx.key_pressed(key) {
											ctx.key_states.insert(key, ButtonState::Released);
											events.push(Event::KeyRelease(key));
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
										events.push(Event::MousePress(button));
									}
								},
								ElementState::Released => {
									if ctx.mouse_down(button) || ctx.mouse_pressed(button) {
										ctx.mouse_states.insert(button, ButtonState::Released);
										events.push(Event::MouseRelease(button));
									}
								},

							}

						}

					},

					WEvent::CursorMoved { position, .. } => {

						let mpos: Vec2 = position.to_logical(ctx.dpi() as f64).into();
// 						let mpos: Vec2 = (*position).into();
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
						let d: Vec2 = (*delta).into();

						events.push(Event::Scroll(vec2!(d.x, -d.y), p));

					},

					WEvent::ReceivedCharacter(ch) => {
						if !INVALID_CHARS.contains(&ch) && !ch.is_control() {
							events.push(Event::CharInput(*ch));
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

						events.push(Event::Resize(w, h));

					},

					WEvent::Touch(touch) => {
						events.push(Event::Touch(touch.id, touch.location.into()));
					},

					WEvent::HoveredFile(path) => {
						events.push(Event::FileHover(path.to_path_buf()));
					},

					WEvent::HoveredFileCancelled => {
						events.push(Event::FileHoverCancel);
					},

					WEvent::DroppedFile(path) => {
						events.push(Event::FileDrop(path.to_path_buf()));
					},

					WEvent::Focused(b) => {
						events.push(Event::Focus(*b));
					},

					WEvent::CursorEntered { .. } => {
						events.push(Event::CursorEnter);
					},

					WEvent::CursorLeft { .. } => {
						events.push(Event::CursorLeave);
					},
					_ => (),
				},

				glutin::event::Event::DeviceEvent { event, .. } => match event {
					DEvent::MouseMotion { delta } => {
						events.push(Event::MouseMove(vec2!(delta.0, delta.1)));
					},
					_ => (),
				},

				glutin::event::Event::RedrawRequested(_) => {

					if let Some(fps_cap) = ctx.conf.fps_cap {

						let real_dt = last_frame_time.elapsed().as_millis();
						let expected_dt = (1000.0 / fps_cap as f32) as u128;

						if real_dt < expected_dt {
							thread::sleep(Duration::from_millis((expected_dt - real_dt) as u64));
						}

					}

					ctx.dt = last_frame_time.elapsed();
					ctx.time += ctx.dt;
					ctx.fps_counter.tick(ctx.dt);

					last_frame_time = Instant::now();

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
					ctx.end_frame();

					#[cfg(feature = "imgui")]
					imgui.render(&mut ctx, |ui| {
						return s.imgui(ui);
					})?;

					ctx.swap_buffers()?;

					if ctx.quit {
						*flow = ControlFlow::Exit;
					}

				},

				glutin::event::Event::MainEventsCleared => {

					// ugly workaround
					update = !update;

					if update {
						ctx.windowed_ctx
							.window()
							.request_redraw();
					}

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

										events.push(Event::GamepadPress(id, button));

									}

								}

							},

							ButtonRepeated(button, ..) => {
								if let Some(button) = GamepadButton::from_extern(button) {
									events.push(Event::GamepadPressRepeat(id, button));
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

										events.push(Event::GamepadRelease(id, button));

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
										events.push(Event::GamepadAxis(id, GamepadAxis::LStick, pos.0));
									},
									gilrs::ev::Axis::LeftStickY => {
										pos.0.y = val;
										events.push(Event::GamepadAxis(id, GamepadAxis::LStick, pos.0));
									},
									gilrs::ev::Axis::RightStickX => {
										pos.1.x = val;
										events.push(Event::GamepadAxis(id, GamepadAxis::RStick, pos.1));
									},
									gilrs::ev::Axis::RightStickY => {
										pos.1.y = val;
										events.push(Event::GamepadAxis(id, GamepadAxis::RStick, pos.1));
									},
									_ => {},

								}

								ctx.gamepad_axis_pos.insert(id, pos);

							},

							Connected => {
								events.push(Event::GamepadConnect(id));
							},

							Disconnected => {
								events.push(Event::GamepadDisconnect(id));
							},

							_ => {},

						}

					}

				},

				_ => {},

			};

			for e in events {

				#[cfg(feature = "imgui")]
				imgui.event(&mut ctx, &e);

				s.event(&mut ctx, &e)?;

			}

		};

		if let Err(err) = event_result {
			eprintln!("{}", err);
		}

	});

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

