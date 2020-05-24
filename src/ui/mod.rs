// wengwengweng

//! A Simple Immediate Mode GUI Lib

use crate::*;
use math::*;
use gfx::shapes;

export!(widget);
export!(theme);
export!(tinput);
export!(text);
export!(slider);
export!(button);
export!(checkbox);
export!(sep);
export!(select);

use std::collections::HashMap;

pub type ID = &'static str;

pub struct UI {
	windows: HashMap<ID, Window>,
	theme: Theme,
	draggin: Option<(ID, Vec2)>,
}

impl UI {

	pub fn new() -> Self {
		return Self {
			windows: hmap![],
			theme: Theme::default(),
			draggin: None,
		};
	}

	pub fn event(&mut self, d: &mut Ctx, e: &input::Event) {

		use input::Event::*;
		use input::Mouse;
		use input::Key;
		use geom::*;

		let mpos = d.window.mouse_pos();
		let t = &self.theme;

		for p in self.windows.values_mut() {
			for w in p.widgets.values_mut() {
				w.event(d, e);
			}
		}

		match e {

			MouseMove(_) => {
				if let Some((id, offset)) = self.draggin {
					if let Some(window) = self.windows.get_mut(id) {
						window.pos = mpos + offset;
					}
				}
			},

			MousePress(m) => {

				match m {

					Mouse::Left => {

						if self.draggin.is_none() {

							// TODO: windows should be sorted
							for (id, window) in &self.windows {

								let bar_height = t.font_size + t.padding;

								let bar = Rect::new(
									window.pos,
									window.pos + vec2!(window.width, -bar_height),
								);

								if col::intersect2d(mpos, bar) {
									self.draggin = Some((id, window.pos - mpos));
									break;
								}

							}

						}

					},

					_ => {},

				}

			},

			MouseRelease(m) => {

				match m {

					Mouse::Left => self.draggin = None,
					_ => {},

				}

			},

			_ => {},

		}

	}

