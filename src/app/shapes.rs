// wengwengweng

use super::*;
use gfx::DrawCmd;
use gl::VertexLayout;

pub struct Sprite<'a> {
	tex: &'a gfx::Tex2D,
	quad: Quad,
	offset: Vec2,
	radius: f32,
	flip: gfx::Flip,
	color: Color,
}

impl<'a> Sprite<'a> {
	pub fn quad(mut self, quad: Quad) -> Self {
		self.quad = quad;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn offset(mut self, offset: Vec2) -> Self {
		self.offset = offset;
		return self;
	}
	pub fn flip(mut self, flip: gfx::Flip) -> Self {
		self.flip = flip;
		return self;
	}
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = r;
		return self
	}
}

pub fn sprite<'a>(tex: &'a gfx::Tex2D) -> Sprite<'a> {
	return Sprite {
		tex: tex,
		quad: quad!(0, 0, 1, 1),
		color: color!(1),
		offset: vec2!(0),
		radius: 0.0,
		flip: gfx::Flip::None,
	};
}

impl<'a> DrawCmd for Sprite<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);

		ctx.push();
		ctx.scale(scale);
		ctx.translate(self.offset * -0.5);

		let shape = gfx::QuadShape::new(ctx.transform, self.quad, self.color, ctx.conf.quad_origin, self.flip);

		ctx.renderer_2d.push_shape(shape, &ctx.cur_shader_2d.handle, Some(&self.tex.handle))?;

		ctx.pop()?;

		return Ok(());

	}

}

pub struct Text<'a> {
	txt: &'a str,
	font: Option<&'a gfx::Font>,
	color: Color,
	offset: Vec2,
	wrap: Option<u32>,
}

impl<'a> Text<'a> {
	pub fn text(mut self, txt: &'a str) -> Self {
		self.txt = txt;
		return self;
	}
	pub fn font(mut self, font: &'a gfx::Font) -> Self {
		self.font = Some(font);
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn offset(mut self, offset: Vec2) -> Self {
		self.offset = offset;
		return self;
	}
	pub fn wrap(mut self, wrap: u32) -> Self {
		self.wrap = Some(wrap);
		return self;
	}
}

pub fn text<'a>(txt: &'a str) -> Text<'a> {
	return Text {
		txt: txt,
		font: None,
		offset: vec2!(0),
		color: color!(1),
		wrap: None,
	};
}

impl<'a> DrawCmd for Text<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let font;

		if let Some(f) = self.font {
			font = f.clone();
		} else {
			font = ctx.default_font.clone();
		}

		let len = self.txt.len();
		let gw = font.width();
		let gh = font.height();
		let tw = font.width() * len as i32;
		let th = gh;
		let w = font.quad_size.x * font.tex.width() as f32;
		let h = font.quad_size.y * font.tex.height() as f32;
		let tex = font.tex.clone();
		let offset = vec2!(gw as f32 * (len as f32 * -0.5 + 0.5), 0);
		let offset = offset + self.offset * vec2!(tw, th) * -0.5;

		ctx.push();
		ctx.translate(offset);

		for (i, ch) in self.txt.chars().enumerate() {

			let x = i as f32 * w;

			if ch != ' ' {

				if let Some(quad) = font.map.get(&ch) {
					ctx.draw(sprite(&tex).quad(*quad).color(self.color))?;
				}

			}

			ctx.translate(vec2!(w, 0));

		}

		ctx.pop()?;

		return Ok(());

	}

}

pub struct Line {
	p1: Vec2,
	p2: Vec2,
	width: f32,
	color: Color,
}

impl Line {
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
}

pub fn line(p1: Vec2, p2: Vec2) -> Line {
	return Line {
		p1: p1,
		p2: p2,
		width: 1.0,
		color: color!(1),
	};
}

impl DrawCmd for Line {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let len = ((self.p2.x - self.p1.x).powi(2) + (self.p2.y - self.p1.y).powi(2)).sqrt();
		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);

		ctx.push();
		ctx.translate(self.p1);
		ctx.rotate(rot);
		ctx.draw(Rect::from_size(len, self.width).color(self.color))?;
		ctx.pop()?;

		return Ok(());

	}

}

pub struct Rect {
	p1: Vec2,
	p2: Vec2,
	radius: f32,
	stroke: Option<f32>,
	color: Color,
}

