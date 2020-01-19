// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
	pub bar_color: Color,
	pub bar_height: f32,
	pub border_color: Color,
	pub border_thickness: f32,
	pub background_color: Color,
	pub title_color: Color,
	pub title_color_selected: Color,
	pub padding: Vec2,
}

impl Default for Theme {
	fn default() -> Self {
		return Self {
			bar_color: rgba!(0, 0.51, 0.51, 1),
			bar_height: 24.0,
			border_color: rgba!(0.02, 0.18, 0.18, 1),
			border_thickness: 2.0,
			background_color: rgba!(0, 0.35, 0.35, 1),
			title_color: rgba!(0.6, 0.78, 0.78, 1),
			title_color_selected: rgba!(1),
			padding: vec2!(6),
		};
	}
}

#[derive(Clone, Copy, Debug)]
pub struct PanelCtx {
	pub width: f32,
	pub pos: Vec2,
}

pub struct Panel {
	pos: Vec2,
	title: String,
	width: f32,
	height: f32,
	theme: Theme,
}

impl Panel {

	pub fn new(title: &str, pos: Vec2, width: f32, height: f32) -> Self {
		return Self {
			pos: pos,
			title: String::from(title),
			width: width,
			height: height,
			theme: Theme::default(),
		};
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

			ctx.draw(
				&shapes::rect(vec2!(0), vec2!(self.width, -theme.bar_height))
					.fill(theme.bar_color)
					.stroke(theme.border_color)
					.line_width(theme.border_thickness)
			)?;

			ctx.draw_t(
				&gfx::t().t2(vec2!(8, -4)),
				&shapes::text(&self.title)
					.size(theme.bar_height - 8.0)
					.color(theme.title_color)
					.align(gfx::Origin::TopLeft)
			)?;

			ctx.push(&gfx::t().t2(vec2!(theme.padding.x, -theme.padding.y - theme.bar_height)), |ctx| {

				let info = PanelCtx {
					width: self.width,
					pos: self.pos,
				};

				let mut y = 0.0;

				for w in widgets {

					ctx.push(&gfx::t().t2(vec2!(0, y)), |ctx| {

						let res = w.draw(ctx, &info)?;

						y += res.height;

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

