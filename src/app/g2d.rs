// wengwengweng

use std::rc::Rc;

use crate::*;
use super::gl;

use gl::VertexLayout;
use gl::Shape;

const MAX_DRAWS: usize = 65536;

// pub struct Ctx {
// 	device: Rc<gl::Device>,
// 	quad_renderer: QuadRenderer,
// }

// impl Ctx {
// 	pub fn new(gfx: &gfx::Ctx) -> Result<Self> {
// 		let device = gfx.device.clone();
// 		return Ok(Self {
// 			quad_renderer: QuadRenderer::new(&device, MAX_DRAWS)?,
// 			device: device,
// 		});
// 	}
// }

pub struct Shader {
	// ...
}

