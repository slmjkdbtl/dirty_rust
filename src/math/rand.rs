// wengwengweng

#[allow(missing_docs)]
#[macro_export]
macro_rules! rand {

	() => {
		rand::random::<f32>()
	};

	($x:expr) => {
		rand!() * $x as f32
	};

	($from:expr, $to:expr) => {
		$from as f32 + rand!() * ($to - $from) as f32
	};

}

