// wengwengweng

use std::f32::consts::PI;

use super::*;
use gfx::Drawable;
use gl::VertexLayout;

#[derive(Clone)]
pub struct Sprite<'a> {
	tex: &'a gfx::Texture,
	quad: Quad,
	offset: Option<Vec2>,
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
		self.offset = Some(offset);
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
		offset: None,
		flip: gfx::Flip::None,
	};
}

impl<'a> Drawable for Sprite<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);
		let offset = self.offset.unwrap_or(ctx.conf.origin.as_pt());

		// TODO: extremely slow
		let t = ctx.transform
			.scale(scale)
			.translate(offset * -0.5)
			;

		let shape = gfx::QuadShape {
			transform: t.as_mat4(),
			quad: self.quad,
			color: self.color,
			flip: self.flip,
		};

		let uniform = gfx::Uniform2D {
			proj: ctx.proj_2d,
			tex: self.tex.clone(),
			custom: ctx.cur_custom_uniform_2d.clone(),
		};

		ctx.renderer_2d.push_shape(shape, &ctx.cur_pipeline_2d, &uniform)?;

		return Ok(());

	}

}

pub struct Text<'a> {
	content: &'a str,
	font: Option<&'a dyn gfx::Font>,
	color: Color,
	align: Option<gfx::Origin>,
	wrap: Option<f32>,
}

pub fn text<'a>(s: &'a str) -> Text<'a> {
	return Text {
		content: s,
		font: None,
		align: None,
		color: color!(1),
		wrap: None,
	};
}

impl<'a> Text<'a> {
	pub fn font(mut self, f: &'a dyn gfx::Font) -> Self {
		self.font = Some(f);
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
	pub fn align(mut self, o: gfx::Origin) -> Self {
		self.align = Some(o);
		return self;
	}
	pub fn wrap(mut self, wrap: f32) -> Self {
		self.wrap = Some(wrap);
		return self;
	}
}

impl<'a> Drawable for Text<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let font = self.font.unwrap_or(&ctx.default_font);
		// TODO: no clone plz
		let tex = font.texture().clone();
		let map = font.map().clone();
		let (tw, th) = (tex.width(), tex.height());

		// TODO: wrap & align

		let mut x = 0.0;

		for ch in self.content.chars() {

			if let Some(quad) = map.get(&ch) {

				ctx.push(&gfx::t()
					.translate(vec2!(x, 0))
				, |ctx| {

					ctx.draw(
						shapes::sprite(&tex)
							.quad(*quad)
							.color(self.color)
					)?;

					x += tw as f32 * quad.w;

					return Ok(());

				})?;

			}

		}

		return Ok(());

	}

}

#[derive(Clone)]
struct Stroke {
	width: f32,
	join: gfx::LineJoin,
	dash: Option<LineDash>,
	color: Color,
}

pub struct Polygon {
	pts: Vec<Vec2>,
	fill: Option<Color>,
	stroke: Option<Stroke>,
	radius: Option<f32>,
}

impl Polygon {
	pub fn fill(mut self, c: Color) -> Self {
		self.fill = Some(c);
		return self;
	}
	pub fn no_fill(mut self) -> Self {
		self.fill = None;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		if let Some(fill) = &mut self.fill {
			fill.a = a;
		}
		if let Some(stroke) = &mut self.stroke {
			stroke.color.a = a;
		}
		return self;
	}
	pub fn stroke(mut self, c: Color) -> Self {
		self.stroke = Some(Stroke {
			width: 1.0,
			join: gfx::LineJoin::None,
			dash: None,
			color: c,
		});
		return self
	}
	pub fn line_join(mut self, j: gfx::LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.width = w;
		}
		return self;
	}
// 	pub fn radius(mut self, r: f32) -> Self {
// 		self.radius = Some(r);
// 		return self
// 	}
}

pub fn polygon(pts: &[Vec2]) -> Polygon {

	return Polygon {
		pts: pts.to_vec(),
		fill: Some(color!()),
		stroke: None,
		radius: None,
	};

}

// TODO: first polygon isn't drawing
impl Drawable for Polygon {

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

		if let Some(color) = self.fill {

			let mut verts = Vec::with_capacity(pts.len() * gfx::Vertex2D::STRIDE);
			let mut indices = Vec::with_capacity((pts.len() - 2) * 3);

			for (i, p) in pts.iter().enumerate() {

				gfx::Vertex2D {
					pos: ctx.transform * *p,
					uv: vec2!(0),
					color: color,
				}.push(&mut verts);

				if i >= 2 {
					indices.extend_from_slice(&[0, (i as u32 - 1), i as u32]);
				}

			}

			ctx.renderer_2d.push(&verts, &indices, &ctx.cur_pipeline_2d, &gfx::Uniform2D {
				proj: ctx.proj_2d,
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_2d.clone(),
			})?;

		}

