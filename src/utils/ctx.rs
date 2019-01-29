// wengwengweng

macro_rules! ctx {

	($state:ident: $type:ty) => {

		#[allow(unused_variables)]
		static mut $state: Option<$type> = None;

		#[allow(dead_code)]
		fn ctx_init(c: $type) {
			unsafe {
				if $state.is_none() {
					$state = Some(c);
				} else {
					panic!("cannot initialize {} twice", stringify!($state).to_lowercase());
				}
			}
		}

		#[allow(dead_code)]
		pub(self) fn ctx_get() -> &'static $type {
			unsafe {
				return $state.as_ref().expect(&format!("{} not initialized", stringify!($state).to_lowercase()));
			}
		}

		#[allow(dead_code)]
		pub(self) fn ctx_get_mut() -> &'static mut $type {
			unsafe {
				return $state.as_mut().expect(&format!("{} not initialized", stringify!($state).to_lowercase()));
			}
		}

		#[allow(dead_code)]
		pub(self) fn ctx_ok() -> bool {
			unsafe {
				return $state.is_some();
			}
		}

	}

}

