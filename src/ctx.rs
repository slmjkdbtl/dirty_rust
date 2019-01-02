// wengwengweng

macro_rules! ctx {

	($name:ident: $type:ty) => {

		static mut $name: Option<$type> = None;

		fn init_ctx(c: $type) {
			unsafe {
				$name = Some(c);
			}
		}

		fn get_ctx() -> &'static $type {
			unsafe {
				return $name.as_ref().expect("ctx not initialized");
			}
		}

		fn get_ctx_mut() -> &'static mut $type {
			unsafe {
				return $name.as_mut().expect("ctx not initialized");
			}
		}

	}

}

