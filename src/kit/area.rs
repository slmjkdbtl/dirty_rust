// wengwengweng

use crate::*;
use app::*;
use super::*;
use sprite::*;
use crate::math::*;

#[derive(Clone)]
pub struct Area {
	verts: Vec<Vec2>,
	gverts: Vec<Vec2>,
}

impl Area {

	pub fn new(verts: &[Vec2]) -> Self {
		return Self {
			verts: verts.to_owned(),
			gverts: verts.to_owned(),
		};
	}

	pub fn from_rect(p1: Vec2, p2: Vec2) -> Self {
		return Self::new(&[
			p1,
			vec2!(p2.x, p1.y),
			p2,
			vec2!(p1.x, p2.y),
		]);
	}

	pub fn from_sprite(s: &Sprite) -> Self {
		return Self::new(&s.verts());
	}

	pub fn apply(&mut self, t: Mat4) {
		self.gverts = self.verts
			.iter()
			.map(|p| t * *p)
			.collect();
	}

	pub fn set_scale(&mut self, s: Vec2) {
		self.verts = self.verts
			.iter()
			.map(|p| *p * s)
			.collect();
	}

	pub fn set_offset(&mut self, o: Vec2) {
		self.verts = self.verts
			.iter()
			.map(|p| *p + o)
			.collect();
	}

	pub fn has_pt(&self, p: Vec2) -> bool {

		use geom::Shape2D::*;

		return geom::overlap2d(
			Point(p),
			Polygon(&self.gverts),
		);

	}

	pub fn collides(&self, other: &Self) -> (bool, Vec2) {

		return geom::sat2d(
			&self.gverts,
			&other.gverts,
		);

	}

	pub fn overlap2d(&self, other: &Self) -> bool {

		use geom::Shape2D::*;

		return geom::overlap2d(
			Polygon(&self.gverts),
			Polygon(&other.gverts),
		);

	}

	pub fn shape(&self) -> shapes::Polygon {
		return shapes::polygon(&self.verts)
			.no_fill()
			.stroke(rgba!(0, 1, 1, 1))
			;
	}

}

impl gfx::Drawable for Area {

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw(&self.shape())?;

		return Ok(());

	}

}

