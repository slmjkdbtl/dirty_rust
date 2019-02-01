// wengwengweng

//! Simple UI

// A tribute to MEKA (http://www.smspower.org/meka/), thanks for making awesome
// tools

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
	theme: Theme,
	background_buffer: gfx::Canvas,

}

/// initialize ui module
pub fn init() {

	let (width, height) = window::size();

	ctx_init(UICtx {

		windows: BTreeMap::new(),
		id_generator: IdGenerator::new(),
		active_window: None,
		dragging_window: None,
		background_buffer: gfx::Canvas::new(width, height),
		theme: Theme::default(),

	});

	set_background(|| {
		g2d::color(color!(0.6, 0.78, 0.78, 1));
		g2d::rect(vec2!(width, height));
	});

}

/// set current theme
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

	let (width, height) = window::size();
	let ctx = ctx_get();
	let ctx_mut = ctx_get_mut();

	g2d::push();
	g2d::reset();

	gfx::render(&ctx.background_buffer);

	for (_, w) in ctx_mut.windows.iter_mut().rev() {
		update_window(w);
	}

	for (_, w) in &ctx_mut.windows {
		draw_window(w);
	}

	g2d::pop();

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

pub fn set_background<F: FnMut()>(mut f: F) {

	let ui = ctx_get();

	gfx::drawon(&ui.background_buffer);
	f();
	gfx::stop_drawon(&ui.background_buffer);

}

fn draw_window(w: &Window) {

	let ctx = ctx_get();
	let theme = &ctx.theme;

// 	gfx::drawon(&w.buffer);
	g2d::push();

		g2d::translate(w.pos);

		// draw background
		g2d::color(theme.back);
		g2d::rect(vec2!(w.width, w.height));

		// draw headbar
		g2d::color(theme.bar);
		g2d::rect(vec2!(w.width, BAR_HEIGHT));

		// draw outlines
		g2d::line_width(3);
		g2d::color(theme.line);
		g2d::poly(&utils::rounded_rect(w.width, w.height, CORNER));
		g2d::line(vec2!(0, BAR_HEIGHT), vec2!(w.width, BAR_HEIGHT));

		// draw title
		g2d::push();

			if let Some(id) = ctx.active_window {
				if id == w.id.expect("oh no") {
					g2d::color(theme.text_active);
				} else {
					g2d::color(theme.text_passive);
				}
			} else {
				g2d::color(theme.text_passive);
			}

			g2d::translate(vec2!(14, 5));
			g2d::scale(vec2!(2));
			g2d::text(&w.title);

		g2d::pop();

		// draw widgets
		g2d::push();

			g2d::translate(vec2!(0, BAR_HEIGHT));
			g2d::translate(vec2!(2, 3));

			for widget in &w.widgets {
				widget.draw();
			}

		g2d::pop();

	g2d::pop();
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

