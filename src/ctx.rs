// wengwengweng

macro_rules! ctx {

	($state:ident: $type:ty) => {

		static mut $state: Option<$type> = None;

		fn ctx_init(c: $type) {
			unsafe {
				$state = Some(c);
			}
		}

		fn ctx_get() -> &'static $type {
			unsafe {
				return $state.as_ref().expect(&format!("{} not initialized", stringify!($state).to_lowercase()));
			}
		}

		fn ctx_get_mut() -> &'static mut $type {
			unsafe {
				return $state.as_mut().expect(&format!("{} not initialized", stringify!($state).to_lowercase()));
			}
		}

		fn ctx_is_ok() -> bool {
			unsafe {
				return $state.is_some();
			}
		}

	}

}

