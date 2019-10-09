// wengwengweng

use std::f32::consts::PI;

use super::*;
use gfx::Drawable;
use gl::VertexLayout;

pub struct Sprite<'a> {
	tex: &'a gfx::Texture,
	quad: Quad,
	offset: Vec2,
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
}

pub fn sprite<'a>(tex: &'a gfx::Texture) -> Sprite<'a> {
	return Sprite {
		tex: tex,
		quad: quad!(0, 0, 1, 1),
		color: color!(1),
		offset: vec2!(0),
		flip: gfx::Flip::None,
	};
}

impl<'a> Drawable for &Sprite<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);

		// TODO: extremely slow
		let t = ctx.transform
			.scale(scale)
			.translate(self.offset * -0.5)
			;

		let shape = gfx::QuadShape::new(t.as_mat4(), self.quad, self.color, ctx.conf.quad_origin, self.flip);

		let uniform = gfx::Uniform2D {
			proj: ctx.proj_2d,
			tex: self.tex.clone(),
		};

		ctx.renderer_2d.push_shape(shape, &ctx.cur_shader_2d.handle, &uniform)?;

		return Ok(());

	}

}

pub struct Text<'a> {
	txt: &'a str,
	font: Option<&'a gfx::BitmapFont>,
	fallback_font: Option<&'a gfx::BitmapFont>,
	color: Color,
	offset: Vec2,
	wrap: Option<u32>,
}

impl<'a> Text<'a> {
	pub fn text(mut self, txt: &'a str) -> Self {
		self.txt = txt;
		return self;
	}
	pub fn font(mut self, font: &'a gfx::BitmapFont) -> Self {
		self.font = Some(font);
		return self;
	}
	pub fn fallback_font(mut self, font: &'a gfx::BitmapFont) -> Self {
		self.fallback_font = Some(font);
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
		fallback_font: None,
		offset: vec2!(0),
		color: color!(1),
		wrap: None,
	};
}

impl<'a> Drawable for &Text<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let font;

		if let Some(f) = self.font {
			font = f;
		} else {
			font = &ctx.default_font;
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
		// TODO: don't clone here
		let map = font.map.clone();

		// TODO: text wrapping
		// TODO: text align
		for (i, ch) in self.txt.chars().enumerate() {

			let x = i as f32 * w;

			ctx.push(&gfx::t()
				.translate(offset + vec2!(x, 0))
			, |ctx| {

				if ch == '\n' {
					// TODO: next line
				} else if ch != ' ' {
					if let Some(quad) = map.get(&ch) {
						ctx.draw(&sprite(&tex).quad(*quad).color(self.color))?;
					}
				}

				return Ok(());

			})?;

		}

		return Ok(());

	}

}

#[derive(Clone)]
struct Stroke {
	width: f32,
	join: gfx::LineJoin,
	dash: Option<LineDash>,
}

pub struct Polygon {
	pts: Vec<Vec2>,
	color: Color,
	stroke: Option<Stroke>,
	radius: Option<f32>,
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
	pub fn stroke(mut self, w: f32) -> Self {
		self.stroke = Some(Stroke {
			width: w,
			join: gfx::LineJoin::None,
			dash: None,
		});
		return self
	}
	pub fn line_join(mut self, j: gfx::LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = Some(r);
		return self
	}
}

pub fn polygon(pts: &[Vec2]) -> Polygon {

	return Polygon {
		pts: pts.to_vec(),
		color: color!(),
		stroke: None,
		radius: None,
	};

}

impl Drawable for &Polygon {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if self.pts.len() < 3 {
			return Ok(());
		}

		use std::borrow::Cow;

		let pts = if let Some(radius) = self.radius {
			Cow::Owned(rounded_poly_verts(&self.pts, radius, None))
		} else {
			Cow::Borrowed(&self.pts)
		};

