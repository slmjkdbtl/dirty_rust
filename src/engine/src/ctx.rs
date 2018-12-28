// wengwengweng

#[macro_export]
macro_rules! create_ctx {

	($name:ident: $type:ty) => (

		static mut $name: Option<$type> = None;

		fn get_ctx() -> &'static $type {
			unsafe {
				match &$name {
					Some(g) => {
						return g;
					}
					None => {
						panic!("ctx not initialized");
					},
				}
			}
		}

		fn get_ctx_mut() -> &'static mut $type {
			unsafe {
				match &mut $name {
					Some(g) => {
						return g;
					}
					None => {
						panic!("ctx not initialized");
					},
				}
			}
		}

	)

}

#[macro_export]
macro_rules! init_ctx {

	($name:ident -> $ctx:expr) => {

		unsafe {
			match &$name {
				Some(_) => {
					panic!("cannot init twice");
				}
				None => {
					$name = Some($ctx);
				}
			}

		}

	}

}


