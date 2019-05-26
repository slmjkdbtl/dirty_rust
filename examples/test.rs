use rlua::*;

struct Window;
struct State {
	n: i32,
}

impl Window {
	fn run(&mut self, mut f: impl FnMut(&mut State)) {
		let mut state = State { n: 0 };
		loop {
			f(&mut state);
		}
	}
}

fn main() -> rlua::Result<()> {

	let lua = Lua::new();

	return lua.context(|ctx| {

// 		let mut s = State { n: 0 };
// 		let test = ctx.create_function(|_, (state): (AnyUserData)| {
// 			let mut state = state.borrow_mut::<State>().unwrap();
// 			state.n = 12;
// 			return Ok(());
// 		})?;

// 		ctx.scope(|s| {
// 			test.call::<_, ()>(&mut s);
// 		});

		let globals = ctx.globals();

		impl UserData for State {}
		impl UserData for &mut State {}

		impl UserData for Window {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method_mut("run", |ctx, win: &mut Window, (cb): (rlua::Function)| {
					return Ok(win.run(|state| {
						// how to make this compile?
						ctx.scope(|scope| {
							cb.call::<_, ()>(scope.create_nonstatic_userdata(state).unwrap()).expect("oh no");
						});
					}));
				});

			}

		}

		globals.set("create_window", ctx.create_function(|_, ()| {
			return Ok(Window);
		})?)?;

		ctx.load(r#"
			local win = create_window()
			win:run(function(state)
			end)
		"#).exec().expect("oh no");

		return Ok(());

	});

}

