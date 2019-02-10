// wengwengweng

macro_rules! ctx {

	($state:ident: $type:ty) => {

		#[allow(unused_variables)]
		static mut $state: Option<$type> = None;
		static CTX_NAME: &str = stringify!($state);

		#[allow(dead_code)]
		fn ctx_init(c: $type) {
			unsafe {
				if $state.is_none() {
					$state = Some(c);
				} else {
					panic!("cannot initialize {} twice", CTX_NAME);
				}
			}
		}

		#[allow(dead_code)]
		pub(self) fn ctx_get() -> &'static $type {
			unsafe {
				return $state.as_ref().expect(&format!("{} not initialized", CTX_NAME));
			}
		}

		#[allow(dead_code)]
		pub(self) fn ctx_get_mut() -> &'static mut $type {
			unsafe {
				return $state.as_mut().expect(&format!("{} not initialized", CTX_NAME));
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

