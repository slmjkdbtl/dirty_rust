// wengwengweng

#[macro_export]
macro_rules! create_context {

	($name:ident, $type:ident) => (

		static mut $name: Option<$type> = None;

		fn get_ctx() -> &'static $type {
			unsafe {
				match &$name {
					Some(g) => {
						return g;
					}
					None => {
						panic!("app not initialized");
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
						panic!("app not initialized");
					},
				}
			}
		}

	)

}

