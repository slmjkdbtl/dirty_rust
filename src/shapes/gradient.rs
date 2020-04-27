// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Gradient {
	p1: Vec2,
	p2: Vec2,
	steps: Vec<(Color, f32)>,
	width: f32,
}

impl Gradient {
	pub fn from(p1: Vec2, p2: Vec2, steps: &[(Color, f32)]) -> Gradient {
		return Self {
			p1: p1,
			p2: p2,
			steps: steps.to_vec(),
			width: 1.0,
		};
	}
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
		return self;
	}
}

pub fn gradient(p1: Vec2, p2: Vec2, steps: &[(Color, f32)]) -> Gradient {
	return Gradient::from(p1, p2, steps);
}

impl Drawable for Gradient {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		if self.steps.len() < 2 {
			return Err(format!("need at least 2 points to draw a gradient"));
		}

		use gfx::Vertex;

		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);
		let mut verts = Vec::with_capacity(4 + 2 * (self.steps.len() - 2) * gfx::Vertex::STRIDE);

		let matrix = ctx.transform
			.t2((self.p1 + self.p2) * 0.5)
			.r(rot - 90f32.to_radians())
			;

		let w = self.width;
		let h = Vec2::dist(self.p1, self.p2);

		let mut last_pos = None;

		for s in &self.steps {

			if (last_pos.is_none()) {
				if (s.1 != 0.0) {
					return Err(format!("gradient step should start at 0.0"));
				}
			}

			last_pos = Some(s.1);

			Vertex {
				pos: (matrix * vec3!(-w / 2.0, -h / 2.0 + h * s.1, 0.0)).xyz(),
				uv: vec2!(0),
				normal: vec3!(0, 0, -1),
				color: s.0,
			}.push(&mut verts);

			Vertex {
				pos: (matrix * vec3!(w / 2.0, -h / 2.0 + h * s.1, 0.0)).xyz(),
				uv: vec2!(0),
				normal: vec3!(0, 0, -1),
				color: s.0,
			}.push(&mut verts);

		}

		if (last_pos != Some(1.0)) {
			return Err(format!("gradient step should end at 1.0"));
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

		ctx.renderer.push(
			gl::Primitive::Triangle,
			&verts,
			&indices,
			&ctx.cur_pipeline,
			&gfx::Uniform {
				model: mat4!(),
				proj: ctx.proj,
				view: ctx.view,
				color: rgba!(1),
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform.clone(),
			}
		)?;

		return Ok(());

	}

}

