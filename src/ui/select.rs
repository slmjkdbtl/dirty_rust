// wengwengweng

use super::*;

pub struct Select {
	prompt: &'static str,
	options: Vec<String>,
	selected: usize,
	state: State,
}

enum State {
	Expanded(Option<usize>),
	Idle(bool),
}

impl Select {
	pub fn new(prompt: &'static str, options: &[&str], i: usize) -> Self {
		return Self {
			prompt: prompt,
			options: options.iter().map(|s| s.to_string()).collect(),
			selected: i,
			state: State::Idle(false),
		};
	}
	pub fn selected(&self) -> usize {
		return self.selected;
	}
}

impl Widget for Select {

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) {

		use input::Event::*;
		use input::Key;
		use input::Mouse;

		let kmods = ctx.key_mods();

		match e {
			MousePress(m) => {
				match *m {
					Mouse::Left => {

						match self.state {
							State::Idle(b) => {
								if b {
									self.state = State::Expanded(None);
								}
							},
							State::Expanded(b) => {
								self.state = State::Idle(false);
								if let Some(i) = b {
									self.selected = i;
								}
							},
						}

					},
					_ => {},
				}
			},
			_ => {},
		}

	}

	fn draw(&mut self, ctx: &mut Ctx, wctx: &WidgetCtx) -> Result<f32> {

		use geom::*;

		let mut y = 0.0;
		let theme = &wctx.theme;

		let ptext = shapes::text(&format!("{}:", self.prompt))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		ctx.draw_t(mat4!().ty(-theme.padding.y), &ptext)?;

		let text = self.options.iter().map(|s| {
			return shapes::text(s)
				.size(theme.font_size)
				.color(theme.title_color)
				.align(gfx::Origin::TopLeft)
				.format(ctx)
				;
		}).collect::<Vec<shapes::FormattedText>>();

		let max_width = text.iter().fold(0.0, |w, t| {
			if t.width() > w {
				return t.width();
			} else {
				return w;
			}
		});

		let ox = ptext.width() + theme.padding.x;
		let bh = ptext.height() + theme.padding.y * 2.0;
		let bw = max_width + theme.padding.x * 2.0 + bh;

		let area = Rect::new(vec2!(ox, 0.0), vec2!(ox + bw, -bh));
		let mpos = ctx.mouse_pos() - wctx.offset;

		if let State::Idle(_) = self.state {
			self.state = State::Idle(col::intersect2d(area, mpos));
		}

		ctx.draw(
			&shapes::rect(
				vec2!(ox, 0.0),
				vec2!(ox + bw, -bh),
			)
				.fill(theme.bar_color)
				.stroke(theme.border_color)
				.line_width(2.0)
		)?;

		if let State::Expanded(_) = self.state {

			let by = self.selected as f32 * bh;
			let by2 = by - text.len() as f32 * bh;

			ctx.draw(
				&shapes::rect(
					vec2!(ox, by),
					vec2!(ox + bw - bh, by2),
				)
					.fill(theme.bar_color)
					.stroke(theme.border_color)
					.line_width(2.0)
			)?;

			for (i, t) in text.iter().enumerate() {

				let oy = (i as f32 - self.selected as f32) * bh;
				let area = Rect::new(vec2!(ox, -oy), vec2!(ox + bw - bh, -oy - bh));
				let hovered = col::intersect2d(area, mpos);

				if hovered {
					self.state = State::Expanded(Some(i));
					ctx.draw(
						&shapes::Rect::from_rect(area)
							.fill(theme.border_color)
					)?;
				}

				ctx.draw_t(mat4!().t2(vec2!(ox + theme.padding.x, -oy - theme.padding.y)), t)?;

			}

		}

		if let Some(t) = text.get(self.selected) {
			ctx.draw_t(mat4!().t2(vec2!(ox + theme.padding.x, -theme.padding.y)), t)?;
		}

		ctx.draw(
			&shapes::rect(
				vec2!(ox + bw - bh, 0.0),
				vec2!(ox + bw, -bh),
			)
				.fill(theme.border_color)
		)?;

		return Ok(bh);

	}

}

