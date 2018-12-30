// wengwengweng

#[macro_export]
macro_rules! nested_macro {

	($($body:tt)*) => {

		macro_rules! __nested_macro {
			$($body)*
		}

		__nested_macro!($);

	};

}

#[macro_export]
macro_rules! count_expr {

	() => {
		0
	};

	($_e: expr $(, $rest: expr)*) => {
		1 + count_expr!($($rest),*)
	};

}