		if let Some(stroke) = &self.stroke {

			// TODO: line join
			for i in 0..pts.len() {

				let p1 = pts[i];
				let p2 = pts[(i + 1) % pts.len()];

				use gfx::LineJoin::*;

				match stroke.join {
					None => {
						ctx.draw(line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Bevel => {
						// TODO
						ctx.draw(line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Miter => {
						// TODO
						ctx.draw(line(p1, p2).width(stroke.width).color(stroke.color))?;
					},
					Round => {
						ctx.draw(line(p1, p2).width(stroke.width).color(stroke.color).cap(gfx::LineCap::Round))?;
					},
				}

			}

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

impl Drawable for Gradient {

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

			Vertex2D {
				pos: matrix * vec2!(-w / 2.0, -h / 2.0 + h * s.1),
				uv: vec2!(0),
				color: s.0,
			}.push(&mut verts);

			Vertex2D {
				pos: matrix * vec2!(w / 2.0, -h / 2.0 + h * s.1),
				uv: vec2!(0),
				color: s.0,
			}.push(&mut verts);

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

		ctx.renderer_2d.push(&verts, &indices, &ctx.cur_pipeline_2d, &gfx::Uniform2D {
			proj: ctx.proj_2d,
			tex: ctx.empty_tex.clone(),
			custom: ctx.cur_custom_uniform_2d.clone(),
		})?;

		return Ok(());

	}

}

pub struct Rect {
	p1: Vec2,
	p2: Vec2,
	radius: Option<f32>,
	fill: Option<Color>,
	stroke: Option<Stroke>,
}

pub fn rect(p1: Vec2, p2: Vec2) -> Rect {
	return Rect::new(p1, p2);
}

impl Rect {
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = Some(r);
		return self
	}
	pub fn fill(mut self, c: Color) -> Self {
		self.fill = Some(c);
		return self;
	}
	pub fn no_fill(mut self) -> Self {
		self.fill = None;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		if let Some(fill) = &mut self.fill {
			fill.a = a;
		}
		if let Some(stroke) = &mut self.stroke {
			stroke.color.a = a;
		}
		return self;
	}
	pub fn new(p1: Vec2, p2: Vec2) -> Self {
		return Rect {
			p1: p1,
			p2: p2,
			radius: None,
			stroke: None,
			fill: Some(color!(1)),
		};
	}
	pub fn stroke(mut self, c: Color) -> Self {
		self.stroke = Some(Stroke {
			width: 1.0,
			join: gfx::LineJoin::None,
			dash: None,
			color: c,
		});
		return self
	}
	pub fn line_join(mut self, j: gfx::LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.width = w;
		}
		return self;
	}
	pub fn from_size(w: f32, h: f32) -> Self {
		return Self::new(vec2!(w, h) * -0.5, vec2!(w, h) * 0.5);
	}
}

impl Drawable for Rect {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let pts = vec![
			self.p1,
			vec2!(self.p2.x, self.p1.y),
			self.p2,
			vec2!(self.p1.x, self.p2.y),
		];

		let poly = Polygon {
			pts: pts.to_vec(),
			fill: self.fill,
			stroke: self.stroke.clone(),
			radius: self.radius,
		};

		ctx.draw(poly)?;

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

impl Drawable for Line {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let len = (self.p2 - self.p1).mag();
		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);

		ctx.push(&gfx::t()

			.translate((self.p1 + self.p2) * 0.5)
			.rotate(rot)

		, |ctx| {

			let w = len;
			let h = self.width;

			ctx.draw(Rect::from_size(w, h).fill(self.color))?;

			if let gfx::LineCap::Round = self.cap {
				ctx.draw(circle(vec2!(-w / 2.0, 0), h / 2.0))?;
				ctx.draw(circle(vec2!(w / 2.0, 0), h / 2.0))?;
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

pub enum PointMode {
	Rect,
	Circle,
}

pub struct Points<'a> {
	pts: &'a[Vec2],
	size: f32,
	mode: PointMode,
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
	pub fn mode(mut self, m: PointMode) -> Self {
		self.mode = m;
		return self;
	}
}

pub fn pts<'a>(pts: &'a[Vec2]) -> Points<'a> {
	return Points {
		pts: pts,
		size: 1.0,
		color: color!(1),
		mode: PointMode::Rect,
	};
}

impl<'a> Drawable for Points<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		for pt in self.pts {
			ctx.push(&gfx::t()
				.translate(*pt)
			, |ctx| {
				return ctx.draw(Rect::from_size(self.size, self.size).fill(self.color));
			})?;
		}

		return Ok(());

	}

}

pub struct Circle {
	center: Vec2,
	radius: f32,
	segments: Option<u32>,
	stroke: Option<Stroke>,
	fill: Option<Color>,
	range: (f32, f32),
}

impl Circle {
	pub fn fill(mut self, c: Color) -> Self {
		self.fill = Some(c);
		return self;
	}
	pub fn no_fill(mut self) -> Self {
		self.fill = None;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		if let Some(fill) = &mut self.fill {
			fill.a = a;
		}
		if let Some(stroke) = &mut self.stroke {
			stroke.color.a = a;
		}
		return self;
	}
	pub fn stroke(mut self, c: Color) -> Self {
		self.stroke = Some(Stroke {
			width: 1.0,
			join: gfx::LineJoin::None,
			dash: None,
			color: c,
		});
		return self;
	}
	pub fn line_join(mut self, j: gfx::LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.width = w;
		}
		return self;
	}
	pub fn segments(mut self, s: u32) -> Self {
		self.segments = Some(s);
		return self
	}
	pub fn range(mut self, p1: f32, p2: f32) -> Self {
		self.range = (p1, p2);
		return self
	}
}

