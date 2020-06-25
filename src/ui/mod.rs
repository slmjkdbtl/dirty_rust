// wengwengweng

//! Simple Immediate Mode Debug GUI
//!
//! ```ignore
//! ui.window(d, "test", top_left + vec2!(64, -64), 240.0, 360.0, |ctx, p| {
//!     p.text(ctx, "yo")?;
//!     p.input(ctx, "name")?;
//!     p.slider::<i32>(ctx, "height", 170, 0, 300)?;
//!     p.select(ctx, "gender", &["unknown", "male", "female"], 1)?;
//!     p.checkbox(ctx, "dead", false)?;
//!     p.sep(ctx)?;
//!     p.button(ctx, "explode")?;
//!     return Ok(());
//! })?;
//! ```
//!
//! See [WidgetManager](struct.WidgetManager.html) and [widgets](widgets/index.html) for built in widgets, or [Widget](trait.Widget.html) trait to implement your own widget
//!
//! See [example](https://git.sr.ht/~slmjkdbtl/DIRTY/tree/master/examples/ui.rs) for full usage

pub mod widgets;
export!(widget);
export!(theme);

use std::any::TypeId;
use std::collections::HashMap;
use std::time::Duration;

use crate::*;
use math::*;
use gfx::*;
use input::*;
use widgets::*;

type ID = u64;

/// UI Context
pub struct UI {
	windows: HashMap<ID, Window>,
	theme: Theme,
	draggin: Option<(ID, Vec2)>,
}

impl UI {

	/// create UI context
	pub fn new() -> Self {
		return Self::with_theme(Theme::default());
	}

	/// create with custom [Theme](struct.Theme.html)
	pub fn with_theme(t: Theme) -> Self {
		return Self {
			windows: hmap![],
			theme: t,
			draggin: None,
		};
	}

	/// handle UI events, returns if an event is processed and should stop propagation
	pub fn event(&mut self, d: &mut Ctx, e: &Event) -> bool {

		use Event::*;
		use geom::*;

		let mpos = d.window.mouse_pos();
		let t = &self.theme;

		for p in self.windows.values_mut() {
			for w in p.widgets.values_mut() {
				// TODO: construct WidgetCtx
				if w.event(e) {
					return true;
				}
			}
		}

		match e {

			MouseMove(_) => {
				if let Some((id, offset)) = self.draggin {
					if let Some(window) = self.windows.get_mut(&id) {
						window.pos = mpos + offset;
					}
				}
			},

			MousePress(m) => {

				match m {

					Mouse::Left => {

						if self.draggin.is_none() {

							let bar_height = t.font_size + t.padding * 2.0;

							// TODO: windows should be sorted
							for (id, window) in &self.windows {

								let bar = Rect::new(
									window.pos,
									window.pos + vec2!(window.width, -bar_height),
								);

								if col::intersect2d(mpos, bar) {
									self.draggin = Some((*id, window.pos - mpos));
									return true;
								}

							}

						}

					},

					_ => {},

				}

			},

			MouseRelease(m) => {
				match m {
					Mouse::Left => {
						self.draggin = None;
						return true;
					}
					_ => {},
				}
			},

			_ => {},

		}

		return false;

	}

	/// init a new window, use a callback with [WidgetManager](struct.WidgetManager) to add widgets
	pub fn window(
		&mut self,
		d: &mut Ctx,
		title: &'static str,
		pos: Vec2,
		w: f32,
		h: f32,
		f: impl FnOnce(&mut Ctx, &mut WidgetManager) -> Result<()>,
	) -> Result<()> {

		let window = self.windows
			.entry(hash!(title))
			.or_insert(Window {
				title: title,
				pos: pos,
				width: w,
				height: h,
				widgets: hmap![],
			});

		let t = &self.theme;
		let bar_height = t.font_size + t.padding * 2.0;
		let box_height = window.height + bar_height;
		let view_width = window.width;
		let view_height = window.height;
		let content_width = view_width - t.padding * 2.0;
		let content_offset = vec2!(t.padding, -bar_height - t.padding);

		let d_window = &mut d.window;
		let d_audio = &mut d.audio;
		let d_app = &mut d.app;

		let window_ctx = WindowCtx {
			theme: t,
			content_width: content_width,
			content_offset: content_offset + window.pos,
		};

		// drawing window frame
		d.gfx.push_t(mat4!().t2(window.pos), |gfx| {

			// background
			gfx.draw(
				&shapes::rect(vec2!(0), vec2!(window.width, -box_height))
					.fill(t.background_color)
					.stroke(t.border_color)
					.line_width(t.border_thickness)
			)?;

			// title bar
			gfx.draw(
				&shapes::rect(vec2!(0), vec2!(window.width, -bar_height))
					.fill(t.bar_color)
					.stroke(t.border_color)
					.line_width(t.border_thickness)
			)?;

			// title
			gfx.draw_t(
				mat4!().t2(vec2!(t.padding, -t.padding)),
				&shapes::text(&window.title)
					.size(t.font_size)
					.color(t.title_color)
					.align(Origin::TopLeft)
			)?;

			// widgets
			gfx.push_t(mat4!().t2(content_offset), |gfx| {

				let mut ctx = Ctx {
					window: d_window,
					audio: d_audio,
					app: d_app,
					gfx: gfx,
				};

				let mut wman = WidgetManager {
					widgets: &mut window.widgets,
					cur_y: 0.0,
					window: window_ctx,
				};

				f(&mut ctx, &mut wman)?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		return Ok(());

	}

}

#[derive(Clone)]
struct WindowCtx<'a> {
	theme: &'a Theme,
	content_width: f32,
	content_offset: Vec2,
}

/// Context For A Single Widget
#[derive(Clone)]
pub struct WidgetCtx<'a> {
	mouse_pos: Vec2,
	key_mods: KeyMod,
	window: &'a WindowCtx<'a>,
	time: Duration,
	dt: Duration,
}

impl<'a> WidgetCtx<'a> {
	/// get current mouse pos which origins from the top left corner of current widget
	pub fn mouse_pos(&self) -> Vec2 {
		return self.mouse_pos;
	}
	/// get key mods
	pub fn key_mods(&self) -> KeyMod {
		return self.key_mods;
	}
	/// get current theme
	pub fn theme(&self) -> &Theme {
		return self.window.theme;
	}
	/// get content width
	pub fn width(&self) -> f32 {
		return self.window.content_width;
	}
	/// get time
	pub fn time(&self) -> Duration {
		return self.time;
	}
	/// get dt
	pub fn dt(&self) -> Duration {
		return self.dt;
	}
}

struct Window {
	pos: Vec2,
	title: &'static str,
	width: f32,
	height: f32,
	widgets: HashMap<ID, Box<dyn Widget>>,
}

/// Manager for Create Widgets
pub struct WidgetManager<'a> {
	widgets: &'a mut HashMap<ID, Box<dyn Widget>>,
	cur_y: f32,
	window: WindowCtx<'a>,
}

impl<'a> WidgetManager<'a> {

