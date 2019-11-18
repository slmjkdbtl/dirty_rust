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

	pub fn from_sprite(s: &Sprite) -> Self {
		return Self::new(&s.verts());
	}

	pub fn apply(&mut self, t: &gfx::Transform) {
		self.gverts = self.verts
			.iter()
			.map(|p| *t * *p)
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

		return geom::overlaps(
			Point(p),
			Polygon(&self.gverts),
		);

	}

	pub fn collides(&self, other: &Self) -> (bool, Vec2) {

		return geom::sat(
			&self.gverts,
			&other.gverts,
		);

	}

	pub fn overlaps(&self, other: &Self) -> bool {

		use geom::Shape2D::*;

		return geom::overlaps(
			Polygon(&self.gverts),
			Polygon(&other.gverts),
		);

	}

}

impl gfx::Drawable for Area {

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

// 		ctx.reset(|ctx| {
			ctx.draw(
				&shapes::polygon(&self.verts)
					.no_fill()
					.stroke(rgba!(0, 1, 1, 1))
			)?;
// 			return Ok(());
// 		})?;

		return Ok(());

	}

}

