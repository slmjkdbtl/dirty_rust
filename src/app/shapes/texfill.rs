// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug)]
pub enum FillMode {
	Stretch,
	Tiled,
}

// TODO: support other origins
#[derive(Clone)]
pub struct TexFill<'a> {
	p1: Vec2,
	p2: Vec2,
	tex: &'a gfx::Texture,
	mode: FillMode,
	color: Color,
}

impl<'a> TexFill<'a> {
	pub fn new(p1: Vec2, p2: Vec2, tex: &'a gfx::Texture) -> Self {
		let lx = f32::min(p1.x, p2.x);
		let mx = f32::max(p1.x, p2.x);
		let ly = f32::min(p1.y, p2.y);
		let my = f32::max(p1.y, p2.y);
		return Self {
			p1: vec2!(lx, ly),
			p2: vec2!(mx, my),
			tex: tex,
			mode: FillMode::Tiled,
			color: rgba!(1),
		};
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn mode(mut self, m: FillMode) -> Self {
		self.mode = m;
		return self;
	}
}

pub fn texfill<'a>(p1: Vec2, p2: Vec2, tex: &'a gfx::Texture) -> TexFill<'a> {
	return TexFill::new(p1, p2, tex);
}

impl<'a> gfx::Drawable for TexFill<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let tw = self.tex.width() as f32;
		let th = self.tex.height() as f32;
		let pw = self.p2.x - self.p1.x;
		let ph = self.p2.y - self.p1.y;

		ctx.push(&gfx::t().t2(self.p1), |ctx| {

			match self.mode {

				FillMode::Stretch => {},
				FillMode::Tiled => {

					let cw = pw / tw;
					let ch = ph / th;

					for i in 0..f32::ceil(cw) as i32 {

						for j in 0..f32::ceil(ch) as i32 {

							let i = i as f32;
							let j = j as f32;

							let qw = if (cw) - i < 1.0 {
								(cw) - i
							} else {
								1.0
							};

							let qh = if (ch) - j < 1.0 {
								(ch) - j
							} else {
								1.0
							};

							ctx.draw_t(
								&gfx::t()
									.t2(vec2!(i * tw, j * th))
									,
								&sprite(&self.tex)
									.color(self.color)
									.offset(vec2!(-1, -1))
									.quad(quad!(0, 0, qw, qh))
									,
							)?;

						}

					}

				},

			}

			return Ok(());

		})?;

		return Ok(());

	}

}

