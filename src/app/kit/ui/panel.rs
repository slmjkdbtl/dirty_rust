// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
	pub bar_color: Color,
	pub border_color: Color,
	pub border_thickness: f32,
	pub background_color: Color,
	pub title_color: Color,
	pub title_color_selected: Color,
	pub padding: Vec2,
	pub margin: f32,
	pub font_size: f32,
}

impl Default for Theme {
	fn default() -> Self {
		return Self {
			bar_color: rgba!(0, 0.51, 0.51, 1),
			border_color: rgba!(0.02, 0.18, 0.18, 1),
			border_thickness: 2.0,
			background_color: rgba!(0, 0.35, 0.35, 1),
			title_color: rgba!(0.6, 0.78, 0.78, 1),
			title_color_selected: rgba!(1),
			padding: vec2!(12),
			margin: 12.0,
			font_size: 13.0,
		};
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PanelCtx<'a> {
	pub width: f32,
	pub pos: Vec2,
	pub theme: &'a Theme,
}

pub struct Panel {
	pos: Vec2,
	title: String,
	width: f32,
	height: f32,
	theme: Theme,
	draggin: Option<DragginCtx>,
}

struct DragginCtx {
	dpos: Vec2,
}

impl Panel {

	pub fn new(title: &str, pos: Vec2, width: f32, height: f32) -> Self {
		return Self {
			pos: pos,
			title: String::from(title),
			width: width,
			height: height,
			theme: Theme::default(),
			draggin: None,
		};
	}

	pub fn event(&mut self, ctx: &mut app::Ctx, e: &app::input::Event, widgets: &mut [&mut dyn Widget]) -> Result<()> {

		use app::input::Event::*;
		use app::input::Mouse;
		use app::kit::geom;
		use geom::Shape2D;

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left => {
						let mpos = ctx.mouse_pos();
						let bar_height = self.theme.font_size + self.theme.padding.y;
						if geom::overlaps(Shape2D::Point(mpos), Shape2D::Rect(self.pos, self.pos + vec2!(self.width, -bar_height))) {
							self.draggin = Some(DragginCtx {
								dpos: self.pos - mpos,
							});
						}
					},
					_ => {},
				}
			},
			MouseRelease(m) => {
				match *m {
					Mouse::Left => {
						self.draggin = None;
					},
					_ => {},
				}
			},
			_ => {},
		}

		let mut y = 0.0;
		let bar_height = self.theme.font_size + self.theme.padding.y;

		y += bar_height;
		y += self.theme.padding.y;

		for w in widgets {

			let pctx = PanelCtx {
				width: self.width,
				pos: self.pos + vec2!(self.theme.padding.x, -y),
				theme: &self.theme,
			};

			y += self.theme.margin + w.height(&self.theme);
			w.event(ctx, &pctx, e);

		}

		return Ok(());

	}

	pub fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if let Some(draggin) = &self.draggin {
			self.pos = ctx.mouse_pos + draggin.dpos;
		}

		return Ok(());

	}

	pub fn draw(&self, ctx: &mut app::Ctx, widgets: &[&dyn Widget]) -> Result<()> {

		let theme = &self.theme;

		ctx.push(&gfx::t().t2(self.pos), |ctx| {

			ctx.draw(
				&shapes::rect(vec2!(0), vec2!(self.width, -self.height))
					.fill(theme.background_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			let bar_height = self.theme.font_size + self.theme.padding.y;

			ctx.draw(
				&shapes::rect(vec2!(0), vec2!(self.width, -bar_height))
					.fill(theme.bar_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			ctx.draw_t(
				&gfx::t().t2(vec2!(theme.padding.x, -theme.padding.y / 2.0)),
				&shapes::text(&self.title)
					.size(theme.font_size)
					.color(theme.title_color)
					.align(gfx::Origin::TopLeft)
			)?;

			ctx.push(&gfx::t().t2(vec2!(theme.padding.x, -theme.padding.y - bar_height)), |ctx| {

				let mut y = 0.0;

				for w in widgets {

					ctx.push(&gfx::t().t2(vec2!(0, -y)), |ctx| {

						let pctx = PanelCtx {
							width: self.width,
							pos: vec2!(0),
							theme: &self.theme,
						};

						w.draw(ctx, &pctx)?;
						y += w.height(&self.theme) + theme.margin;

						return Ok(());

					})?;

				}

				return Ok(());

			})?;

			return Ok(());

		})?;

		return Ok(());
	}

}

