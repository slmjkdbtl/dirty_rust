// wengwengweng

macro_rules! nested_macro {

	($($body:tt)*) => {

		macro_rules! __nested_macro {
			$($body)*
		}

		__nested_macro!($);

	};

}

macro_rules! fail {

	($($arg:tt)*) => {

		eprintln!($($arg)*);
		std::process::exit(1);

	};

}

