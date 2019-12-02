// wengwengweng

//! Common Drawing Primitives

use std::mem;
use std::f32::consts::PI;

use crate::Result;
use crate::Error;
use super::math::*;
use super::gfx;
use super::app::Ctx;
use super::gl;
use gfx::Drawable;
use gl::VertexLayout;

export!(sprite);
export!(texfill);
export!(polygon);
export!(rect);
export!(line);
export!(circle);
export!(points);
export!(canvas);
export!(gradient);
export!(text);
export!(spline);
export!(checkerboard);
export!(model);
export!(skybox);
export!(cube);
export!(line3d);
export!(rect3d);
export!(circle3d);
export!(sprite3d);

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

