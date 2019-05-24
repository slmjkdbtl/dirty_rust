// wengwengweng

//! Lua Bindings

use std::path::Path;

use rlua::Lua;
use rlua::UserData;
use rlua::UserDataMethods;
use rlua::MetaMethod;
use rlua::Value;
use rlua::Context;
use rlua::ToLua;
use rlua::FromLua;
use rlua::Table;

use crate::*;
use crate::err::Error;

impl From<Error> for rlua::Error {
	fn from(err: Error) -> rlua::Error {
		return rlua::Error::RuntimeError(format!("{}", err));
	}
}

impl From<rlua::Error> for Error {
	fn from(_: rlua::Error) -> Error {
		return Error::Lua;
	}
}

trait ContextExt {
	fn include_module(&self, name: &str, code: &str) -> rlua::Result<()>;
	fn add_module(&self, name: &str, val: Value) -> rlua::Result<()>;
}

impl<'lua> ContextExt for Context<'lua> {

	fn include_module(&self, name: &str, code: &str) -> rlua::Result<()> {
		return self.add_module(name, self.load(include_str!("res/json.lua")).eval()?);
	}

	fn add_module(&self, name: &str, val: Value<'_>) -> rlua::Result<()> {

		let preloads: Table = self.globals().get::<_, Table>("package")?.get("preload")?;

		let f = self.create_function(|_, (v): (Value)| {
			return Ok(v);
		})?;

// 		let key = self.create_registry_value(val)?;

// 		preloads.set(name, f.bind(self.registry_value::<Value>(&key)?)?)?;

		return Ok(());

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
		let window = ctx.create_table()?;
		let http = ctx.create_table()?;
		let img = ctx.create_table()?;
		let audio = ctx.create_table()?;

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
			return Ok(fs::read_str(&path)?);
		})?)?;

		impl<'a, T: Send + Clone + 'static + for<'lua> ToLua<'lua>> UserData for thread::Task<T> {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method("done", |_, t: &thread::Task<T>, ()| {
					return Ok(t.done());
				});

				methods.add_method_mut("poll", |_, t: &mut thread::Task<T>, (): ()| {
					return Ok(t.poll());
				});

				methods.add_method("data", |_, t: &thread::Task<T>, (): ()| {
					if t.done() {
						return Ok(t.data().unwrap());
					} else {
						return Err(Error::Lua.into());
					}
				});

			}

		}

		fs.set("async_read", ctx.create_function(|_, (path): (String)| {
			return Ok(thread::exec(move || {
				return fs::read_str(&path).unwrap();
			}));
		})?)?;

		fs.set("read_bytes", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::read_bytes(&path)?);
		})?)?;

		fs.set("async_read_bytes", ctx.create_function(|_, (path): (String)| {
			return Ok(thread::exec(move || {
				return fs::read_bytes(&path).unwrap();
			}));
		})?)?;

		fs.set("basename", ctx.create_function(|_, (path): (String)| {
			return Ok(fs::basename(&path)?);
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

		impl<'lua> FromLua<'lua> for window::Conf {

			fn from_lua(val: Value<'lua>, ctx: Context<'lua>) -> rlua::Result<Self> {

				let mut conf = Self::default();

				let t = match val {
					Value::Table(t) => t,
					_ => return Err(Error::Lua.into()),
				};

				return Ok(conf);

			}

		}

		impl UserData for window::Window {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method_mut("run", |_, win: &mut window::Window, (f): (rlua::Function)| {
					return Ok(win.run(|_| {
						let res = f.call::<_, ()>(());
					})?);
				});

			}

		}

		window.set("create", ctx.create_function(|_, (conf): (Value)| {
			return Ok(window::Window::new(window::Conf::default()));
		})?)?;

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

		impl UserData for audio::Sound {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method("play", |_, s: &audio::Sound, ()| {
					return Ok(audio::play(s));
				});

			}

		}

		audio.set("load", ctx.create_function(|_, (data): (Vec<u8>)| {
			return Ok(audio::Sound::from_bytes(&data)?);
		})?)?;

		audio.set("load_file", ctx.create_function(|_, (path): (String)| {
			return Ok(audio::Sound::from_bytes(&fs::read_bytes(&path)?)?);
		})?)?;

		audio.set("async_load_file", ctx.create_function(|_, (path): (String)| {
			return Ok(thread::exec(move || {
				return audio::Sound::from_bytes(&fs::read_bytes(&path).unwrap()).unwrap();
			}));
		})?)?;

		impl UserData for math::Vec2 {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_meta_method(MetaMethod::Index, |_, this, key: String| {
					match key.as_ref() {
						"x" => Ok(this.x),
						"y" => Ok(this.y),
						_ => Err(Error::Lua.into()),
					}
				});

			}

		}

		impl UserData for math::Vec3 {}
		impl UserData for math::Color {}

		globals.set("vec2", ctx.create_function(|_, (x, y): (f32, f32)| {
			return Ok(vec2!(x, y));
		})?)?;

		globals.set("color", ctx.create_function(|_, (r, g, b, a): (f32, f32, f32, f32)| {
			return Ok(color!(r, g, b, a));
		})?)?;

		let preloads: Table = ctx.globals().get::<_, Table>("package")?.get("preload")?;

		let f = ctx.create_function(|_, (v): (Value)| {
			return Ok(v);
		})?;

		let json: Value = ctx.load(include_str!("res/json.lua")).eval()?;

		let json_key = ctx.create_registry_value(json)?;
		let fs_key = ctx.create_registry_value(fs)?;
		let window_key = ctx.create_registry_value(window)?;
		let http_key = ctx.create_registry_value(http)?;
		let img_key = ctx.create_registry_value(img)?;
		let audio_key = ctx.create_registry_value(audio)?;

		preloads.set("json", f.bind(ctx.registry_value::<Value>(&json_key)?)?)?;
		preloads.set("fs", f.bind(ctx.registry_value::<Value>(&fs_key)?)?)?;
		preloads.set("window", f.bind(ctx.registry_value::<Value>(&window_key)?)?)?;
		preloads.set("http", f.bind(ctx.registry_value::<Value>(&http_key)?)?)?;
		preloads.set("img", f.bind(ctx.registry_value::<Value>(&img_key)?)?)?;
		preloads.set("audio", f.bind(ctx.registry_value::<Value>(&audio_key)?)?)?;

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

pub fn run_file(path: impl AsRef<Path>, args: Option<&[String]>) -> Result<()> {

	let path = path.as_ref();
	let code = std::fs::read_to_string(path)?;

	return Ok(run(&code, Some(path), args)?);

}

