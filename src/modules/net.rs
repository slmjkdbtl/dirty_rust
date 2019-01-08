// wengwengweng

//! Networking

use crate::*;

// context
ctx!(NET: NetCtx);

struct NetCtx {}

/// initialize network
pub fn init() {

	if !app::enabled() {
		panic!("can't init net without app");
	}

	ctx_init(NetCtx {});

}

