// wengwengweng

use std::f32::consts::PI;

use crate::gfx;
use crate::geom;
use crate::Result;
use crate::math::*;
use gfx::Gfx;
use gfx::Drawable;
use gfx::Vertex;
use gfx::Primitive;

export!(raw);
export!(sprite);
export!(uvrect);
export!(polygon);
export!(rect);
export!(line);
export!(circle);
export!(points);
export!(canvas);
export!(gradient);
export!(text);
export!(model);
export!(line3d);
export!(rect3d);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineDash {
	pub len: f32,
	pub interval: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineJoin {
	None,
	Round,
	Bevel,
	Miter,
}

#[derive(Debug, Clone, Copy)]
pub enum LineCap {
	Square,
	Butt,
	Round,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stroke {
	pub width: f32,
	pub join: LineJoin,
	pub dash: Option<LineDash>,
	pub color: Color,
}

