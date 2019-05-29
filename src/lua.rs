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
use crate::Error;

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

fn bind(ctx: &Context) -> Result<()> {

	let globals = ctx.globals();
	let fs = ctx.create_table()?;
	let data = ctx.create_table()?;
	let window = ctx.create_table()?;
	let gfx = ctx.create_table()?;
	let http = ctx.create_table()?;
	let img = ctx.create_table()?;
	let audio = ctx.create_table()?;
	let term = ctx.create_table()?;
	let mut pool = thread::Pool {};

	impl<'a, T: Send + Clone + 'static + for<'lua> ToLua<'lua>> UserData for thread::Task<T> {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method_mut("poll", |_, t: &mut thread::Task<T>, (): ()| {
				return Ok(t.poll());
			});

		}

	}

	fs.set("glob", ctx.create_function(|_, (pat): (String)| {
		return Ok(fs::glob(&pat)?);
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

// 	ctx.scope(|s| {
// 		return fs.set("async_read", s.create_function_mut(|_, (path): (String)| {
// 			return Ok(pool.exec(move || {
// 				return fs::read(&path).unwrap();
// 			}));
// 		})?);
// 	})?;

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

	fs.set("data_dir", ctx.create_function(|_, (org): (String)| {
		return Ok(format!("{}", fs::data_dir(&org)?.display()));
	})?)?;

	fs.set("join", ctx.create_function(|_, (a, b): (String, String)| {
		return Ok(format!("{}", fs::join(&a, &b).display()));
	})?)?;

	impl<'lua> FromLua<'lua> for window::Conf {

		fn from_lua(val: Value<'lua>, _: Context<'lua>) -> rlua::Result<Self> {

			let mut conf = Self::default();

			let t = match val {
				Value::Table(t) => t,
				_ => return Err(Error::Lua.into()),
			};

			macro_rules! set {
				($lt:ident[$lk:expr] -> $val:ident::$key:ident) => {
					if let Ok(v) = $lt.raw_get($lk) {
						$val.$key = v;
					}
				}
			}

			set!(t["width"] -> conf::width);
			set!(t["height"] -> conf::height);
			set!(t["title"] -> conf::title);
			set!(t["hidpi"] -> conf::hidpi);
			set!(t["resizable"] -> conf::resizable);
			set!(t["fullscreen"] -> conf::fullscreen);
			set!(t["always_on_top"] -> conf::always_on_top);
			set!(t["borderless"] -> conf::borderless);
			set!(t["transparent"] -> conf::transparent);
			set!(t["vsync"] -> conf::vsync);
			set!(t["hide_title"] -> conf::hide_title);
			set!(t["hide_titlebar_buttons"] -> conf::hide_titlebar_buttons);
			set!(t["fullsize_content"] -> conf::fullsize_content);
			set!(t["titlebar_transparent"] -> conf::titlebar_transparent);
			set!(t["cursor_hidden"] -> conf::cursor_hidden);
			set!(t["cursor_locked"] -> conf::cursor_locked);

			return Ok(conf);

		}

	}


	impl UserData for &mut window::Ctx {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("fps", |_, c, ()| {
				return Ok(c.fps());
			});

			methods.add_method("time", |_, c, ()| {
				return Ok(c.time());
			});

			methods.add_method("dt", |_, c, ()| {
				return Ok(c.dt());
			});

			methods.add_method_mut("close", |_, c, ()| {
				return Ok(c.close());
			});

			methods.add_method("key_pressed", |_, c, (k): (String)| {
				return Ok(c.key_pressed(window::str_to_key(&k).ok_or(Error::Window)?));
			});

			methods.add_method("key_down", |_, c, (k): (String)| {
				return Ok(c.key_down(window::str_to_key(&k).ok_or(Error::Window)?));
			});

			methods.add_method("key_released", |_, c, (k): (String)| {
				return Ok(c.key_released(window::str_to_key(&k).ok_or(Error::Window)?));
			});

			methods.add_method("mouse_pressed", |_, c, (m): (String)| {
				return Ok(c.mouse_pressed(window::str_to_mouse(&m).ok_or(Error::Window)?));
			});

			methods.add_method("mouse_down", |_, c, (m): (String)| {
				return Ok(c.mouse_down(window::str_to_mouse(&m).ok_or(Error::Window)?));
			});

			methods.add_method("mouse_released", |_, c, (m): (String)| {
				return Ok(c.mouse_released(window::str_to_mouse(&m).ok_or(Error::Window)?));
			});

			methods.add_method("mouse_pos", |_, c, ()| -> rlua::Result<math::Vec2> {
				return Ok(c.mouse_pos().into());
			});

			methods.add_method("mouse_delta", |_, c, ()| -> rlua::Result<math::Vec2> {
				return Ok(c.mouse_delta().unwrap_or(window::MouseDelta::new(0, 0)).into());
			});

			methods.add_method("scroll_delta", |_, c, ()| -> rlua::Result<math::Vec2> {
				return Ok(c.scroll_delta().unwrap_or(window::ScrollDelta::new(0, 0)).into());
			});

			methods.add_method("text_input", |_, c, ()| {
				return Ok(c.text_input().unwrap_or(String::new()));
			});

			methods.add_method_mut("set_fullscreen", |_, c, (b): (bool)| {
				return Ok(c.set_fullscreen(b));
			});

			methods.add_method("is_fullscreen", |_, c, ()| {
				return Ok(c.is_fullscreen());
			});

			methods.add_method_mut("toggle_fullscreen", |_, c, ()| {
				return Ok(c.toggle_fullscreen());
			});

			methods.add_method_mut("set_cursor_hidden", |_, c, (b): (bool)| {
				return Ok(c.set_cursor_hidden(b));
			});

			methods.add_method("is_cursor_hidden", |_, c, ()| {
				return Ok(c.is_cursor_hidden());
			});

			methods.add_method_mut("toggle_cursor_hidden", |_, c, ()| {
				return Ok(c.toggle_cursor_hidden());
			});

			methods.add_method_mut("set_cursor_locked", |_, c, (b): (bool)| {
				return Ok(c.set_cursor_locked(b));
			});

			methods.add_method("is_cursor_locked", |_, c, ()| {
				return Ok(c.is_cursor_locked());
			});

			methods.add_method_mut("toggle_cursor_locked", |_, c, ()| {
				return Ok(c.toggle_cursor_locked());
			});

			methods.add_method_mut("set_title", |_, c, (s): (String)| {
				return Ok(c.set_title(&s));
			});

			methods.add_method("title", |_, c, ()| {
				return Ok(c.title());
			});

		}

	}

	impl UserData for window::Window {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method_mut("run", |ctx, win, (cb): (rlua::Function)| {
				return Ok(win.run(|c| {
					ctx.scope(|s| -> rlua::Result<()> {
						return Ok(cb.call::<_, ()>(s.create_nonstatic_userdata(c)?)?);
					});
				})?);
			});


		}

	}

	window.set("make", ctx.create_function(|ctx, (conf): (Value)| {
		return Ok(window::Window::new(window::Conf::from_lua(conf, ctx)?));
	})?)?;

	impl UserData for gfx::Texture {}
	impl UserData for gfx::Canvas {}

	gfx.set("texture", ctx.create_function(|_, (d): (Vec<u8>)| {
		return Ok(gfx::Texture::from_bytes(&d)?);
	})?)?;

	gfx.set("canvas", ctx.create_function(|_, (w, h): (u32, u32)| {
		return Ok(gfx::Canvas::new(w, h));
	})?)?;

	impl UserData for http::Response {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("text", |_, res, ()| {
				return Ok(res.text().clone());
			});

			methods.add_method("bytes", |_, res, ()| {
				return Ok(res.bytes().clone());
			});

			methods.add_method("status", |_, res, ()| {
				return Ok(res.status());
			});

		}

	}

	http.set("get", ctx.create_function(|_, (uri): (String)| {
		return Ok(http::get(&uri)?);
	})?)?;

	impl UserData for img::Image {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("width", |_, img, ()| {
				return Ok(img.width());
			});

			methods.add_method("height", |_, img, ()| {
				return Ok(img.height());
			});

			methods.add_method("write", |_, img, (path): (String)| {
				return Ok(img.write(path)?);
			});

		}

	}

	img.set("load", ctx.create_function(|_, (data): (Vec<u8>)| {
		return Ok(img::Image::from_bytes(&data)?);
	})?)?;

	impl UserData for audio::Sound {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("play", |_, s, ()| {
				return Ok(s.play()?);
			});

			methods.add_method("speed", |_, s, (sp): (f32)| {
				return Ok(s.speed(sp));
			});

			methods.add_method("volume", |_, s, (v): (f32)| {
				return Ok(s.volume(v));
			});

			methods.add_method("repeat", |_, s, ()| {
				return Ok(s.repeat());
			});

			methods.add_method("fadein", |_, s, (f): (u64)| {
				return Ok(s.fadein(f));
			});

		}

	}

	impl UserData for audio::Track {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("resume", |_, t, ()| {
				return Ok(t.resume());
			});

			methods.add_method("pause", |_, t, ()| {
				return Ok(t.pause());
			});

		}

	}

	audio.set("load", ctx.create_function(|_, (data): (Vec<u8>)| {
		return Ok(audio::Sound::from_bytes(&data)?);
	})?)?;

	audio.set("load_file", ctx.create_function(|_, (path): (String)| {
		return Ok(audio::Sound::from_file(&path)?);
	})?)?;

	audio.set("async_load_file", ctx.create_function(|_, (path): (String)| {
		return Ok(thread::exec(move || {
			return audio::Sound::from_file(&path).unwrap();
		}));
	})?)?;

	impl UserData for term::Term {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("clear_screen", |_, t, ()| {
				return Ok(t.clear()?);
			});

			methods.add_method("clear_line", |_, t, ()| {
				return Ok(t.clear_line()?);
			});

			methods.add_method("write_line", |_, t, (s): (String)| {
				return Ok(t.write_line(&s)?);
			});

			methods.add_method("read_char", |_, t, ()| {
				return Ok(t.read_char()?.to_string());
			});

			methods.add_method("read_line", |_, t, ()| {
				return Ok(t.read_line()?);
			});

			methods.add_method("width", |_, t, ()| {
				return Ok(t.width());
			});

			methods.add_method("height", |_, t, ()| {
				return Ok(t.height());
			});

			methods.add_method("render_text", |_, t, (lines): (Vec<String>)| {
				return Ok(t.render_text(&lines));
			});

		}

	}

	term.set("session", ctx.create_function(|_, (): ()| {
		return Ok(term::Term::new());
	})?)?;

	macro_rules! wrap_ansi {
		($name:ident) => {
			term.set(stringify!($name), ctx.create_function(|_, (s): (String)| {
				return Ok(term::$name(&s));
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
	impl UserData for math::Vec4 {}
	impl UserData for math::Mat4 {}
	impl UserData for math::Color {}
	impl UserData for math::Rect {}

	globals.set("vec2", ctx.create_function(|_, (x, y): (f32, f32)| {
		return Ok(vec2!(x, y));
	})?)?;

	globals.set("vec3", ctx.create_function(|_, (x, y, z): (f32, f32, f32)| {
		return Ok(vec3!(x, y, z));
	})?)?;

	globals.set("vec4", ctx.create_function(|_, (x, y, z, w): (f32, f32, f32, f32)| {
		return Ok(vec4!(x, y, z, w));
	})?)?;

	globals.set("color", ctx.create_function(|_, (r, g, b, a): (f32, f32, f32, f32)| {
		return Ok(color!(r, g, b, a));
	})?)?;

	globals.set("rect", ctx.create_function(|_, (x, y, w, h): (f32, f32, f32, f32)| {
		return Ok(rect!(x, y, w, h));
	})?)?;

	globals.set("sleep", ctx.create_function(|_, (t): (u64)| {
		return Ok(std::thread::sleep(std::time::Duration::from_millis(t)));
	})?)?;

	ctx.add_module("fs", fs)?;
	ctx.add_module("window", window)?;
	ctx.add_module("gfx", gfx)?;
	ctx.add_module("http", http)?;
	ctx.add_module("img", img)?;
	ctx.add_module("audio", audio)?;
	ctx.add_module("term", term)?;
	ctx.add_lua_module("json", include_str!("res/json.lua"))?;

	return Ok(());

}

fn remove_shebang(code: &str) -> String {

	if let Some(fl) = code.lines().next() {
		if fl.get(0..2) == Some("#!") {
			return code
				.lines()
				.skip(1)
				.collect::<Vec<&str>>()
				.join("\n");
		}
	}

	return code.to_owned();

}

pub fn run(code: &str, path: Option<impl AsRef<Path>>, args: Option<&[String]>) -> Result<()> {

	let lua = Lua::new();

	let args = match args {
		Some(a) => a.to_vec(),
		None => vec![],
	};

	return lua.context(|ctx| {

		bind(&ctx)?;

		let code = remove_shebang(code);
		let mut runtime = ctx.load(&code);

		if let Some(path) = path {
			runtime = runtime.set_name(&format!("{}", path.as_ref().display()))?;
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

		if let Err(err) = runtime.call::<_, ()>(args) {

			handle_err(&err);

			if let rlua::Error::CallbackError { traceback, cause } = err {
				handle_err(&cause);
				eprintln!("{}", traceback);
			}

		}

		return Ok(());

	});

}