		if let Some(stroke) = &self.stroke {

			// TODO: line join
			for i in 0..pts.len() {

				let p1 = pts[i];
				let p2 = pts[(i + 1) % pts.len()];

				use gfx::LineJoin::*;

				match stroke.join {
					None => {
						ctx.draw(&line(p1, p2).width(stroke.width).color(self.color))?;
					},
					Bevel => {
						// TODO
						ctx.draw(&line(p1, p2).width(stroke.width).color(self.color))?;
					},
					Miter => {
						// TODO
						ctx.draw(&line(p1, p2).width(stroke.width).color(self.color))?;
					},
					Round => {
						ctx.draw(&line(p1, p2).width(stroke.width).color(self.color).cap(gfx::LineCap::Round))?;
					},
				}

			}

		} else {

			let mut verts = Vec::with_capacity(pts.len() * gfx::Vertex2D::STRIDE);
			let mut indices = Vec::new();

			for (i, p) in pts.iter().enumerate() {

				gfx::Vertex2D::new(ctx.transform.as_mat4() * *p, vec2!(0), self.color).push(&mut verts);

				if i >= 2 {
					indices.extend_from_slice(&[0, (i as u32 - 1), i as u32]);
				}

			}

			ctx.renderer_2d.push(&verts, &indices, &ctx.cur_shader_2d.handle, &gfx::Uniform2D {
				proj: ctx.proj_2d,
				tex: ctx.empty_tex.clone(),
			})?;

		}

		return Ok(());

	}

}

pub struct Gradient {
	p1: Vec2,
	p2: Vec2,
	steps: Vec<(Color, f32)>,
	width: f32,
}

pub fn gradient(p1: Vec2, p2: Vec2, steps: &[(Color, f32)]) -> Gradient {
	return Gradient {
		p1: p1,
		p2: p2,
		steps: steps.to_vec(),
		width: 1.0,
	};
}

impl Gradient {
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
		return self;
	}
}

impl Drawable for &Gradient {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if self.steps.len() < 2 {
			return Err(Error::Gfx("need at least 2 points to draw a gradient".into()));
		}

		use gfx::Vertex2D;

		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);
		let mut verts = Vec::with_capacity(4 + 2 * (self.steps.len() - 2) * gfx::Vertex2D::STRIDE);

		let matrix = ctx.transform
			.translate((self.p1 + self.p2) * 0.5)
			.rotate(rot - 90f32.to_radians())
			.as_mat4();

		let w = self.width;
		let h = Vec2::dis(self.p1, self.p2);

		let mut last_pos = None;

		for s in &self.steps {

			if (last_pos.is_none()) {
				if (s.1 != 0.0) {
					return Err(Error::Gfx("gradient step should start at 0.0".into()));
				}
			}

			last_pos = Some(s.1);

			Vertex2D::new(matrix * vec2!(-w / 2.0, -h / 2.0 + h * s.1), vec2!(0), s.0).push(&mut verts);
			Vertex2D::new(matrix * vec2!(w / 2.0, -h / 2.0 + h * s.1), vec2!(0), s.0).push(&mut verts);

		}

		if (last_pos != Some(1.0)) {
			return Err(Error::Gfx("gradient step should end at 1.0".into()));
		}

		let indices = [
			0, 1, 2,
			1, 2, 3,
		];

		let indices: Vec<u32> = indices
			.iter()
			.cycle()
			.take((self.steps.len() - 1) * indices.len())
			.enumerate()
			.map(|(i, vertex)| vertex + i as u32 / 6 * 2 )
			.collect();

		ctx.renderer_2d.push(&verts, &indices, &ctx.cur_shader_2d.handle, &gfx::Uniform2D {
			proj: ctx.proj_2d,
			tex: ctx.empty_tex.clone(),
		})?;

		return Ok(());

	}

}

pub struct Rect {
	p1: Vec2,
	p2: Vec2,
	radius: Option<f32>,
	stroke: Option<Stroke>,
	color: Color,
}

pub fn rect(p1: Vec2, p2: Vec2) -> Rect {
	return Rect::new(p1, p2);
}

impl Rect {
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = Some(r);
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
			radius: None,
			stroke: None,
			color: color!(1),
		};
	}
	pub fn stroke(mut self, w: f32) -> Self {
		self.stroke = Some(Stroke {
			width: w,
			join: gfx::LineJoin::None,
			dash: None,
		});
		return self
	}
	pub fn line_join(mut self, j: gfx::LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn from_size(w: f32, h: f32) -> Self {
		return Self::new(vec2!(w, h) * -0.5, vec2!(w, h) * 0.5);
	}
}

