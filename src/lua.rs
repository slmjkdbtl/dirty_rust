// wengwengweng

//! Lua Bindings

use std::path::Path;

use rlua::Lua;
use rlua::UserData;
use rlua::AnyUserData;
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

trait ContextExt<'lua> {

	fn add_module<T: ToLua<'lua>>(&self, name: &str, val: T) -> rlua::Result<()>;
	fn add_lua_module(&self, name: &str, code: &str) -> rlua::Result<()>;

}

impl<'lua> ContextExt<'lua> for Context<'lua> {

	fn add_lua_module(&self, name: &str, code: &str) -> rlua::Result<()> {
		return self.add_module(name, self.load(code).eval::<Value>()?);
	}

	fn add_module<T: ToLua<'lua>>(&self, name: &str, val: T) -> rlua::Result<()> {

		let preloads: Table = self.globals().get::<_, Table>("package")?.get("preload")?;

		let f = self.create_function(|_, (v): (Value)| {
			return Ok(v);
		})?;

		let key = self.create_registry_value(val)?;

		preloads.set(name, f.bind(self.registry_value::<Value>(&key)?)?)?;
		self.remove_registry_value(key)?;

		return Ok(());

	}

}

pub fn bind(ctx: &Context) -> Result<()> {

	let globals = ctx.globals();
	let fs = ctx.create_table()?;
	let window = ctx.create_table()?;
	let http = ctx.create_table()?;
	let img = ctx.create_table()?;
	let audio = ctx.create_table()?;
	let joystick = ctx.create_table()?;
	let term = ctx.create_table()?;
	let ansi = ctx.create_table()?;

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
		return Ok(fs::read(&path)?);
	})?)?;

	fs.set("async_read", ctx.create_function(|_, (path): (String)| {
		return Ok(thread::exec(move || {
			return fs::read(&path).unwrap();
		}));
	})?)?;

	fs.set("read_str", ctx.create_function(|_, (path): (String)| {
		return Ok(fs::read_str(&path)?);
	})?)?;

	fs.set("async_read_str", ctx.create_function(|_, (path): (String)| {
		return Ok(thread::exec(move || {
			return fs::read_str(&path).unwrap();
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

	fs.set("write", ctx.create_function(|_, (path, content): (String, Vec<u8>)| {
		return Ok(fs::write(&path, &content)?);
	})?)?;

	fs.set("write_str", ctx.create_function(|_, (path, content): (String, String)| {
		return Ok(fs::write(&path, &content)?);
	})?)?;

	fs.set("size", ctx.create_function(|_, (path): (String)| {
		return Ok(fs::size(&path)?);
	})?)?;

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

	impl UserData for window::Ctx {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("fps", |ctx, c: &window::Ctx, ()| {
				return Ok(c.fps());
			});

			methods.add_method("time", |ctx, c: &window::Ctx, ()| {
				return Ok(c.time());
			});

		}

	}

	impl UserData for &mut window::Ctx {}

	impl UserData for window::Window {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method_mut("run", |ctx, win: &mut window::Window, (cb): (rlua::Function)| {
				return ctx.scope(|scope| {
					return Ok(win.run(|_| {
						cb.call::<_, ()>(());
					})?);
				});
			});


		}

	}

	window.set("make", ctx.create_function(|_, (conf): (Value)| {
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
		return Ok(audio::Sound::from_bytes(&fs::read(&path)?)?);
	})?)?;

	audio.set("async_load_file", ctx.create_function(|_, (path): (String)| {
		return Ok(thread::exec(move || {
			return audio::Sound::from_bytes(&fs::read(&path).unwrap()).unwrap();
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

	struct Term {
		term: console::Term,
	}

	impl UserData for Term {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("clear_screen", |_, t: &Term, ()| {
				return Ok(t.term.clear_screen().map_err(|e| Error::from(e))?);
			});

			methods.add_method("clear_line", |_, t: &Term, ()| {
				return Ok(t.term.clear_line().map_err(|e| Error::from(e))?);
			});

			methods.add_method("clear_last_lines", |_, t: &Term, (u)| {
				return Ok(t.term.clear_last_lines(u).map_err(|e| Error::from(e))?);
			});

			methods.add_method("write_line", |_, t: &Term, (s): (String)| {
				return Ok(t.term.write_line(&s).map_err(|e| Error::from(e))?);
			});

			methods.add_method("read_char", |_, t: &Term, ()| {
				return Ok(t.term.read_char().map_err(|e| Error::from(e))?.to_string());
			});

			methods.add_method("read_line", |_, t: &Term, ()| {
				return Ok(t.term.read_line().map_err(|e| Error::from(e))?);
			});

			methods.add_method("move_cursor_up", |_, t: &Term, (u)| {
				return Ok(t.term.move_cursor_up(u).map_err(|e| Error::from(e))?);
			});

			methods.add_method("move_cursor_down", |_, t: &Term, (u)| {
				return Ok(t.term.move_cursor_down(u).map_err(|e| Error::from(e))?);
			});

		}

	}

	term.set("session", ctx.create_function(|_, (): ()| {
		return Ok(Term { term: console::Term::stdout() });
	})?)?;

	macro_rules! wrap_ansi {
		($name:ident) => {
			ansi.set(stringify!($name), ctx.create_function(|_, (s): (String)| {
				return Ok(ansi::$name(&s));
			})?)?;
		}
	}

	wrap_ansi!(black);
	wrap_ansi!(red);
	wrap_ansi!(green);
	wrap_ansi!(yellow);
	wrap_ansi!(blue);
	wrap_ansi!(magenta);
	wrap_ansi!(cyan);
	wrap_ansi!(white);
	wrap_ansi!(bold);
	wrap_ansi!(italic);

	impl UserData for math::Vec3 {}
	impl UserData for math::Color {}

	globals.set("vec2", ctx.create_function(|_, (x, y): (f32, f32)| {
		return Ok(vec2!(x, y));
	})?)?;

	globals.set("color", ctx.create_function(|_, (r, g, b, a): (f32, f32, f32, f32)| {
		return Ok(color!(r, g, b, a));
	})?)?;

	globals.set("sleep", ctx.create_function(|_, (t): (u64)| {
		return Ok(std::thread::sleep(std::time::Duration::from_millis(t)));
	})?)?;

	ctx.add_module("fs", fs)?;
	ctx.add_module("window", window)?;
	ctx.add_module("http", http)?;
	ctx.add_module("img", img)?;
	ctx.add_module("audio", audio)?;
	ctx.add_module("joystick", joystick)?;
	ctx.add_module("term", term)?;
	ctx.add_module("ansi", ansi)?;
	ctx.add_lua_module("json", include_str!("res/json.lua"))?;

	return Ok(());

}

pub fn run(code: &str, fname: Option<impl AsRef<Path>>, args: Option<&[String]>) -> Result<()> {

	let lua = Lua::new();

	let args = match args {
		Some(a) => a.to_vec(),
		None => vec![],
	};

	return lua.context(|ctx| {

		ctx.globals().set("arg", args)?;
		bind(&ctx)?;

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

