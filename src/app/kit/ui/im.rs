// wengwengweng

use std::collections::HashMap;
use std::any::Any;

use super::*;

pub type ID = usize;

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

	pub fn event(&mut self, e: &app::input::Event) {
		// ...
	}

	pub fn frame(&mut self, ctx: &mut app::Ctx, f: impl FnOnce(&mut PanelManager)) {
		let mut pman = PanelManager {
			panels: &mut self.panels,
			ctx: ctx,
			theme: &self.theme,
		};
		f(&mut pman);
	}

}

pub struct PanelManager<'a> {
	panels: &'a mut HashMap<ID, Panel>,
	ctx: &'a mut app::Ctx,
	theme: &'a Theme,
}

impl<'a> PanelManager<'a> {

	pub fn panel(
		&mut self,
		id: ID,
		title: &str,
		pos: Vec2,
		w: f32,
		h: f32,
		f: impl FnOnce(&mut WidgetManager),
	) -> Result<()> {

		let panel = self.panels.entry(id).or_insert(Panel {
			title: String::from(title),
			pos: pos,
			width: w,
			height: h,
			widgets: hmap![],
		});

		let ctx = &mut self.ctx;
		let theme = &self.theme;
		let bar_height = theme.font_size + theme.padding.y;

		ctx.push(mat4!().t2(panel.pos), |ctx| {

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

// 		ctx.push(mat4!().t2(panel.pos).ty(-bar_height), |ctx| {
// 			let mut wman = WidgetManager {
// 				widgets: &mut panel.widgets,
// 				ctx: ctx,
// 				theme: &self.theme,
// 			};
// 			f(&mut wman);
// 			return Ok(());
// 		});

		return Ok(());

	}

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
	ctx: &'a mut app::Ctx,
	theme: &'a Theme,
}

pub struct Input {
	buf: String,
}

impl Widget for Input {
}

pub struct Slider<T> {
	val: T,
	min: T,
	max: T,
}

impl<T: 'static> Widget for Slider<T> {
}

pub struct Menu {
	items: HashMap<ID, String>,
	cur: ID,
}

impl Widget for Menu {
	// ...
}

impl<'a> WidgetManager<'a> {

	pub fn add<W: Widget>(&mut self, id: ID, w: W) -> Option<&mut W> {

		let b = self.widgets
			.entry(id)
			.or_insert_with(|| box w);

		return b.as_mut().as_any_mut().downcast_mut::<W>();

	}

	pub fn input(&mut self, id: ID) -> &str {

		if let Some(input) = self.add(id, Input {
			buf: String::new(),
		}) {
			return &input.buf;
		}

		return "";

	}

	pub fn slider<T: PartialEq + Copy + 'static>(&mut self, id: ID, val: T, min: T, max: T) -> T {

		if let Some(slider) = self.add(id, Slider {
			val: val,
			min: min,
			max: max,
		}) {
			return slider.val;
		}

		return val;

	}

	pub fn menu(&mut self, id: ID, f: impl FnOnce(&mut MenuManager)) {
	}

}

pub struct MenuManager {}

impl MenuManager {
	pub fn item(&mut self, id: ID, text: &str) -> bool {
		return false;
	}
}

pub trait AsAny {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

pub trait Widget: AsAny + 'static {
	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {
		return Ok(());
	}
}