// struct Stroke {
// 	width: f32,
// 	line_join: gfx::LineJoin,
// }

impl Drawable for &Rect {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut pts = vec![
			self.p1,
			vec2!(self.p2.x, self.p1.y),
			self.p2,
			vec2!(self.p1.x, self.p2.y),
		];

		if let Some(radius) = self.radius {
			pts = rounded_poly_verts(&pts, radius, None);
		}

		if let Some(stroke) = &self.stroke {
			ctx.draw(&polygon(&pts).color(self.color).stroke(stroke.width).line_join(stroke.join))?;
		} else {
			ctx.draw(&polygon(&pts).color(self.color))?;
		}

		return Ok(());

	}

}

#[derive(Clone)]
struct LineDash {
	len: f32,
	interval: f32,
}

pub struct Line {
	p1: Vec2,
	p2: Vec2,
	width: f32,
	color: Color,
	cap: gfx::LineCap,
	dash: Option<LineDash>,
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
	pub fn cap(mut self, c: gfx::LineCap) -> Self {
		self.cap = c;
		return self;
	}
	pub fn dashed(mut self, len: f32, interval: f32) -> Self {
		self.dash = Some(LineDash {
			len: len,
			interval: interval,
		});
		return self;
	}
}

pub fn line(p1: Vec2, p2: Vec2) -> Line {
	return Line {
		p1: p1,
		p2: p2,
		width: 1.0,
		color: color!(1),
		cap: gfx::LineCap::Butt,
		dash: None,
	};
}

impl Drawable for &Line {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let len = (self.p2 - self.p1).mag();
		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);

		ctx.push(&gfx::t()

			.translate((self.p1 + self.p2) * 0.5)
			.rotate(rot)

		, |ctx| {

			let w = len;
			let h = self.width;

			ctx.draw(&Rect::from_size(w, h).color(self.color))?;

			if let gfx::LineCap::Round = self.cap {
				ctx.draw(&circle(vec2!(-w / 2.0, 0), h / 2.0))?;
				ctx.draw(&circle(vec2!(w / 2.0, 0), h / 2.0))?;
			}

			return Ok(());

		})?;

		return Ok(());

	}

}

// TODO
pub struct Curve {
	// ...
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

impl<'a> Drawable for &Points<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		for pt in self.pts {
			ctx.push(&gfx::t()
				.translate(*pt)
			, |ctx| {
				return ctx.draw(&Rect::from_size(self.size, self.size).color(self.color));
			})?;
		}

		return Ok(());

	}

}

pub struct Circle {
	center: Vec2,
	radius: f32,
	color: Color,
	segments: Option<u32>,
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
	pub fn stroke(mut self, s: f32) -> Self {
		self.stroke = Some(s);
		return self
	}
	pub fn segments(mut self, s: u32) -> Self {
		self.segments = Some(s);
		return self
	}
}

pub fn circle(center: Vec2, radius: f32) -> Circle {
	return Circle {
		center: center,
		radius: radius,
		color: color!(1),
		segments: None,
		stroke: None,
	};
}

// TODO: is this correct?
fn circle_segments(radius: f32) -> u32 {
	return (radius.sqrt() * 6.0) as u32;
}

fn normalize_angle(angle: f32) -> f32 {
	if angle < 0.0 {
		return PI * 2.0 + angle;
	} else {
		return angle;
	}
}

fn rounded_poly_verts(verts: &[Vec2], radius: f32, segments: Option<u32>) -> Vec<Vec2> {

	let segments = segments.unwrap_or(circle_segments(radius));
	let segments = segments as usize;
	let mut nv = Vec::with_capacity(segments);
	let len = verts.len();

	for i in 0..len {

		// TODO: fix weirdness
		let prev = verts.get(i - 1).map(|p| *p).unwrap_or(verts[len - 1]);
		let p = verts[i];
		let next = verts.get(i + 1).map(|p| *p).unwrap_or(verts[0]);
		let angle = normalize_angle(p.angle(prev) - p.angle(next));
		let dis = radius / f32::tan(angle / 2.0);

		let p1 = p + (prev - p) * (dis / (prev - p).mag());
		let p2 = p + (next - p) * (dis / (next - p).mag());

		let center = p + (p1 - p) + (p2 - p);

		let start_angle = center.angle(p1);
		let end_angle = start_angle + angle;

		let arc = arc_verts(radius, start_angle, end_angle, None)
			.iter()
			.map(|p| *p + center)
			.collect::<Vec<Vec2>>()
			;

		nv.extend_from_slice(&arc);

	}

	return nv;

}

