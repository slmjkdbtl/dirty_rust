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

	pub fn frame(&mut self, ctx: &mut Ctx, f: impl FnOnce(&mut PanelManager)) {
		let mut pman = PanelManager {
			panels: &mut self.panels,
			ctx: ctx,
			theme: &self.theme,
		};
		f(&mut pman);
	}

}

pub struct PanelManager<'a> {
	panels: &'a mut HashMap<&'static str, Panel>,
	ctx: &'a mut Ctx,
	theme: &'a Theme,
}

impl<'a> PanelManager<'a> {

	pub fn panel(
		&mut self,
		title: &'static str,
		pos: Vec2,
		w: f32,
		h: f32,
		f: impl FnOnce(&mut WidgetManager),
	) -> Result<()> {

		let panel = self.panels.entry(title).or_insert(Panel {
			title: String::from(title),
			pos: pos,
			width: w,
			height: h,
		});

		let ctx = &mut self.ctx;
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

		let theme = self.theme.clone();

		ctx.push_t(mat4!().t2(panel.pos).ty(-bar_height).t2(theme.padding * vec2!(1, -1)), |ctx| {
			let mut wman = WidgetManager {
				ctx: ctx,
				theme: &theme,
				cur_y: 0.0,
			};
			f(&mut wman);
			return Ok(());
		})?;

		return Ok(());

	}

}

pub struct Panel {
	pos: Vec2,
	title: String,
	width: f32,
	height: f32,
}

pub struct WidgetManager<'a> {
	ctx: &'a mut Ctx,
	theme: &'a Theme,
	cur_y: f32,
}

impl<'a> WidgetManager<'a> {

	pub fn text(&mut self, s: &str) -> Result<()> {

		let t = Text::new(s);
		let theme = self.theme.clone();
		let mut height = 0.0;

		self.ctx.push_t(mat4!().ty(-self.cur_y), |ctx| {
			height = t.draw(ctx, &theme)?;
			return Ok(());
		})?;

		self.cur_y += height + theme.margin;

		return Ok(());

	}

	pub fn input(&mut self, prompt: &'static str) -> Result<()> {
		return Ok(());
	}

}

pub trait Widget {
	fn draw(&self, _: &mut Ctx, _: &Theme) -> Result<f32> {
		return Ok(0.0);
	}
}

