// wengwengweng

use super::*;
use std::ops::*;

pub struct Slider {
	prompt: &'static str,
	val: f32,
	min: f32,
	max: f32,
	draggin: Option<f32>,
	hovering: bool,
}

impl Slider {
	pub fn new(p: &'static str, val: f32, min: f32, max: f32) -> Self {
		return Self {
			prompt: p,
			val: val,
			min: min,
			max: max,
			draggin: None,
			hovering: false,
		};
	}
	pub fn value(&self) -> f32 {
		return self.val;
	}
}

impl Widget for Slider {

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) {

		use input::Event::*;
		use input::Key;
		use input::Mouse;

		match e {

			MousePress(m) => {
				match *m {
					Mouse::Left if self.hovering => self.draggin = Some(ctx.mouse_pos().x),
					_ => {},
				}
			},

			MouseRelease(m) => {
				match *m {
					Mouse::Left => self.draggin = None,
					_ => {},
				}
			},

			_ => {},

		}

	}

	fn draw(&mut self, ctx: &mut Ctx, wctx: &WidgetCtx) -> Result<f32> {

		use input::Mouse;
		use geom::*;

		let mut y = 0.0;
		let theme = &wctx.theme;

		let ptext = shapes::text(&format!("{}:", self.prompt))
			.size(theme.font_size)
			.color(theme.title_color)
			.align(gfx::Origin::TopLeft)
			.format(ctx)
			;

		y += ptext.height() + theme.padding.y;

		ctx.draw(&ptext)?;

		let itext = shapes::text(&format!("{:.2}", self.val))
			.size(theme.font_size)
			.color(theme.title_color)
			.format(ctx)
			;

		let box_height = itext.height() + theme.padding.y * 2.0;
		let r = (self.val - self.min) / (self.max - self.min);
		let handle_width = 24.0;
		let bpos = vec2!(
			handle_width * 0.5 + (wctx.width - handle_width - 4.0) * r,
			-y - box_height * 0.5
		);

		let rect = Rect::new(vec2!(0, -y), vec2!(wctx.width, -y - box_height));
		let mpos = ctx.mouse_pos() - wctx.offset;

		self.hovering = col::intersect2d(rect, mpos);

		if let Some(prev_x) = self.draggin {

			let delta_x = ctx.mouse_pos().x - prev_x;

			self.val += (delta_x / wctx.width) * (self.max - self.min);
			self.val = self.val.clamp(self.min, self.max);

			self.draggin = Some(ctx.mouse_pos().x)

		}

		let c = if self.draggin.is_some() {
			theme.bar_color.brighten(0.1)
		} else {
			theme.bar_color
		};

		ctx.draw(
			&shapes::rect(
				vec2!(0, -y),
				vec2!(wctx.width - 4.0, -y - box_height)
			)
				.stroke(theme.border_color)
				.line_width(2.0)
				.fill(c)
		)?;

		ctx.draw(
			&shapes::rect(
				bpos - vec2!(handle_width * 0.5, box_height * 0.5),
				bpos + vec2!(handle_width * 0.5, box_height * 0.5),
			)
				.fill(theme.border_color)
		)?;

		ctx.draw_t(
			mat4!()
				.t2(vec2!(wctx.width / 2.0, -y - box_height * 0.5))
				,
			&itext
		)?;

		y += box_height;

		return Ok(y);

	}

}