pub fn circle(center: Vec2, radius: f32) -> Circle {
	return Circle {
		center: center,
		radius: radius,
		segments: None,
		stroke: None,
		fill: Some(color!(1)),
		range: (0.0, 2.0 * PI),
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

	let (start, end) = if end < start {
		(end, start)
	} else {
		(start, end)
	};

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

impl Drawable for Circle {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if self.radius < 0.0 {
			return Ok(());
		}

		let p1 = self.range.0.max(0.0).min(PI * 2.0);
		let p2 = self.range.1.max(0.0).min(PI * 2.0);

		let mut pts = arc_verts(self.radius, p1, p2, self.segments);

		if p1 != 0.0 || p2 != PI * 2.0 {
			pts.push(self.center);
		}

		ctx.push(&gfx::t()
			.translate(self.center)
		, |ctx| {

			let poly = Polygon {
				pts: pts,
				fill: self.fill,
				stroke: self.stroke.clone(),
				radius: None,
			};

			ctx.draw(poly)?;

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

impl<'a> Drawable for Canvas<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.push(&gfx::t()
			.scale(vec2!(1.0 / ctx.dpi() as f32))
		, |ctx| {
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

impl<'a> Drawable for Model<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		for m in &self.model.meshes {
			m.draw(&ctx.cur_pipeline_3d, Some(&gfx::Uniform3D {
				proj: ctx.proj_3d,
				view: ctx.view_3d,
				model: ctx.transform,
				color: self.color,
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_3d.clone(),
			}));
		}

		return Ok(());

	}

}

pub struct Cube;

pub fn cube() -> Cube {
	return Cube;
}

impl Drawable for Cube {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_calls += 1;

		ctx.cube_renderer.draw(&ctx.cur_pipeline_3d, Some(&gfx::Uniform3D {
			proj: ctx.proj_3d,
			view: ctx.view_3d,
			model: ctx.transform,
			color: color!(),
			tex: ctx.empty_tex.clone(),
			custom: ctx.cur_custom_uniform_3d.clone(),
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

impl<'a> Drawable for Sprite3D<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);
		let offset = self.offset * -0.5;

		ctx.push(&gfx::t()
			.scale_3d(vec3!(scale.x, scale.y, 1.0))
			.translate_3d(vec3!(offset.x, offset.y, 0.0))
		, |ctx| {

			let shape = gfx::FlagShape {
				transform: ctx.transform.as_mat4(),
				quad: self.quad,
				color: self.color,
				flip: self.flip,
			};

			ctx.renderer_3d.push_shape(shape, &ctx.cur_pipeline_3d, &gfx::Uniform3D {
				proj: ctx.proj_3d,
				view: ctx.view_3d,
				model: gfx::Transform::new(),
				color: color!(),
				tex: self.tex.clone(),
				custom: ctx.cur_custom_uniform_3d.clone(),
			})?;

			return Ok(());

		})?;

		return Ok(());

	}

}