	pub fn window(
		&mut self,
		d: &mut Ctx,
		title: &'static str,
		pos: Vec2,
		w: f32,
		h: f32,
		f: impl FnOnce(&mut Ctx, &mut WidgetManager) -> Result<()>,
	) -> Result<()> {

		let window = self.windows.entry(title).or_insert(Window {
			title: title,
			pos: pos,
			width: w,
			height: h,
			widgets: hmap![],
		});

		let theme = &self.theme;
		let bar_height = theme.font_size + theme.padding * 2.0;
		let out_height = window.height + bar_height;
		let view_width = window.width;
		let view_height = window.height;

		// drawing window frame
		d.gfx.push_t(mat4!().t2(window.pos), |gfx| {

			// background
			gfx.draw(
				&shapes::rect(vec2!(0), vec2!(window.width, -window.height))
					.fill(theme.background_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			// title bar
			gfx.draw(
				&shapes::rect(vec2!(0), vec2!(window.width, -bar_height))
					.fill(theme.bar_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			// title
			gfx.draw_t(
				mat4!().t2(vec2!(theme.padding, -theme.padding)),
				&shapes::text(&window.title)
					.size(theme.font_size)
					.color(theme.title_color)
					.align(gfx::Origin::TopLeft)
			)?;

			return Ok(());

		})?;

		let width = window.width - theme.padding * 2.0;
		let offset = window.pos + vec2!(theme.padding, -bar_height - theme.padding);

		let window_ctx = WindowCtx {
			theme: &self.theme,
			width: width,
			offset: offset,
		};

		let dwindow = &mut d.window;
		let daudio = &mut d.audio;
		let dapp = &mut d.app;

		// TODO: overflow: hidden
		d.gfx.push_t(mat4!().t2(window_ctx.offset), |gfx| {

			// TODO: ???
			let mut ctx = Ctx {
				window: dwindow,
				audio: daudio,
				app: dapp,
				gfx: gfx,
			};

			let mut wman = WidgetManager {
				widgets: &mut window.widgets,
				cur_y: 0.0,
				ctx: window_ctx,
			};

			f(&mut ctx, &mut wman)?;

			return Ok(());

		})?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct WindowCtx<'a> {
	pub theme: &'a Theme,
	pub width: f32,
	pub offset: Vec2,
}

#[derive(Clone)]
pub struct WidgetCtx<'a> {
	pub theme: &'a Theme,
	pub width: f32,
	pub offset: Vec2,
	pub mouse_pos: Vec2,
}

pub struct Window {
	pos: Vec2,
	title: &'static str,
	width: f32,
	height: f32,
	widgets: HashMap<ID, Box<dyn Widget>>,
}

pub struct WidgetManager<'a> {
	widgets: &'a mut HashMap<ID, Box<dyn Widget>>,
	cur_y: f32,
	ctx: WindowCtx<'a>,
}

impl<'a> WidgetManager<'a> {

	fn widget_light<W: Widget>(&mut self, d: &mut Ctx, mut w: W) -> Result<()> {

		let mut height = 0.0;
		let offset = self.ctx.offset + vec2!(0, -self.cur_y);

		let wctx = WidgetCtx {
			theme: self.ctx.theme,
			width: self.ctx.width,
			offset: offset,
			mouse_pos: d.window.mouse_pos() - offset,
		};

		d.gfx.push_t(mat4!().ty(-self.cur_y), |gfx| {
			height = w.draw(gfx, &wctx)?;
			return Ok(());
		})?;

		self.cur_y += height + self.ctx.theme.padding;

		return Ok(());

	}

	fn widget<O, W: Widget>(
		&mut self,
		d: &mut Ctx,
		id: ID,
		w: impl FnOnce() -> W,
		f: impl FnOnce(&W) -> O
	) -> Result<O> {

		let mut height = 0.0;
		let val;

		let w = self.widgets
			.entry(id)
			.or_insert_with(|| Box::new(w()))
			.as_mut()
			.as_any_mut()
			.downcast_mut::<W>()
			.ok_or(format!("failed to cast widget types"))?;

		let offset = self.ctx.offset + vec2!(0, -self.cur_y);

		let wctx = WidgetCtx {
			theme: self.ctx.theme,
			width: self.ctx.width,
			offset: offset,
			mouse_pos: d.window.mouse_pos() - offset,
		};

		val = Ok(f(w));

		d.gfx.push_t(mat4!().ty(-self.cur_y), |gfx| {
			height = w.draw(gfx, &wctx)?;
			return Ok(());
		})?;

		self.cur_y += height + self.ctx.theme.padding;

		return val;

	}

	pub fn text(&mut self, d: &mut Ctx, s: &str) -> Result<()> {
		return self.widget_light(d, Text::new(s));
	}

	pub fn input(&mut self, d: &mut Ctx, prompt: &'static str) -> Result<String> {
		return self.widget(d, prompt, || Input::new(prompt), |i| {
			return i.text();
		});
	}

	pub fn slider(&mut self, d: &mut Ctx, prompt: &'static str, val: f32, min: f32, max: f32) -> Result<f32> {
		return self.widget(d, prompt, || Slider::new(prompt, val, min, max), |i| {
			return i.value();
		});
	}

	pub fn button(&mut self, d: &mut Ctx, text: &'static str) -> Result<bool> {
		return self.widget(d, text, || Button::new(text), |i| {
			return i.clicked();
		});
	}

	pub fn checkbox(&mut self, d: &mut Ctx, prompt: &'static str, b: bool) -> Result<bool> {
		return self.widget(d, prompt, || CheckBox::new(prompt, b), |i| {
			return i.checked();
		});
	}

	pub fn sep(&mut self, d: &mut Ctx) -> Result<()> {
		return self.widget_light(d, Sep);
	}

	pub fn select(&mut self, d: &mut Ctx, prompt: &'static str, options: &[&str], i: usize) -> Result<usize> {
		return self.widget(d, prompt, || Select::new(prompt, options, i), |i| {
			return i.selected();
		});
	}

	// TODO
	pub fn canvas(&mut self, d: &mut Ctx, f: impl FnOnce(&mut Ctx, &mut WidgetCtx) -> Result<()>) -> Result<()> {
		return Ok(());
	}

}

