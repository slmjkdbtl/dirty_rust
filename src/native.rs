// wengwengweng

use std::rc::Rc;
use std::collections::HashSet;

use glutin::dpi::*;
use glutin::event_loop::EventLoop;

use crate::*;
use math::*;
use input::*;
use window::WindowEvent;

pub struct Window {
	gl: Rc<gl::Device>,
	event_loop: Option<EventLoop<()>>,
	windowed_ctx: glutin::WindowedContext<glutin::PossiblyCurrent>,
	pressed_keys: HashSet<Key>,
	pressed_mouse: HashSet<Mouse>,
	width: i32,
	height: i32,
	mouse_pos: Vec2,
	scroll_phase: input::ScrollPhase,
}

impl Window {

	pub fn new(conf: &conf::Conf) -> Result<Self> {

		let event_loop = EventLoop::new();

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

		let ctx_builder = glutin::ContextBuilder::new()
			.with_vsync(conf.vsync)
			.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (2, 1)))
			;

		let windowed_ctx = unsafe {
			ctx_builder
				.build_windowed(window_builder, &event_loop)
				.map_err(|_| format!("failed to build window"))?
				.make_current()
				.map_err(|_| format!("failed to make opengl context"))?
		};

		let gl = gl::Device::from_loader(|s| {
			return windowed_ctx.get_proc_address(s) as *const _;
		});

		return Ok(Self {
			gl: Rc::new(gl),
			event_loop: Some(event_loop),
			windowed_ctx: windowed_ctx,
			pressed_keys: hset![],
			pressed_mouse: hset![],
			mouse_pos: vec2!(),
			width: conf.width,
			height: conf.height,
			scroll_phase: input::ScrollPhase::Solid,
		});

	}

}

impl window::WindowCtx for Window {

	fn gl(&self) -> &gl::Device {
		return &self.gl;
	}

	fn swap(&self) -> Result<()> {
		self.windowed_ctx
			.swap_buffers()
			.map_err(|_| format!("failed to swap buffer"))?;
		return Ok(());
	}

	fn key_down(&self, k: Key) -> bool {
		return self.pressed_keys.contains(&k);
	}

	fn mouse_down(&self, m: Mouse) -> bool {
		return self.pressed_mouse.contains(&m);
	}

	fn dpi(&self) -> f32 {
		return self.windowed_ctx.window().scale_factor() as f32;
	}

	fn width(&self) -> i32 {
		return self.width;
	}

	fn height(&self) -> i32 {
		return self.height;
	}

