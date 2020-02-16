// wengwengweng

//! Misc Helpers

pub use once_cell;

#[allow(unused_macros)]
macro_rules! export {
	($name:ident) => {
		mod $name;
		pub use $name::*;
	}
}

#[allow(unused_macros)]
macro_rules! mexport {
	($name:ident) => {
		#[macro_use]
		mod $name;
		pub use $name::*;
	}
}

pub mod term;
mexport!(colmac);

use std::panic;

pub fn set_panic<F: 'static + Fn(Option<&str>, Option<&panic::Location>) + Send + Sync>(f: F) {

	panic::set_hook(Box::new(move |info: &panic::PanicInfo| {

		let msg: Option<&str> = if let Some(s) = info.payload().downcast_ref::<&str>() {
			Some(*s)
		} else if let Some(s) = info.payload().downcast_ref::<String>() {
			Some(&s)
		} else {
			None
		};

		f(msg, info.location());

	}));

}

