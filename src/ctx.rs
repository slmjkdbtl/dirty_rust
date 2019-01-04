// wengwengweng

macro_rules! ctx {

	($name:ident: $type:ty) => {

		static mut $name: Option<$type> = None;

		fn ctx_init(c: $type) {
			unsafe {
				$name = Some(c);
			}
		}

		fn ctx_get() -> &'static $type {
			unsafe {
				return $name.as_ref().expect("ctx not initialized");
			}
		}

		fn ctx_get_mut() -> &'static mut $type {
			unsafe {
				return $name.as_mut().expect("ctx not initialized");
			}
		}

		fn ctx_is_ok() -> bool {
			unsafe {
				return $name.is_some();
			}
		}

	}

}