fn arc_verts(radius: f32, start: f32, end: f32, segments: Option<u32>) -> Vec<Vec2> {

	assert!(end > start, "end angle should be larger than start");
	let segments = segments.unwrap_or(f32::ceil(circle_segments(radius) as f32 * (end - start) / (PI * 2.0)) as u32);
	let segments = segments as usize;
	let step = (end - start) / segments as f32;
	let mut verts = Vec::with_capacity(segments);

	for i in 0..=segments {

		let angle = start + i as f32 * step;
		verts.push(Vec2::from_angle(angle) * radius);

	}

	return verts;

}

fn circle_verts(radius: f32, segments: Option<u32>) -> Vec<Vec2> {
	return arc_verts(radius, 0.0, PI * 2.0, segments);
}

impl Drawable for &Circle {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if self.radius < 0.0 {
			return Ok(());
		}

		let verts = circle_verts(self.radius, self.segments);

		ctx.push(&gfx::t()
			.translate(self.center)
		, |ctx| {

			if let Some(stroke) = self.stroke {
				ctx.draw(&polygon(&verts).color(self.color).stroke(stroke))?;
			} else {
				ctx.draw(&polygon(&verts).color(self.color))?;
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

impl<'a> Drawable for &Canvas<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.push(&gfx::t()
			.scale(vec2!(1.0 / ctx.dpi() as f32))
		, |ctx| {
			return ctx.draw(&sprite(&self.canvas.tex).color(self.color));
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

impl<'a> Drawable for &Model<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		self.model.renderer.draw(&ctx.cur_shader_3d.handle, Some(&gfx::Uniform3D {
			proj: ctx.proj_3d,
			view: ctx.view_3d,
			model: ctx.transform,
			color: self.color,
			tex: ctx.empty_tex.clone(),
		}));

		return Ok(());

	}

}

pub struct Cube;

pub fn cube() -> Cube {
	return Cube;
}

impl Drawable for &Cube {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		ctx.cube_renderer.draw(&ctx.cur_shader_3d.handle, Some(&gfx::Uniform3D {
			proj: ctx.proj_3d,
			view: ctx.view_3d,
			model: ctx.transform,
			color: color!(),
			tex: ctx.empty_tex.clone(),
		}));

		return Ok(());

	}

}

pub struct Sprite3D<'a> {
	tex: &'a gfx::Texture,
	quad: Quad,
	offset: Vec2,
	flip: gfx::Flip,
	color: Color,
}

// TODO: up side down?
// TODO: clean
impl<'a> Sprite3D<'a> {
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
}

pub fn sprite3d<'a>(tex: &'a gfx::Texture) -> Sprite3D<'a> {
	return Sprite3D {
		tex: tex,
		quad: quad!(0, 0, 1, 1),
		color: color!(1),
		offset: vec2!(0),
		flip: gfx::Flip::None,
	};
}

impl<'a> Drawable for &Sprite3D<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);
		let offset = self.offset * -0.5;

		ctx.push(&gfx::t()
			.scale_3d(vec3!(scale.x, scale.y, 1.0))
			.translate_3d(vec3!(offset.x, offset.y, 0.0))
		, |ctx| {

			let shape = gfx::FlagShape::new(ctx.transform.as_mat4(), self.quad, self.color, ctx.conf.quad_origin, self.flip);

			ctx.renderer_3d.push_shape(shape, &ctx.cur_shader_3d.handle, &gfx::Uniform3D {
				proj: ctx.proj_3d,
				view: ctx.view_3d,
				model: gfx::Transform::new(),
				color: color!(),
				tex: self.tex.clone(),
			})?;

			return Ok(());

		})?;

		return Ok(());

	}

}

