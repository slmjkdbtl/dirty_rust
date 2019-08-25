// wengwengweng

use super::*;
use gfx::DrawCmd;
use gfx::Transform::*;
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
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
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

		ctx.push(&[

			Scale(scale),
			Translate(self.offset * -0.5),

		], |ctx| {

			let shape = gfx::QuadShape::new(ctx.transform, self.quad, self.color, ctx.conf.quad_origin, self.flip);

			ctx.renderer_2d.push_shape(shape, &ctx.cur_shader_2d.handle, Some(&self.tex.handle))?;

			return Ok(());

		})?;

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
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
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

		for (i, ch) in self.txt.chars().enumerate() {

			let x = i as f32 * w;

			ctx.push(&[
				Translate(offset + vec2!(x, 0))
			], |ctx| {

				if ch != ' ' {

					if let Some(quad) = font.map.get(&ch) {
						ctx.draw(sprite(&tex).quad(*quad).color(self.color))?;
					}

				}

				return Ok(());

			})?;

		}

		return Ok(());

	}

}

pub struct Polygon {
	pts: Vec<Vec2>,
	color: Color,
	stroke: Option<f32>,
}

impl Polygon {
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn stroke(mut self, s: f32) -> Self {
		self.stroke = Some(s);
		return self
	}
}

pub fn polygon(pts: &[Vec2]) -> Polygon {

	return Polygon {
		pts: pts.to_vec(),
		color: color!(),
		stroke: None,
	};

}

impl DrawCmd for Polygon {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let len = self.pts.len();

		if len < 3 {
			return Ok(());
		}

		if let Some(stroke) = self.stroke {

			// TODO: smooth line join
			for i in 0..len {

				let p1 = self.pts[i];
				let p2 = self.pts[(i + 1) % len];

				ctx.draw(line(p1, p2).width(stroke).color(self.color))?;

			}

		} else {

			let mut verts = Vec::new();
			let mut indices = Vec::new();

			for (i, p) in self.pts.iter().enumerate() {

				gfx::Vertex2D::new(ctx.transform * *p, vec2!(0), self.color).push(&mut verts);

				if i >= 2 {
					indices.extend_from_slice(&[0, (i as u32 - 1), i as u32]);
				}

			}

			ctx.renderer_2d.push(&verts, &indices, &ctx.cur_shader_2d.handle, Some(&ctx.empty_tex.handle))?;

		}

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
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
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

		let pts = [
			self.p1,
			vec2!(self.p2.x, self.p1.y),
			self.p2,
			vec2!(self.p1.x, self.p2.y),
		];

		if let Some(stroke) = self.stroke {
			ctx.draw(polygon(&pts).color(self.color).stroke(stroke))?;
		} else {
			ctx.draw(polygon(&pts).color(self.color))?;
		}

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
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
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

	// TODO: clean
	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let len = (self.p2 - self.p1).mag();
		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);

		ctx.push(&[
			Translate(self.p1 + (self.p2 - self.p1) * 0.5),
			Rotate(rot),
		], |ctx| {
			return ctx.draw(Rect::from_size(len, self.width).color(self.color));
		})?;

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
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
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
			ctx.push(&[
				Translate(*pt)
			], |ctx| {
				return ctx.draw(Rect::from_size(self.size, self.size).color(self.color));
			})?;
		}

		return Ok(());

	}

}

pub struct Circle {
	center: Vec2,
	radius: f32,
	color: Color,
	sides: usize,
	stroke: Option<f32>,
}

impl Circle {
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn sides(mut self, s: usize) -> Self {
		self.sides = s;
		return self;
	}
	pub fn stroke(mut self, s: f32) -> Self {
		self.stroke = Some(s);
		return self
	}
}

pub fn circle(center: Vec2, radius: f32) -> Circle {
	return Circle {
		center: center,
		radius: radius,
		color: color!(),
		// TODO: calculate sides
		sides: radius as usize,
		stroke: None,
	};
}

impl DrawCmd for Circle {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut verts = Vec::new();

		for i in 0..self.sides {

			let angle = 360.0 / self.sides as f32 * i as f32;
			let pt = Vec2::from_angle(angle.to_radians()) * self.radius;

			verts.push(pt);

		}

		ctx.push(&[
			Translate(self.center)
		], |ctx| {

			if let Some(stroke) = self.stroke {
				ctx.draw(polygon(&verts).color(self.color).stroke(stroke))?;
			} else {
				ctx.draw(polygon(&verts).color(self.color))?;
			}

			return Ok(());

		})?;

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
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
}

impl<'a> DrawCmd for Canvas<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.push(&[
			Scale(vec2!(1.0 / ctx.dpi() as f32))
		], |ctx| {
			return ctx.draw(sprite(&self.canvas.tex).color(self.color));
		})?;

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
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
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

