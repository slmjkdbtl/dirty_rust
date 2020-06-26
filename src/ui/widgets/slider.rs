// wengwengweng

use std::fmt;
use std::ops;
use super::*;

const HANDLE_WIDTH: f32 = 32.0;

pub trait SliderValue:
	Copy
	+ PartialOrd
	+ ops::Sub<Output = Self>
	+ fmt::Display
	+ 'static
{
	fn into_f32(self) -> f32;
	fn from_f32(f: f32) -> Self;
}

macro_rules! impl_slider_val {
	($ty:ty) => {
		impl SliderValue for $ty {
			fn into_f32(self) -> f32 {
				return self as f32;
			}
			fn from_f32(f: f32) -> Self {
				return f as $ty;
			}
		}
	}
}

impl_slider_val!(u8);
impl_slider_val!(u16);
impl_slider_val!(u32);
impl_slider_val!(u64);
impl_slider_val!(u128);
impl_slider_val!(usize);
impl_slider_val!(i8);
impl_slider_val!(i16);
impl_slider_val!(i32);
impl_slider_val!(i64);
impl_slider_val!(i128);
impl_slider_val!(isize);
impl_slider_val!(f32);
impl_slider_val!(f64);

pub struct Slider<T: SliderValue> {
	label: &'static str,
	val: T,
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
			val: val,
			min: min,
			max: max,
			hovering: false,
			dragging: false,
			unit: 0.0,
		};
	}
	pub fn value(&self) -> T {
		return self.val;
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
					self.val = T::from_f32(self.val.into_f32() + delta.x * self.unit);
					if self.val > self.max {
						self.val = self.max;
					}
					if self.val < self.min {
						self.val = self.min;
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

		let value_shape = shapes::text(&format!("{:.2}", self.val))
			.size(theme.font_size)
			.color(theme.title_color)
			.format(gfx)
			;

		let box_height = value_shape.height() + theme.padding * 2.0;
		let box_area = Rect::new(vec2!(0, -y), vec2!(ctx.width(), -y - box_height));

		self.hovering = col::intersect2d(box_area, ctx.mouse_pos());
		self.unit = (self.max - self.min).into_f32() / ctx.width();

		let ratio = (self.val - self.min).into_f32() / (self.max - self.min).into_f32();

		let handle_pos = vec2!(
			HANDLE_WIDTH * 0.5 + (ctx.width() - HANDLE_WIDTH) * ratio,
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
				.line_width(2.0)
				.fill(bg_color)
		)?;

		// draw handle
		gfx.draw(
			&shapes::rect(
				handle_pos - vec2!(HANDLE_WIDTH * 0.5, box_height * 0.5),
				handle_pos + vec2!(HANDLE_WIDTH * 0.5, box_height * 0.5),
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

}

