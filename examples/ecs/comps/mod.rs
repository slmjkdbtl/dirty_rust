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

export!(trans);
export!(vel);
export!(sprite);
export!(body);
export!(powder);
export!(flower);
export!(petal);

