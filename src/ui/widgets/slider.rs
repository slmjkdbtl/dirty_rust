// wengwengweng

use std::fmt;
use std::ops;
use super::*;

pub trait SliderValue:
	Copy
	+ PartialOrd
	+ ops::Sub<Output = Self>
	+ fmt::Display
	+ 'static
{
	fn to_f32(self) -> f32;
	fn from_f32(f: f32) -> Self;
}

macro_rules! impl_slider_val {
	($ty:ty, $f:ident) => {
		impl SliderValue for $ty {
			fn to_f32(self) -> f32 {
				return self as f32;
			}
			fn from_f32(f: f32) -> Self {
				return f.$f() as $ty;
			}
		}
	};
	($ty:ty) => {
		impl SliderValue for $ty {
			fn to_f32(self) -> f32 {
				return self as f32;
			}
			fn from_f32(f: f32) -> Self {
				return f as $ty;
			}
		}
	}
}

impl_slider_val!(u8, round);
impl_slider_val!(u16, round);
impl_slider_val!(u32, round);
impl_slider_val!(u64, round);
impl_slider_val!(u128, round);
impl_slider_val!(usize, round);
impl_slider_val!(i8, round);
impl_slider_val!(i16, round);
impl_slider_val!(i32, round);
impl_slider_val!(i64, round);
impl_slider_val!(i128, round);
impl_slider_val!(isize, round);
impl_slider_val!(f32);
impl_slider_val!(f64);

pub struct Slider<T: SliderValue> {
	label: &'static str,
	val: f32,
	min: T,
	max: T,
	hovering: bool,
	dragging: bool,
	unit: f32,
}

impl<T: SliderValue> Slider<T> {
	pub fn new(p: &'static str, val: T, min: T, max: T) -> Self {
		return Self {
			label: p,
			val: val.to_f32(),
			min: min,
			max: max,
			hovering: false,
			dragging: false,
			unit: 0.0,
		};
	}
	pub fn value(&self) -> T {
		return T::from_f32(self.val);
	}
}

impl<T: SliderValue> Widget for Slider<T> {

	fn event(&mut self, e: &Event) -> bool {

		use Event::*;

		match e {

			MousePress(m) => {
				match *m {
					Mouse::Left if self.hovering => {
						self.dragging = true;
						return true;
					},
					_ => {},
				}
			},

			MouseMove(delta) => {
				if self.dragging {
					let max = self.max.to_f32();
					let min = self.min.to_f32();
					self.val = self.val + delta.x * self.unit;
					if self.val > max {
						self.val = max;
					}
					if self.val < min {
						self.val = min;
					}
					return true;
				}
			}

			MouseRelease(m) => {
				match *m {
					Mouse::Left => {
						if self.dragging {
							self.dragging = false;
							return true;
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

		let mut y = 0.0;
		let theme = ctx.theme();

		let label_shape = shapes::text(&format!("{}:", self.label))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(gfx)
			;

		y += label_shape.height() + theme.padding;

		// draw label
		gfx.draw(&label_shape)?;

		let value_shape = shapes::text(&format!("{:.2}", self.value()))
			.size(theme.font_size)
			.color(theme.title_color)
			.format(gfx)
			;

		let box_height = value_shape.height() + theme.padding * 2.0;
		let box_area = Rect::new(vec2!(0, -y), vec2!(ctx.width(), -y - box_height));
		let max = self.max.to_f32();
		let min = self.min.to_f32();

		self.hovering = col::intersect2d(box_area, ctx.mouse_pos());
		self.unit = (max - min) / ctx.width();

		let ratio = (self.val - min) / (max - min);
		let handle_width = theme.font_size * 2.5;

		let handle_pos = vec2!(
			handle_width * 0.5 + (ctx.width() - handle_width) * ratio,
			-y - box_height * 0.5
		);

		let bg_color = if self.dragging {
			theme.bar_color.brighten(0.1)
		} else {
			theme.bar_color
		};

		// draw box
		gfx.draw(
			&shapes::rect(box_area.p1, box_area.p2)
				.stroke(theme.border_color)
				.line_join(shapes::LineJoin::Round)
				.line_width(theme.line_width)
				.fill(bg_color)
		)?;

		// draw handle
		gfx.draw(
			&shapes::rect(
				handle_pos - vec2!(handle_width * 0.5, box_height * 0.5),
				handle_pos + vec2!(handle_width * 0.5, box_height * 0.5),
			)
				.fill(theme.border_color)
		)?;

		// draw value
		gfx.draw_t(
			mat4!()
				.t2(vec2!(ctx.width() / 2.0, -y - box_height * 0.5))
				,
			&value_shape
		)?;

		y += box_height;

		return Ok(y);

	}

	fn focused(&self) -> bool {
		return self.dragging;
	}

}

