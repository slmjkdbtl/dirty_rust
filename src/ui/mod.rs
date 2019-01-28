// wengwengweng

use std::any::Any;
use std::collections::BTreeMap;

use crate::*;
use crate::math::*;
use crate::addons::col;

use crate::utils::id::*;

const BAR_HEIGHT: u32 = 40;
const CORNER: f32 = 1.4;

ctx!(UI: UICtx);

struct UICtx {
	windows: BTreeMap<Id, Window>,
	id_generator: IdGenerator,
	active_window: Option<Id>,
}

pub trait Widget: Any {
	fn update(&mut self) {}
	fn draw(&self) {}
}

/// initialize res module
pub fn init() {

	ctx_init(UICtx {
		windows: BTreeMap::new(),
		id_generator: IdGenerator::new(),
		active_window: None,
	});

}

enum WindowState {
	Idle,
	Dragged(Vec2),
}

pub struct Window {

	title: String,
	pos: Vec2,
	width: u32,
	height: u32,
	state: WindowState,
	widgets: Vec<Box<Widget>>,
	id: Option<Id>,

}

impl Window {

	pub fn new(title: &str, pos: Vec2, width: u32, height: u32) -> Self {

		return Self {

			title: String::from(title),
			pos: pos,
			width: width,
			height: height,
			state: WindowState::Idle,
			widgets: Vec::new(),
			id: None,

		};

	}

	pub fn add<W: Widget>(&mut self, w: W) {
		self.widgets.push(Box::new(w));
	}

}

pub fn draw() {

	let ctx_mut = ctx_get_mut();

	gfx::push();
	gfx::reset();

	for (id, w) in &mut ctx_mut.windows {

		update_window(w);
		draw_window(w);

	}

	gfx::pop();

}

fn update_window(w: &mut Window) {

	let mpos = window::mouse_pos();

	if window::mouse_pressed(Mouse::Left) {

		if col::point_rect(mpos, rect!(w.pos.x, w.pos.y, w.width, BAR_HEIGHT)) {

			let ctx_mut = ctx_get_mut();
			let id = w.id.expect("oh no");
			let windows = &mut ctx_mut.windows;

			w.state = WindowState::Dragged(mpos - w.pos);
			ctx_mut.active_window = Some(id);
// 			windows.remove(&id);
// 			windows.insert(id, w);

		}

	}

	if let WindowState::Dragged(pos) = w.state {

		w.pos = mpos - pos;

		if window::mouse_released(Mouse::Left) {
			w.state = WindowState::Idle;
		}

	}

}

fn draw_window(w: &Window) {

	let ctx = ctx_get();

	gfx::push();
	gfx::translate(w.pos);

	gfx::color(color!(0, 0.35, 0.35, 1));
	gfx::rect(vec2!(w.width, w.height));

	gfx::color(color!(0, 0.51, 0.51, 1));
	gfx::rect(vec2!(w.width, BAR_HEIGHT));

	gfx::line_width(3);
	gfx::color(color!(0.02, 0.18, 0.18, 1));

	let pts = [
		vec2!(0.0 + CORNER, 0.0 - CORNER),
		vec2!(w.width as f32 - CORNER, 0.0 - CORNER),
		vec2!(w.width as f32 + CORNER, 0.0 + CORNER),
		vec2!(w.width as f32 + CORNER, w.height as f32 - CORNER),
		vec2!(w.width as f32 - CORNER, w.height as f32 + CORNER),
		vec2!(0.0 + CORNER, w.height as f32 + CORNER),
		vec2!(0.0 - CORNER, w.height as f32 - CORNER),
		vec2!(0.0 - CORNER, 0.0 + CORNER),
	];

	gfx::poly(&pts);

	gfx::line(vec2!(0, BAR_HEIGHT), vec2!(w.width, BAR_HEIGHT));

	gfx::push();

	if let Some(id) = ctx.active_window {
		if id == w.id.expect("oh no") {
			gfx::color(color!(1));
		} else {
			gfx::color(color!(0.56, 0.76, 0.76, 1));
		}
	} else {
		gfx::color(color!(0.56, 0.76, 0.76, 1));
	}

	gfx::translate(vec2!(12, 5));
	gfx::scale(vec2!(2));
	gfx::text(&w.title);
	gfx::pop();

	gfx::pop();

}

pub fn add(w: Window) {

	let ctx_mut = ctx_get_mut();
	let windows = &mut ctx_mut.windows;
	let id = ctx_mut.id_generator.get();

	windows.insert(id, w);
	windows.get_mut(&id).expect("failed to add window").id = Some(id);
	ctx_mut.active_window = Some(id);

}

struct Canvas {
	handle: gfx::Canvas,
}

impl Widget for Canvas {
	fn draw(&self) {
		gfx::render(&self.handle);
	}
}