	fn run(
		mut self,
		mut handle: impl FnMut(&mut Self, WindowEvent) -> Result<()> + 'static,
	) {

		use glutin::event_loop::ControlFlow;

		let mut update = false;

		let event_loop = match self.event_loop.take() {
			Some(e) => e,
			None => return,
		};

		event_loop.run(move |e, _, flow| {

			*flow = ControlFlow::Poll;

			let event_result: Result<()> = try {

				use glutin::event::WindowEvent as WEvent;
				use glutin::event::DeviceEvent as DEvent;
				use glutin::event::TouchPhase;
				use glutin::event::ElementState;
				use input::*;

// 				#[cfg(feature = "midi")]
// 				if let Ok(mut buf) = midi_buf.lock() {
// 					for msg in std::mem::replace(&mut *buf, vec![]) {
// 						handle(Event::MIDI(msg.clone()));
// 					}
// 				}

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

											handle(&mut self, WindowEvent::Input(Event::KeyPressRepeat(key)));

											if !self.key_down(key) {
												handle(&mut self, WindowEvent::Input(Event::KeyPress(key)));
											}

											self.pressed_keys.insert(key);

										},

										ElementState::Released => {
											self.pressed_keys.remove(&key);
											handle(&mut self, WindowEvent::Input(Event::KeyRelease(key)));
										},

									}

								}

							}

						},

						WEvent::MouseInput { button, state, .. } => {

							if let Some(button) = Mouse::from_extern(*button) {

								match state {

									ElementState::Pressed => {
										self.pressed_mouse.insert(button);
										handle(&mut self, WindowEvent::Input(Event::MousePress(button)));
									},
									ElementState::Released => {
										self.pressed_mouse.remove(&button);
										handle(&mut self, WindowEvent::Input(Event::MouseRelease(button)));
									},

								}

							}

						},

						WEvent::CursorMoved { position, .. } => {

							let mpos: Vec2 = position.to_logical(self.dpi() as f64).into();
							let (w, h) = (self.width as f32, self.height as f32);
							let mpos = vec2!(mpos.x - w / 2.0, h / 2.0 - mpos.y);

							self.mouse_pos = mpos;

						},

						WEvent::MouseWheel { delta, phase, .. } => {

							match phase {
								TouchPhase::Started => {
									self.scroll_phase = ScrollPhase::Solid;
								},
								TouchPhase::Ended => {
									self.scroll_phase = ScrollPhase::Trailing;
								},
								_ => {},
							}

							let p = self.scroll_phase;
							let d: Vec2 = (*delta).into();

							handle(&mut self, WindowEvent::Input(Event::Wheel(vec2!(d.x, -d.y), p)));

						},

						WEvent::ReceivedCharacter(ch) => {
							if !INVALID_CHARS.contains(&ch) && !ch.is_control() {
								handle(&mut self, WindowEvent::Input(Event::CharInput(*ch)));
							}
						},

						WEvent::Resized(size) => {

							let dpi = self.dpi() as f64;
							let lsize: LogicalSize<f64> = size.to_logical(dpi);
							let w = lsize.width as i32;
							let h = lsize.height as i32;

							self.width = w;
							self.height = h;
// 							let cam = self.default_cam();
// 							self.apply_cam(&cam);
							self.windowed_ctx.resize(*size);

							handle(&mut self, WindowEvent::Input(Event::Resize(w, h)));

						},

						WEvent::Touch(touch) => {
							handle(&mut self, WindowEvent::Input(Event::Touch(touch.id, touch.location.into())));
						},

						WEvent::HoveredFile(path) => {
							handle(&mut self, WindowEvent::Input(Event::FileHover(path.to_path_buf())));
						},

						WEvent::HoveredFileCancelled => {
							handle(&mut self, WindowEvent::Input(Event::FileHoverCancel));
						},

						WEvent::DroppedFile(path) => {
							handle(&mut self, WindowEvent::Input(Event::FileDrop(path.to_path_buf())));
						},

						WEvent::Focused(b) => {
							handle(&mut self, WindowEvent::Input(Event::Focus(*b)));
						},

						WEvent::CursorEntered { .. } => {
							handle(&mut self, WindowEvent::Input(Event::CursorEnter));
						},

						WEvent::CursorLeft { .. } => {
							handle(&mut self, WindowEvent::Input(Event::CursorLeave));
						},

						_ => (),

					},

					glutin::event::Event::DeviceEvent { event, .. } => match event {
						DEvent::MouseMotion { delta } => {
							handle(&mut self, WindowEvent::Input(Event::MouseMove(vec2!(delta.0, -delta.1))));
						},
						_ => (),
					},

					glutin::event::Event::RedrawRequested(_) => {

						handle(&mut self, WindowEvent::Frame);

// 						if ctx.quit {
// 							*flow = ControlFlow::Exit;
// 						}

					},

					glutin::event::Event::MainEventsCleared => {

						// ugly workaround
						update = !update;

						if update {
							self.windowed_ctx
								.window()
								.request_redraw();
						}

					},

					_ => {},

				};

			};

			if let Err(err) = event_result {
				eprintln!("{}", err);
			}

		});

	}

}

impl From<glutin::event::MouseScrollDelta> for Vec2 {
	fn from(delta: glutin::event::MouseScrollDelta) -> Self {
		use glutin::event::MouseScrollDelta;
		match delta {
			MouseScrollDelta::PixelDelta(pos) => {
				return vec2!(pos.x, pos.y);
			},
			MouseScrollDelta::LineDelta(x, y) => {
				return vec2!(x, y);
			}
		};
	}
}

impl From<Vec2> for LogicalPosition<f64> {
	fn from(pos: Vec2) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

impl From<LogicalPosition<f64>> for Vec2 {
	fn from(pos: LogicalPosition<f64>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

#[cfg(not(web))]
impl From<PhysicalPosition<f64>> for Vec2 {
	fn from(pos: PhysicalPosition<f64>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

impl From<PhysicalPosition<i32>> for Vec2 {
	fn from(pos: PhysicalPosition<i32>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

