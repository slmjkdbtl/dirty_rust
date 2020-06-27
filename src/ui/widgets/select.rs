// wengwengweng

use std::fmt;
use super::*;

// TODO: clean up

pub trait SelectValue:
	Clone
	+ fmt::Display
	+ 'static
{}

impl<T> SelectValue for T
	where T:
	Clone
	+ fmt::Display
	+ 'static
{}

pub struct Select<T: SelectValue> {
	label: &'static str,
	options: Vec<T>,
	selected: usize,
	state: State,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
	Expanded(Option<usize>),
	Idle(bool),
}

impl<T: SelectValue> Select<T> {

	pub fn new(label: &'static str, options: &[T], i: usize) -> Self {
		return Self {
			label,
			options: options.to_vec(),
			selected: i,
			state: State::Idle(false),
		};
	}

	pub fn selected(&self) -> usize {
		return self.selected;
	}

}

impl<T: SelectValue> Widget for Select<T> {

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
			return shapes::text(&format!("{}", s))
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

		let box_x = label_shape.width() + theme.padding;
		let box_height = label_shape.height() + theme.padding * 2.0;
		let button_size = box_height;
		let box_width = max_width + theme.padding * 2.0;

		let box_area = Rect::new(vec2!(box_x, 0.0), vec2!(box_x + box_width + button_size, -box_height));

		if let State::Idle(_) = self.state {
			self.state = State::Idle(col::intersect2d(box_area, ctx.mouse_pos));
		}

		// draw box
		gfx.draw(
			&shapes::Rect::from_rect(box_area)
				.fill(theme.bar_color)
				.stroke(theme.border_color)
				.line_join(shapes::LineJoin::Round)
				.line_width(theme.line_width)
		)?;

		if let State::Expanded(_) = self.state {

			let by = self.selected as f32 * box_height;
			let by2 = by - option_shapes.len() as f32 * box_height;

			gfx.push_t(mat4!().tz(1.0), |gfx| {

				gfx.draw(
					&shapes::rect(
						vec2!(box_x, by),
						vec2!(box_x + box_width, by2),
					)
						.fill(theme.bar_color)
						.stroke(theme.border_color)
						.line_join(shapes::LineJoin::Round)
						.line_width(theme.line_width)
				)?;

				for (i, t) in option_shapes.iter().enumerate() {

					let oy = (i as f32 - self.selected as f32) * box_height;
					let option_area = Rect::new(
						vec2!(box_x, -oy),
						vec2!(box_x + box_width, -oy - box_height)
					);
					let hovered = col::intersect2d(option_area, ctx.mouse_pos);

					if hovered {
						self.state = State::Expanded(Some(i));
						gfx.draw(
							&shapes::Rect::from_rect(option_area)
								.fill(theme.border_color)
						)?;
					}

					gfx.draw_t(
						mat4!()
							.t2(vec2!(box_x + theme.padding, -oy - theme.padding))
							,
						t
					)?;

				}

				return Ok(());

			})?;

		}

		if let Some(t) = option_shapes.get(self.selected) {
			gfx.draw_t(mat4!().t2(vec2!(box_x + theme.padding, -theme.padding)), t)?;
		}

		// draw button
		gfx.draw(
			&shapes::rect(
				vec2!(box_x + box_width, 0.0),
				vec2!(box_x + box_width + button_size, -box_height),
			)
				.fill(theme.border_color)
		)?;

		// draw arrow
		gfx.draw(
			&shapes::line(
				vec2!(box_x + box_width + button_size * 0.4, -button_size * 0.4),
				vec2!(box_x + box_width + button_size * 0.5, -button_size * 0.3),
			)
				.color(theme.bar_color)
				.cap(shapes::LineCap::Round)
				.width(theme.line_width)
		)?;

		gfx.draw(
			&shapes::line(
				vec2!(box_x + box_width + button_size * 0.6, -button_size * 0.6),
				vec2!(box_x + box_width + button_size * 0.5, -button_size * 0.7),
			)
				.color(theme.bar_color)
				.cap(shapes::LineCap::Round)
				.width(theme.line_width)
		)?;

		gfx.draw(
			&shapes::line(
				vec2!(box_x + box_width + button_size * 0.4, -button_size * 0.6),
				vec2!(box_x + box_width + button_size * 0.5, -button_size * 0.7),
			)
				.color(theme.bar_color)
				.cap(shapes::LineCap::Round)
				.width(theme.line_width)
		)?;

		gfx.draw(
			&shapes::line(
				vec2!(box_x + box_width + button_size * 0.6, -button_size * 0.4),
				vec2!(box_x + box_width + button_size * 0.5, -button_size * 0.3),
			)
				.color(theme.bar_color)
				.cap(shapes::LineCap::Round)
				.width(theme.line_width)
		)?;

		return Ok(box_height);

	}

	fn focused(&self) -> bool {
		if let State::Expanded(_) = self.state {
			return true;
		} else {
			return false;
		}
	}

}

