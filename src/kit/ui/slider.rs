// wengwengweng

use super::*;
use std::ops::*;

pub trait SliderValue =
	'static
	+ Copy
	+ Into<f32>
	+ std::fmt::Display
	+ Add<Self, Output = Self>
	+ Sub<Self, Output = Self>
	+ Mul<Self, Output = Self>
	+ Div<Self, Output = Self>
	;

pub struct Slider<T: SliderValue> {
	prompt: &'static str,
	val: T,
	min: T,
	max: T,
}

impl<T: SliderValue> Slider<T> {
	pub fn new(p: &'static str, val: T, min: T, max: T) -> Self {
		return Self {
			prompt: p,
			val: val,
			min: min,
			max: max,
		};
	}
	pub fn value(&self) -> T {
		return self.val;
	}
}

impl<T: SliderValue> Widget for Slider<T> {

	fn draw(&self, ctx: &mut Ctx, pctx: &PanelCtx) -> Result<f32> {

		let mut height = 0.0;

		let ptext = shapes::text(&format!("{}:", self.prompt))
			.size(pctx.theme.font_size)
			.color(pctx.theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		height += ptext.height() + pctx.theme.margin * 0.8;

		ctx.draw(&ptext)?;

		let itext = shapes::text(&format!("{}", self.val))
			.size(pctx.theme.font_size)
			.color(pctx.theme.title_color)
// 			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		let padding = 9.0;
		let box_height = itext.height() + padding * 2.0;
		let a: f32 = (self.val - self.min).into();
		let b: f32 = (self.max - self.min).into();
		let r = a / b;
		let bpos = vec2!(pctx.width * r, -height - box_height * 0.5);

		ctx.draw(
			&shapes::rect(
				vec2!(0, -height),
				vec2!(pctx.width - 4.0, -height - box_height)
			)
				.stroke(pctx.theme.border_color)
				.line_width(2.0)
				.fill(pctx.theme.bar_color)
		)?;

		ctx.draw(
			&shapes::rect(
				bpos - vec2!(12, box_height * 0.5),
				bpos + vec2!(12, box_height * 0.5),
			)
				.fill(pctx.theme.border_color)
		)?;

		ctx.draw_t(
			mat4!()
// 				.t2(vec2!(padding, -height - padding))
				.t2(vec2!(pctx.width / 2.0, -height - box_height * 0.5))
				,
			&itext
		)?;

		height += box_height;

		return Ok(height);

	}

}

