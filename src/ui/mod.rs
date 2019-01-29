// wengwengweng

//! Simple UI

// Style heavily inspired by MEKA (http://www.smspower.org/meka/), thanks for making awesome
// tools ocornut

use std::collections::BTreeMap;

use crate::*;
use crate::math::*;
use crate::addons::col;

use crate::utils::id::*;

mod widget;
mod canvas;
mod button;
mod text_box;
mod theme;
mod utils;

pub use widget::*;
pub use canvas::*;
pub use button::*;
pub use text_box::*;
pub use theme::*;

const BAR_HEIGHT: u32 = 42;
const CORNER: f32 = 1.4;

ctx!(UI: UICtx);

struct UICtx {

	windows: BTreeMap<Id, Window>,
	id_generator: IdGenerator,
	active_window: Option<Id>,
	dragging_window: Option<Id>,
	buffer: gfx::Canvas,
	theme: Theme,

}

/// initialize ui module
pub fn init() {

	let (width, height) = window::size();

	ctx_init(UICtx {

		windows: BTreeMap::new(),
		id_generator: IdGenerator::new(),
		active_window: None,
		dragging_window: None,
		buffer: gfx::Canvas::new(width, height),
		theme: Theme::default(),

	});

}

pub fn set_theme(t: Theme) {
	ctx_get_mut().theme = t;
}

enum WindowState {
	Idle,
	Dragged(Vec2),
}

/// widget container
pub struct Window {

	title: String,
	pos: Vec2,
	width: u32,
	height: u32,
	state: WindowState,
	widgets: Vec<Box<Widget>>,
	id: Option<Id>,
	buffer: gfx::Canvas,

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
			buffer: gfx::Canvas::new(width, height),

		};

	}

	pub fn add<W: Widget>(&mut self, w: W) {
		self.widgets.push(Box::new(w));
	}

}

/// draw every window and widgets
pub fn draw() {

	let ctx_mut = ctx_get_mut();

	gfx::push();
	gfx::reset();

	for (_, w) in ctx_mut.windows.iter_mut().rev() {
		update_window(w);
	}

	for (_, w) in &ctx_mut.windows {
		draw_window(w);
	}

	gfx::pop();

}

fn update_window(w: &mut Window) {

	let ctx = ctx_get();
	let mpos = window::mouse_pos();

	if ctx.dragging_window.is_none() {

		if window::mouse_pressed(Mouse::Left) {

			if col::point_rect(mpos, rect!(w.pos.x, w.pos.y, w.width, BAR_HEIGHT)) {

				let ctx_mut = ctx_get_mut();
				let id = w.id.expect("oh no");

				w.state = WindowState::Dragged(mpos - w.pos);
				ctx_mut.active_window = Some(id);
				ctx_mut.dragging_window = Some(id);
				add(remove(id).expect("oh no"));

			}

		}

	} else {

		if let WindowState::Dragged(pos) = w.state {

			let mut still_dragged = true;

			if window::mouse_released(Mouse::Left) {

				let ctx_mut = ctx_get_mut();

				still_dragged = false;
				ctx_mut.dragging_window = None;
				w.state = WindowState::Idle;

			}

			if still_dragged {
				w.pos = mpos - pos;
			}

		}

	}

	for widget in &mut w.widgets {
		widget.update();
	}

}

fn draw_window(w: &Window) {

	let ctx = ctx_get();
	let theme = &ctx.theme;

// 	gfx::drawon(&w.buffer);
	gfx::push();

		gfx::translate(w.pos);

		// draw background
		gfx::color(theme.back);
		gfx::rect(vec2!(w.width, w.height));

		// draw headbar
		gfx::color(theme.bar);
		gfx::rect(vec2!(w.width, BAR_HEIGHT));

		// draw outlines
		gfx::line_width(3);
		gfx::color(theme.line);
		gfx::poly(&utils::rounded_rect(w.width, w.height, CORNER));
		gfx::line(vec2!(0, BAR_HEIGHT), vec2!(w.width, BAR_HEIGHT));

		// draw title
		gfx::push();

			if let Some(id) = ctx.active_window {
				if id == w.id.expect("oh no") {
					gfx::color(theme.text_active);
				} else {
					gfx::color(theme.text_passive);
				}
			} else {
				gfx::color(theme.text_passive);
			}

			gfx::translate(vec2!(14, 5));
			gfx::scale(vec2!(2));
			gfx::text(&w.title);

		gfx::pop();

		// draw widgets
		gfx::push();

			gfx::translate(vec2!(0, BAR_HEIGHT));
			gfx::translate(vec2!(2, 3));

			for widget in &w.widgets {
				widget.draw();
			}

		gfx::pop();

	gfx::pop();
// 	gfx::stop_drawon(&w.buffer);
// 	gfx::render(&w.buffer);

}

/// add a window
pub fn add(w: Window) {

	let ctx_mut = ctx_get_mut();
	let windows = &mut ctx_mut.windows;
	let id = ctx_mut.id_generator.get();

	windows.insert(id, w);
	windows.get_mut(&id).expect("failed to add window").id = Some(id);
	ctx_mut.active_window = Some(id);

}

/// remove a window
pub fn remove(id: Id) -> Option<Window> {
	return ctx_get_mut().windows.remove(&id);
}

