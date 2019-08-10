// wengwengweng

use super::*;

pub struct StencilOps {
	pub sfail: StencilOp,
	pub dpfail: StencilOp,
	pub dppass: StencilOp,
}

pub struct StencilFunc {
	pub cmp: Cmp,
	pub rf: i32,
	pub mask: u32,
}

pub struct StencilDraw<F: Fn()> {
	pub ops: StencilOps,
	pub func: StencilFunc,
	pub draw: F,
}

