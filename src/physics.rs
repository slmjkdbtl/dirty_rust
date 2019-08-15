// wengwengweng

use crate::Result;
use crate::math::*;
use crate::geom;

pub struct World2D {
	bodies: Vec<Body2D>,
}

impl World2D {
	// ...
}

pub struct Body2D {
	shape: geom::Shape2D,
}

