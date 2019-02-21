// wengwengweng

macro_rules! export {

	($name:ident) => {
		mod $name;
		pub use $name::*;
	};

	($name:ident, [$($thing:ident),*$(,)?]) => {
		mod $name;
		$(
			pub use $name::$thing;
		)*
	};

}

export!(camera);

