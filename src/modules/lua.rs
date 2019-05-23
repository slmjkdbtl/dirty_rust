// wengwengweng

//! Lua Bindings

use std::path::Path;
use crate::*;
use crate::err::Error;

use rlua::Lua;
use rlua::Table;
use rlua::UserData;
use rlua::UserDataMethods;
use rlua::MetaMethod;
use rlua::Result;

impl From<Error> for rlua::Error {
	fn from(err: Error) -> rlua::Error {
		return rlua::Error::RuntimeError(format!("{}", err));
	}
}

pub fn run(code: &str, fname: Option<impl AsRef<Path>>, args: Option<&[String]>) -> Result<()> {

	let lua = Lua::new();

	let args = match args {
		Some(a) => a.to_vec(),
		None => vec![],
	};

	return lua.context(|ctx| {

		let globals = ctx.globals();
		let fs = ctx.create_table()?;
		let win = ctx.create_table()?;
		let http = ctx.create_table()?;
		let img = ctx.create_table()?;

		globals.set("arg", args)?;

		fs.set("glob", ctx.create_function(|_, (pat): (String)| {
			return Ok(fs::glob(&pat));
		})?)?;

		fs.set("copy", ctx.create_function(|_, (p1, p2): (String, String)| {
			return Ok(fs::copy(&p1, &p2)?);
		})?)?;

		fs.set("mkdir", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::mkdir(&path)?);
		})?)?;

		fs.set("is_file", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::is_file(&path));
		})?)?;

		fs.set("is_dir", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::is_dir(&path));
		})?)?;

		fs.set("exists", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::exists(&path));
		})?)?;

		fs.set("read", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::read_str(&path));
		})?)?;

		fs.set("read_bytes", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::read_bytes(&path));
		})?)?;

		fs.set("basename", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::basename(&path));
		})?)?;

		fs.set("remove", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::remove(&path)?);
		})?)?;

		fs.set("remove_dir", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::remove_dir(&path)?);
		})?)?;

		fs.set("rename", ctx.create_function(|_, (old, new): (String, String)| {
			return Ok(fs::rename(&old, &new)?);
		})?)?;

		fs.set("write", ctx.create_function(|_, (path, content): (String, String)| {
			return Ok(fs::write(&path, &content)?);
		})?)?;

		fs.set("write_bytes", ctx.create_function(|_, (path, content): (String, Vec<u8>)| {
			return Ok(fs::write(&path, &content)?);
		})?)?;

		impl UserData for &mut window::Ctx {}

		impl UserData for window::Ctx {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method_mut("close", |_, ctx: &mut window::Ctx, ()| {
					return Ok(ctx.close());
				});

				methods.add_method("fps", |_, ctx: &window::Ctx, ()| {
					return Ok(ctx.fps());
				});

				methods.add_method("time", |_, ctx: &window::Ctx, ()| {
					return Ok(ctx.time());
				});

				methods.add_method("dt", |_, ctx: &window::Ctx, ()| {
					return Ok(ctx.dt());
				});

			}

		}

		impl UserData for window::Window {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method_mut("run", |_, win: &mut window::Window, (f): (rlua::Function)| {
					return Ok(win.run(|ctx| {
// 						let res = f.call::<_, ()>(ctx);
// 						dbg!(res);
					})?);
				});

			}

		}

		win.set("create", ctx.create_function(|_, (conf): (rlua::Value)| {
			return Ok(window::Window::new(window::Conf::default()));
		})?)?;

// 		win.set("run", ctx.create_function(|_, (f): (rlua::Function)| {
// 			return Ok(window::run(|| {
// 				if f.call::<_, ()>(()).is_err() {
// 					panic!("failed to run");
// 				}
// 			}));
// 		})?)?;

// 		win.set("fps", ctx.create_function(|_, (): ()| {
// 			return Ok(window::fps());
// 		})?)?;

// 		win.set("dt", ctx.create_function(|_, (): ()| {
// 			return Ok(window::dt());
// 		})?)?;

		impl UserData for http::Response {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method("text", |_, res: &http::Response, ()| {
					return Ok(res.text().clone());
				});

				methods.add_method("bytes", |_, res: &http::Response, ()| {
					return Ok(res.bytes().clone());
				});

				methods.add_method("status", |_, res: &http::Response, ()| {
					return Ok(res.status());
				});

			}

		}

		http.set("get", ctx.create_function(|_, (uri): (String)| {
			return Ok(http::get(&uri)?);
		})?)?;

		impl UserData for img::Image {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method("write_png", |_, img: &img::Image, (fname): (String)| {
					return Ok(img::Image::write_png(img, fname)?);
				});

				methods.add_method("width", |_, img: &img::Image, ()| {
					return Ok(img.width());
				});

				methods.add_method("height", |_, img: &img::Image, ()| {
					return Ok(img.height());
				});

				methods.add_method("pixels", |_, img: &img::Image, ()| {
					return Ok(img.pixels().clone());
				});

			}

		}

		img.set("decode_png", ctx.create_function(|_, (data): (Vec<u8>)| {
			return Ok(img::decode_png(&data)?);
		})?)?;

		impl UserData for math::Vec2 {}
		impl UserData for math::Vec3 {}
		impl UserData for math::Color {}

		globals.set("vec2", ctx.create_function(|_, (x, y): (f32, f32)| {
			return Ok(vec2!(x, y));
		})?)?;

		globals.set("color", ctx.create_function(|_, (r, g, b, a): (f32, f32, f32, f32)| {
			return Ok(color!(r, g, b, a));
		})?)?;

		globals.set("fs", fs)?;
		globals.set("win", win)?;
		globals.set("http", http)?;
		globals.set("img", img)?;

		let mut runtime = ctx.load(code);

		if let Some(fname) = fname {
			runtime = runtime.set_name(&format!("{}", fname.as_ref().display()))?;
		}

		let handle_err = |err: &rlua::Error| {

			use rlua::Error::*;

			match err {
				SyntaxError { message, .. } => eprintln!("{}", message),
				RuntimeError(m) => eprintln!("{}", m),
				MemoryError(m) => eprintln!("{}", m),
				GarbageCollectorError(m) => eprintln!("{}", m),
				ToLuaConversionError { from, to, .. } => {
					eprintln!("expected {}, found {}", to, from);
				},
				FromLuaConversionError { from, to, .. } => {
					eprintln!("expected {}, found {}", to, from);
				},
				RecursiveMutCallback => eprintln!("recursive callback error"),
				CallbackDestructed => eprintln!("callback destructed"),
				StackError => eprintln!("stack error"),
				BindError => eprintln!("bind error"),
				CoroutineInactive => eprintln!("coroutine inactive"),
				UserDataTypeMismatch => eprintln!("userdata type mismatch"),
				UserDataBorrowError => eprintln!("userdata borrow error"),
				UserDataBorrowMutError => eprintln!("user data borrow mut error"),
				MismatchedRegistryKey => eprintln!("mismatched registry key"),
				ExternalError(_) => eprintln!("external error"),
				_ => {},
			}

		};

		if let Err(err) = runtime.exec() {

			handle_err(&err);

			if let rlua::Error::CallbackError { traceback, cause } = err {
				handle_err(&cause);
				eprintln!("{}", traceback);
			}

		}

		return Ok(());

	});

}

pub fn run_file(path: impl AsRef<Path>, args: Option<&[String]>) {

	let path = path.as_ref();

	if let Ok(code) = std::fs::read_to_string(path) {
		run(&code, Some(path), args);
	} else {
		panic!("failed to read {}", path.display());
	}

}

