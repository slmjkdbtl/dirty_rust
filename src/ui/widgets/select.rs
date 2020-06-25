// wengwengweng

use super::*;

// TODO: clean up

pub struct Select {
	label: &'static str,
	options: Vec<String>,
	selected: usize,
	state: State,
}

enum State {
	Expanded(Option<usize>),
	Idle(bool),
}

impl Select {

	pub fn new(label: &'static str, options: &[&str], i: usize) -> Self {
		return Self {
			label,
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

	fn event(&mut self, e: &Event) -> bool {

		use Event::*;

		match e {

			MousePress(m) => {

				match *m {

					Mouse::Left => {

						match self.state {
							State::Idle(b) => {
								if b {
									self.state = State::Expanded(None);
									return true;
								}
							},
							State::Expanded(b) => {
								self.state = State::Idle(false);
								if let Some(i) = b {
									self.selected = i;
									return true;
								}
							},
						}

					},

					_ => {},

				}

			},

			_ => {},

		}

		return false;

	}

	fn draw(&mut self, gfx: &mut gfx::Gfx, ctx: &WidgetCtx) -> Result<f32> {

		use geom::*;

		let theme = ctx.theme();

		let label_shape = shapes::text(&format!("{}:", self.label))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		// draw label
		gfx.draw_t(mat4!().ty(-theme.padding), &label_shape)?;

		let option_shapes = self.options.iter().map(|s| {
			return shapes::text(s)
				.size(theme.font_size)
				.color(theme.title_color)
				.align(gfx::Origin::TopLeft)
				.format(gfx)
				;
		}).collect::<Vec<shapes::FormattedText>>();

		// get the width of the longest option
		let max_width = option_shapes.iter().fold(0.0, |w, t| {
			if t.width() > w {
				return t.width();
			} else {
				return w;
			}
		});

		let ox = label_shape.width() + theme.padding;
		let bh = label_shape.height() + theme.padding * 2.0;
		let bw = max_width + theme.padding * 2.0 + bh;

		let area = Rect::new(vec2!(ox, 0.0), vec2!(ox + bw, -bh));

		if let State::Idle(_) = self.state {
			self.state = State::Idle(col::intersect2d(area, ctx.mouse_pos));
		}

		// draw container
		gfx.draw(
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
			let by2 = by - option_shapes.len() as f32 * bh;

			gfx.draw(
				&shapes::rect(
					vec2!(ox, by),
					vec2!(ox + bw - bh, by2),
				)
					.fill(theme.bar_color)
					.stroke(theme.border_color)
					.line_width(2.0)
			)?;

			for (i, t) in option_shapes.iter().enumerate() {

				let oy = (i as f32 - self.selected as f32) * bh;
				let area = Rect::new(vec2!(ox, -oy), vec2!(ox + bw - bh, -oy - bh));
				let hovered = col::intersect2d(area, ctx.mouse_pos);

				if hovered {
					self.state = State::Expanded(Some(i));
					gfx.draw(
						&shapes::Rect::from_rect(area)
							.fill(theme.border_color)
					)?;
				}

				gfx.draw_t(mat4!().t2(vec2!(ox + theme.padding, -oy - theme.padding)), t)?;

			}

		}

		if let Some(t) = option_shapes.get(self.selected) {
			gfx.draw_t(mat4!().t2(vec2!(ox + theme.padding, -theme.padding)), t)?;
		}

		// draw arrow (?)
		gfx.draw(
			&shapes::rect(
				vec2!(ox + bw - bh, 0.0),
				vec2!(ox + bw, -bh),
			)
				.fill(theme.border_color)
		)?;

// 		gfx.draw(
// 			&shapes::line(
// 				vec2!(ox + bw - bh * 0.7, -bh * 0.4),
// 				vec2!(ox + bw - bh * 0.5, -bh * 0.6),
// 			)
// 				.color(theme.border_color)
// 				.width(2.0)
// 		)?;

// 		gfx.draw(
// 			&shapes::line(
// 				vec2!(ox + bw - bh * 0.3, -bh * 0.4),
// 				vec2!(ox + bw - bh * 0.5, -bh * 0.6),
// 			)
// 				.color(theme.border_color)
// 				.width(2.0)
// 		)?;

		return Ok(bh);

	}

}

