// wengwengweng

macro_rules! expose {

	($tname:ident, $mod:ident, {$($fn:ident($($argn:ident: $argt:ty),*)$( -> $return:ty)?),*}) => {

		pub trait $tname {
			$(
				fn $fn(&self, $($argn: $argt),*)$( -> $return)?;
			)*
		}

		impl $tname for app::Ctx {
			$(
				fn $fn(&self, $($argn: $argt),*)$( -> $return)? {
					// ...
				}
			)*
		}

		$(
			pub fn $fn(ctx: &app::Ctx, $($argn: $argt),*)$( -> $return)? {
				return ctx.$mod.$fn($($argn),*);
			}
		)*

	};

	($mod:ident, $fn:ident($($argn:ident: $argt:ty),*)$( -> $return:ty)?) => {
		pub fn $fn(ctx: &app::Ctx, $($argn: $argt),*)$( -> $return)? {
			return ctx.$mod.$fn($($argn),*);
		}
	};
	($mod:ident(mut), $fn:ident($($argn:ident: $argt:ty),*)$( -> $return:ty)?) => {
		pub fn $fn(ctx: &mut app::Ctx, $($argn: $argt),*)$( -> $return)? {
			return ctx.$mod.$fn($($argn),*);
		}
	};
}

pub mod gl;
pub mod gfx;
pub mod g2d;
pub mod window;

use std::thread;
use std::time::Instant;
use std::time::Duration;

use crate::*;
use crate::math::*;

pub struct Ctx {

	pub(self) window: window::Ctx,
	pub(self) gfx: gfx::Ctx,

	quit: bool,
	dt: f32,
	time: f32,
	fps_cap: u16,
	fps_counter: FPSCounter,

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
	pub clear_color: Color,
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
			vsync: false,
			fullsize_content: false,
			hide_title: false,
			hide_titlebar_buttons: false,
			titlebar_transparent: false,
			cursor_hidden: false,
			cursor_locked: false,
			clear_color: color!(0, 0, 0, 1),
		};
	}

}

pub trait State {

	fn init(_: &mut Ctx) -> Result<Self> where Self: Sized;

	fn run(&mut self, _: &mut Ctx, _: f32) -> Result<()> {
		return Ok(());
	}

	fn quit(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

}

pub fn run<S: State>(conf: Conf) -> Result<()> {

	let window = window::Ctx::new(&conf)?;
	let gfx = gfx::Ctx::new(&window, &conf)?;

	let mut ctx = Ctx {

		window: window,
		gfx: gfx,

		quit: false,
		dt: 0.0,
		time: 0.0,
		fps_cap: 60,
		fps_counter: FPSCounter::new(16),

	};

	let mut s = S::init(&mut ctx)?;

	loop {

		let start_time = Instant::now();

		ctx.window.poll()?;

		if ctx.window.should_quit() {
			break;
		}

		let dt = ctx.dt;

		ctx.gfx.begin();
		s.run(&mut ctx, dt)?;
		ctx.gfx.end();
		ctx.window.swap()?;

		if ctx.quit {
			break;
		}

		let real_dt = start_time.elapsed().as_millis();
		let expected_dt = (1000.0 / ctx.fps_cap as f32) as u128;

		if real_dt < expected_dt {
			thread::sleep(Duration::from_millis((expected_dt - real_dt) as u64));
		}

		ctx.dt = start_time.elapsed().as_millis() as f32 / 1000.0;
		ctx.time += ctx.dt;
		ctx.fps_counter.push((1.0 / ctx.dt) as u16);

	}

	return Ok(());

}

pub fn quit(ctx: &mut Ctx) {
	ctx.quit = true;
}

pub fn dt(ctx: &Ctx) -> f32 {
	return ctx.dt;
}

pub fn fps(ctx: &Ctx) -> u16 {
	return ctx.fps_counter.get_avg();
}

pub fn time(ctx: &Ctx) -> f32 {
	return ctx.time;
}

struct FPSCounter {
	buffer: Vec<u16>,
}

impl FPSCounter {

	fn new(max: usize) -> Self {
		return Self {
			buffer: Vec::with_capacity(max),
		}
	}

	fn push(&mut self, fps: u16) {
		if self.buffer.len() == self.buffer.capacity() {
			self.buffer.remove(0);
		}
		self.buffer.push(fps);
	}

	fn get_avg(&self) -> u16 {

		if self.buffer.is_empty() {
			return 0;
		}

		let sum: u16 = self.buffer.iter().sum();
		return sum / self.buffer.len() as u16;

	}

}

