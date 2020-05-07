// wengwengweng

use std::collections::HashMap;

use super::*;

pub type ID = &'static str;

pub struct UI {
	panels: HashMap<ID, Panel>,
	theme: Theme,
}

impl UI {

	pub fn new() -> Self {
		return Self {
			panels: hmap![],
			theme: Theme::default(),
		};
	}

	pub fn event(&mut self, e: &input::Event) {
		// ...
	}

	pub fn frame(&mut self, f: impl FnOnce(&mut PanelManager) -> Result<()>) -> Result<()> {
		let mut pman = PanelManager {
			panels: &mut self.panels,
			theme: &self.theme,
		};
		f(&mut pman)?;
		return Ok(());
	}

}

pub struct PanelManager<'a> {
	panels: &'a mut HashMap<&'static str, Panel>,
	theme: &'a Theme,
}

impl<'a> PanelManager<'a> {

	pub fn panel(
		&mut self,
		ctx: &mut Ctx,
		title: &'static str,
		pos: Vec2,
		w: f32,
		h: f32,
		f: impl FnOnce(&mut Ctx, &mut WidgetManager) -> Result<()>,
	) -> Result<()> {

		let panel = self.panels.entry(title).or_insert(Panel {
			title: String::from(title),
			pos: pos,
			width: w,
			height: h,
			widgets: hmap![],
		});

		let theme = &self.theme;
		let bar_height = theme.font_size + theme.padding.y;

		// drawing panel frame
		ctx.push_t(mat4!().t2(panel.pos), |ctx| {

			ctx.draw(
				&shapes::rect(vec2!(0), vec2!(panel.width, -panel.height))
					.fill(theme.background_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			ctx.draw(
				&shapes::rect(vec2!(0), vec2!(panel.width, -bar_height))
					.fill(theme.bar_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			ctx.draw_t(
				mat4!().t2(vec2!(theme.padding.x, -theme.padding.y / 2.0)),
				&shapes::text(&panel.title)
					.size(theme.font_size)
					.color(theme.title_color)
					.align(gfx::Origin::TopLeft)
			)?;

			return Ok(());

		})?;

		let width = panel.width - theme.padding.x * 2.0 - theme.border_thickness * 2.0;

		let panel_ctx = PanelCtx {
			theme: &self.theme,
			width: width,
		};

		ctx.push_t(mat4!().t2(panel.pos).ty(-bar_height).t2(theme.padding * vec2!(1, -1)), |ctx| {
			let mut wman = WidgetManager {
				widgets: &mut panel.widgets,
				cur_y: 0.0,
				ctx: panel_ctx,
			};
			f(ctx, &mut wman)?;
			return Ok(());
		})?;

		return Ok(());

	}

}

#[derive(Clone)]
pub struct PanelCtx<'a> {
	pub theme: &'a Theme,
	pub width: f32,
}

pub struct Panel {
	pos: Vec2,
	title: String,
	width: f32,
	height: f32,
	widgets: HashMap<ID, Box<dyn Widget>>,
}

pub struct WidgetManager<'a> {
	widgets: &'a mut HashMap<ID, Box<dyn Widget>>,
	cur_y: f32,
	ctx: PanelCtx<'a>,
}

impl<'a> WidgetManager<'a> {

	fn widget_light<W: Widget>(&mut self, ctx: &mut Ctx, w: W) -> Result<()> {

		let mut height = 0.0;

		ctx.push_t(mat4!().ty(-self.cur_y), |ctx| {
			height = w.draw(ctx, &self.ctx)?;
			return Ok(());
		})?;

		self.cur_y += height + self.ctx.theme.margin;

		return Ok(());

	}

	fn widget<O, W: Widget>(&mut self, ctx: &mut Ctx, id: ID, w: W, f: impl FnOnce(&W) -> O) -> Result<O> {

		let pctx = self.ctx.clone();
		let mut height = 0.0;
		let val;

		let w = self.widgets
			.entry(id)
			.or_insert_with(|| box w)
			.as_mut()
			.as_any_mut()
			.downcast_mut::<W>()
			.ok_or(format!("failed to cast widget types"))?;

		ctx.push_t(mat4!().ty(-self.cur_y), |ctx| {
			height = w.draw(ctx, &pctx)?;
			return Ok(());
		})?;

		val = Ok(f(w));
		self.cur_y += height + pctx.theme.margin;

		return val;

	}

	pub fn text(&mut self, ctx: &mut Ctx, s: &str) -> Result<()> {
		return self.widget_light(ctx, Text::new(s));
	}

	pub fn input(&mut self, ctx: &mut Ctx, prompt: &'static str) -> Result<String> {
		return self.widget(ctx, prompt, Input::new(prompt), |i| {
			return i.text();
		});
	}

}