	/// add a widget with no persistent state
	pub fn widget_light<W: Widget>(&mut self, d: &mut Ctx, mut w: W) -> Result<()> {

		let mut height = 0.0;
		let offset = self.window.content_offset + vec2!(0, -self.cur_y);

		let wctx = WidgetCtx {
			window: &self.window,
			mouse_pos: d.window.mouse_pos() - offset,
			key_mods: d.window.key_mods(),
			time: d.app.time(),
			dt: d.app.dt(),
		};

		d.gfx.push_t(mat4!().ty(-self.cur_y), |gfx| {
			height = w.draw(gfx, &wctx)?;
			return Ok(());
		})?;

		self.cur_y += height + self.window.theme.padding;

		return Ok(());

	}

	/// add a widget with persistent state
	pub fn widget<O, W: Widget>(
		&mut self,
		d: &mut Ctx,
		id: ID,
		w: impl FnOnce() -> W,
		f: impl FnOnce(&W) -> O
	) -> Result<O> {

		let mut height = 0.0;
		let val;
		let id = hash!(TypeId::of::<W>(), id);

		let w = self.widgets
			.entry(id)
			.or_insert_with(|| Box::new(w()))
			.as_mut()
			.as_any_mut()
			.downcast_mut::<W>()
			.ok_or(format!("failed to cast widget types"))?;

		let offset = self.window.content_offset + vec2!(0, -self.cur_y);

		let wctx = WidgetCtx {
			window: &self.window,
			mouse_pos: d.window.mouse_pos() - offset,
			key_mods: d.window.key_mods(),
			time: d.app.time(),
			dt: d.app.dt(),
		};

		val = Ok(f(w));

		d.gfx.push_t(mat4!().ty(-self.cur_y), |gfx| {
			height = w.draw(gfx, &wctx)?;
			return Ok(());
		})?;

		self.cur_y += height + self.window.theme.padding;

		return val;

	}

	pub fn text(&mut self, d: &mut Ctx, s: &str) -> Result<()> {
		return self.widget_light(d, Text::new(s));
	}

	pub fn input(&mut self, d: &mut Ctx, label: &'static str) -> Result<String> {
		return self.widget(d, hash!(label), || Input::new(label), |i| {
			return i.text();
		});
	}

	pub fn slider<T: SliderValue>(
		&mut self,
		d: &mut Ctx,
		label: &'static str,
		val: T,
		min: T,
		max: T
	) -> Result<T> {
		return self.widget(d, hash!(label), || Slider::new(label, val, min, max), |i| {
			return i.value();
		});
	}

	pub fn button(&mut self, d: &mut Ctx, text: &'static str) -> Result<bool> {
		return self.widget(d, hash!(text), || Button::new(text), |i| {
			return i.clicked();
		});
	}

	pub fn checkbox(&mut self, d: &mut Ctx, label: &'static str, b: bool) -> Result<bool> {
		return self.widget(d, hash!(label), || CheckBox::new(label, b), |i| {
			return i.checked();
		});
	}

	pub fn sep(&mut self, d: &mut Ctx) -> Result<()> {
		return self.widget_light(d, Sep);
	}

	pub fn select(&mut self, d: &mut Ctx, label: &'static str, options: &[&str], i: usize) -> Result<usize> {
		return self.widget(d, hash!(label), || Select::new(label, options, i), |i| {
			return i.selected();
		});
	}

	// TODO
	pub fn canvas(&mut self, d: &mut Ctx, f: impl FnOnce(&mut Ctx, &mut WidgetCtx) -> Result<()>) -> Result<()> {
		return Ok(());
	}

}

