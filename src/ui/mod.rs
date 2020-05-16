// wengwengweng

use crate::*;
use math::*;

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

	pub fn event(&mut self, ctx: &mut Ctx, e: &input::Event) {

		use input::Event::*;
		use input::Mouse;
		use input::Key;
		use geom::*;

		let mpos = ctx.mouse_pos();
		let t = &self.theme;

		for p in self.windows.values_mut() {
			for w in p.widgets.values_mut() {
				w.event(ctx, e);
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
		ctx: &mut Ctx,
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

		// drawing window frame
		ctx.push_t(mat4!().t2(window.pos), |ctx| {

			// background
			ctx.draw(
				&shapes::rect(vec2!(0), vec2!(window.width, -window.height))
					.fill(theme.background_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			// title bar
			ctx.draw(
				&shapes::rect(vec2!(0), vec2!(window.width, -bar_height))
					.fill(theme.bar_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			// title
			ctx.draw_t(
				mat4!().t2(vec2!(theme.padding, -theme.padding)),
				&shapes::text(&window.title)
					.size(theme.font_size)
					.color(theme.title_color)
					.align(gfx::Origin::TopLeft)
			)?;

			return Ok(());

		})?;

		let width = window.width - theme.padding * 2.0;

		let window_ctx = WindowCtx {
			theme: &self.theme,
			width: width,
			offset: window.pos + vec2!(theme.padding, -bar_height - theme.padding),
		};

		// TODO: overflow: hidden
		ctx.push_t(mat4!().t2(window_ctx.offset), |ctx| {

			let mut wman = WidgetManager {
				widgets: &mut window.widgets,
				cur_y: 0.0,
				ctx: window_ctx,
			};

			f(ctx, &mut wman)?;

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

	fn widget_light<W: Widget>(&mut self, ctx: &mut Ctx, mut w: W) -> Result<()> {

		let mut height = 0.0;

		let wctx = WidgetCtx {
			theme: self.ctx.theme,
			width: self.ctx.width,
			offset: self.ctx.offset + vec2!(0, -self.cur_y),
		};

		ctx.push_t(mat4!().ty(-self.cur_y), |ctx| {
			height = w.draw(ctx, &wctx)?;
			return Ok(());
		})?;

		self.cur_y += height + self.ctx.theme.padding;

		return Ok(());

	}

	fn widget<W: Widget>(&mut self, ctx: &mut Ctx, id: ID, w: impl FnOnce() -> W) -> Result<&W> {

		let mut height = 0.0;

		let w = self.widgets
			.entry(id)
			.or_insert_with(|| box w())
			.as_mut()
			.as_any_mut()
			.downcast_mut::<W>()
			.ok_or(format!("failed to cast widget types"))?;

		let wctx = WidgetCtx {
			theme: self.ctx.theme,
			width: self.ctx.width,
			offset: self.ctx.offset + vec2!(0, -self.cur_y),
		};

		ctx.push_t(mat4!().ty(-self.cur_y), |ctx| {
			height = w.draw(ctx, &wctx)?;
			return Ok(());
		})?;

		self.cur_y += height + self.ctx.theme.padding;

		return Ok(w);

	}

	pub fn text(&mut self, ctx: &mut Ctx, s: &str) -> Result<()> {
		return self.widget_light(ctx, Text::new(s));
	}

	pub fn input(&mut self, ctx: &mut Ctx, prompt: &'static str) -> Result<String> {
		let i = self.widget(ctx, prompt, || Input::new(prompt))?;
		return Ok(i.text());
	}

	pub fn slider(&mut self, ctx: &mut Ctx, prompt: &'static str, val: f32, min: f32, max: f32) -> Result<f32> {
		let s = self.widget(ctx, prompt, || Slider::new(prompt, val, min, max))?;
		return Ok(s.value());
	}

	pub fn button(&mut self, ctx: &mut Ctx, text: &'static str) -> Result<bool> {
		let b = self.widget(ctx, text, || Button::new(text))?;
		return Ok(b.clicked());
	}

	pub fn checkbox(&mut self, ctx: &mut Ctx, prompt: &'static str, b: bool) -> Result<bool> {
		let c = self.widget(ctx, prompt, || CheckBox::new(prompt, b))?;
		return Ok(c.checked());
	}

	pub fn sep(&mut self, ctx: &mut Ctx) -> Result<()> {
		return self.widget_light(ctx, Sep);
	}

	pub fn select(&mut self, ctx: &mut Ctx, prompt: &'static str, options: &[&str], i: usize) -> Result<usize> {
		let s = self.widget(ctx, prompt, || Select::new(prompt, options, i))?;
		return Ok(s.selected());
	}

	// TODO
	pub fn canvas(&mut self, ctx: &mut Ctx, f: impl FnOnce(&mut Ctx, &mut WidgetCtx) -> Result<()>) -> Result<()> {
		return Ok(());
	}

}