pub fn rect(p1: Vec2, p2: Vec2) -> Rect {
	return Rect::new(p1, p2);
}

impl Rect {
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = r;
		return self
	}
	pub fn stroke(mut self, s: f32) -> Self {
		self.stroke = Some(s);
		return self
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn new(p1: Vec2, p2: Vec2) -> Self {
		return Rect {
			p1: p1,
			p2: p2,
			radius: 0.0,
			stroke: None,
			color: color!(1),
		};
	}
	pub fn from_size(w: f32, h: f32) -> Self {
		return Self::new(vec2!(w, h) * -0.5, vec2!(w, h) * 0.5);
	}
}

impl DrawCmd for Rect {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if let Some(stroke) = self.stroke {
			// ...
		} else {

			let center = (self.p1 + self.p2) / 2.0;
			let width = self.p2.x - self.p1.x;
			let height = self.p2.y - self.p1.y;

			ctx.push();
			ctx.translate(center);
			ctx.scale(vec2!(width, height));
			ctx.draw(sprite(&ctx.empty_tex.clone()).color(self.color))?;
			ctx.pop()?;

		}

		return Ok(());

	}

}

pub struct Points<'a> {
	pts: &'a[Vec2],
	size: f32,
	color: Color,
}

impl<'a> Points<'a> {
	pub fn size(mut self, s: f32) -> Self {
		self.size = s;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
}

pub fn pts<'a>(pts: &'a[Vec2]) -> Points<'a> {
	return Points {
		pts: pts,
		size: 1.0,
		color: color!(1),
	};
}

impl<'a> DrawCmd for Points<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		for pt in self.pts {
			ctx.push();
			ctx.translate(*pt);
			ctx.draw(Rect::from_size(self.size, self.size).color(self.color))?;
			ctx.pop()?;
		}

		return Ok(());

	}

}

pub struct Polygon {
	pts: Vec<Vec2>,
	color: Color,
}

impl Polygon {
	fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
}

// TODO: calculate correct verts and indices
pub fn polygon(pts: &[Vec2]) -> Polygon {

	if pts.len() < 3 {
		// TODO: error
	}

	return Polygon {
		pts: pts.to_vec(),
		color: color!(),
	};

}

impl DrawCmd for Polygon {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut verts = Vec::new();
		let mut indices = Vec::new();

		for (i, p) in self.pts.iter().enumerate() {

			gfx::Vertex2D::new(*p, vec2!(0), self.color).push(&mut verts);

			if i >= 2 {
				indices.extend_from_slice(&[0, (i as u32 - 1), i as u32]);
			}

		}

		ctx.renderer_2d.push(&verts, &indices, &ctx.cur_shader_2d.handle, Some(&ctx.empty_tex.handle))?;

		return Ok(());

	}

}

pub struct Canvas<'a> {
	canvas: &'a gfx::Canvas,
	color: Color,
}

pub fn canvas<'a>(c: &'a gfx::Canvas) -> Canvas<'a> {
	return Canvas {
		canvas: c,
		color: color!(1),
	};
}

impl<'a> Canvas<'a> {
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
}

impl<'a> DrawCmd for Canvas<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.push();
		ctx.scale(vec2!(1.0 / ctx.dpi() as f32));
		ctx.draw(sprite(&self.canvas.tex).color(self.color))?;
		ctx.pop()?;

		return Ok(());

	}

}

pub struct Model<'a> {
	model: &'a gfx::Model,
	color: Color,
}

pub fn model<'a>(m: &'a gfx::Model) -> Model<'a> {
	return Model {
		model: m,
		color: color!(1),
	};
}

impl<'a> Model<'a> {
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
}

impl<'a> DrawCmd for Model<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.cur_shader_3d.send("model", ctx.transform);
		ctx.cur_shader_3d.send("color", self.color);
		ctx.draw_calls += 1;
		self.model.renderer.draw(&ctx.cur_shader_3d.handle);
		ctx.cur_shader_3d.send("color", color!(1));

		return Ok(());

	}

}

pub struct Cube;

pub fn cube() -> Cube {
	return Cube;
}

impl DrawCmd for Cube {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.cur_shader_3d.send("model", ctx.transform);
		ctx.draw_calls += 1;
		ctx.cube_renderer.draw(&ctx.cur_shader_3d.handle);

		return Ok(());

	}

}

