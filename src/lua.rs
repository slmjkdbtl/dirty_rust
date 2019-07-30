// wengwengweng

//! Lua Bindings

use std::path::Path;

use rlua::Lua;
use rlua::UserData;
use rlua::UserDataMethods;
use rlua::MetaMethod;
use rlua::Value;
use rlua::ToLua;
use rlua::FromLua;
use rlua::Table;
use rlua::Context;

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
	fn add_module_from_lua(&self, name: &str, code: &str) -> rlua::Result<()>;

}

impl<'lua> ContextExt<'lua> for Context<'lua> {

	fn add_module_from_lua(&self, name: &str, code: &str) -> rlua::Result<()> {
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

#[cfg(feature = "fs")]
fn bind_fs(ctx: &Context) -> Result<()> {

	let fs = ctx.create_table()?;

// 	fs.set("glob", ctx.create_function(|_, (pat): (String)| {
// 		return Ok(fs::glob(&pat)?);
// 	})?)?;

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
// 		return fs.set("read_async", s.create_function_mut(|_, (path): (String)| {
// 			return Ok(pool.exec(move || {
// 				return fs::read(&path).unwrap();
// 			}));
// 		})?);
// 	})?;

	fs.set("read_async", ctx.create_function(|_, (path): (String)| {
		return Ok(thread::exec(move || {
			return fs::read(&path).ok();
		}));
	})?)?;

	fs.set("read_str", ctx.create_function(|_, (path): (String)| {
		return Ok(fs::read_str(&path)?);
	})?)?;

	fs.set("read_str_async", ctx.create_function(|_, (path): (String)| {
		return Ok(thread::exec(move || {
			return fs::read_str(&path).ok();
		}));
	})?)?;

	fs.set("basename", ctx.create_function(|_, (path): (String)| {
		return Ok(fs::basename(&path)?);
	})?)?;

	fs.set("extname", ctx.create_function(|_, (path): (String)| {
		return Ok(fs::extname(&path)?);
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

	ctx.add_module("fs", fs)?;

	return Ok(());

}

#[cfg(feature = "app")]
fn bind_app(ctx: &Context) -> Result<()> {

	let app = ctx.create_table()?;

	impl<'lua> FromLua<'lua> for app::Conf {

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
			set!(t["clear_color"] -> conf::clear_color);

			return Ok(conf);

		}

	}

	impl UserData for &mut app::Ctx {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			use app::*;

			methods.add_method("time", |_, ctx, ()| {
				return Ok(ctx.time());
			});

		}

	}

	impl UserData for app::Ctx {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			use app::*;

			methods.add_method_mut("init", |ctx, app, (cb): (rlua::Function)| {
				ctx.scope(|s| -> rlua::Result<()> {
					return Ok(cb.call::<_, ()>(s.create_nonstatic_userdata(app)?)?);
				});
				return Ok(());
			});

			methods.add_method_mut("run", |ctx, app, (cb): (rlua::Function)| {
				return Ok(app.run(|c| {
					ctx.scope(|s| -> rlua::Result<()> {
						return Ok(cb.call::<_, ()>(s.create_nonstatic_userdata(c)?)?);
					});
					return Ok(());
				})?);
			});

		}

	}

	app.set("make", ctx.create_function(|ctx, (conf): (Option<Value>)| {
		if let Some(conf) = conf {
			return Ok(app::Ctx::new(app::Conf::from_lua(conf, ctx)?)?);
		} else {
			return Ok(app::Ctx::new(app::Conf::default())?);
		}
	})?)?;

	ctx.add_module("app", app)?;

	return Ok(());

}

#[cfg(feature = "audio")]
fn bind_audio(ctx: &Context) -> Result<()> {

	let audio = ctx.create_table()?;

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

			methods.add_method("play", |_, t, ()| {
				return Ok(t.play());
			});

			methods.add_method("pause", |_, t, ()| {
				return Ok(t.pause());
			});

		}

	}

	audio.set("load", ctx.create_function(|_, (b): (Vec<u8>)| {
		return Ok(audio::Sound::from_bytes(&b)?);
	})?)?;

	audio.set("read", ctx.create_function(|_, (p): (String)| {
		return Ok(audio::Sound::from_file(&p)?);
	})?)?;

	audio.set("read_async", ctx.create_function(|_, (path): (String)| {
		return Ok(thread::exec(move || {
			return audio::Sound::from_file(&path).ok();
		}));
	})?)?;

	ctx.add_module("audio", audio)?;

	return Ok(());

}

#[cfg(feature = "img")]
fn bind_img(ctx: &Context) -> Result<()> {

	let img = ctx.create_table()?;

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

	img.set("load", ctx.create_function(|_, (b): (Vec<u8>)| {
		return Ok(img::Image::from_bytes(&b)?);
	})?)?;

	img.set("read", ctx.create_function(|_, (p): (String)| {
		return Ok(img::Image::from_file(&p)?);
	})?)?;

	ctx.add_module("img", img)?;

	return Ok(());

}

#[cfg(feature = "http")]
fn bind_http(ctx: &Context) -> Result<()> {

	let http = ctx.create_table()?;

	impl UserData for http::Status {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("code", |_, s, ()| {
				return Ok(s.code());
			});

		}

	}

	impl UserData for http::Response {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("text", |_, res, ()| {
				return Ok(res.text());
			});

			methods.add_method("bytes", |_, res, ()| {
				return Ok(res.bytes().to_vec());
			});

			methods.add_method("status", |_, res, ()| {
				return Ok(res.status());
			});

		}

	}

	impl UserData for http::Request {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method("path", |_, req, ()| {
				return Ok(req.path().to_owned());
			});

		}

	}

	http.set("get", ctx.create_function(|_, (uri): (String)| {
		return Ok(http::get(&uri)?);
	})?)?;

	http.set("post", ctx.create_function(|_, (uri, data): (String, Vec<u8>)| {
		return Ok(http::post(&uri, &data)?);
	})?)?;

	http.set("serve", ctx.create_function(|_, (loc, port, handler): (String, u16, rlua::Function)| {
		return Ok(http::serve(&loc, port, |req| {
			return handler.call::<_, http::Response>(req).ok().expect("response required");
		})?);
	})?)?;

	let res = ctx.create_table()?;

	macro_rules! gen_res {
		($fname:expr => $tname:ident($type:ty)) => {
			res.set($fname, ctx.create_function(|_, (data): ($type)| {
				return Ok(http::Response::new(http::Status::Ok, http::ContentType::$tname, &data));
			})?)?;
		};
	};

	gen_res!("text" => Text(String));
	gen_res!("html" => HTML(String));
	gen_res!("css" => CSS(String));
	gen_res!("javascript" => JavaScript(String));
	gen_res!("json" => JSON(String));
	gen_res!("markdown" => Markdown(String));
	gen_res!("png" => PNG(Vec<u8>));
	gen_res!("jpeg" => JPEG(Vec<u8>));
	gen_res!("gif" => GIF(Vec<u8>));
	gen_res!("pdf" => PDF(Vec<u8>));
	gen_res!("mp3" => MP3(Vec<u8>));
	gen_res!("ogg" => OGG(Vec<u8>));
	gen_res!("wav" => WAV(Vec<u8>));
	gen_res!("midi" => MIDI(Vec<u8>));
	gen_res!("ttf" => TTF(Vec<u8>));
	gen_res!("otf" => OTF(Vec<u8>));
	gen_res!("woff" => WOFF(Vec<u8>));
	gen_res!("woff2" => WOFF2(Vec<u8>));
	gen_res!("mp4" => MP4(Vec<u8>));
	gen_res!("zip" => ZIP(Vec<u8>));

	res.set("no", ctx.create_function(|_, (data): (String)| {
		return Ok(http::Response::new(http::Status::NotFound, http::ContentType::HTML, &data));
	})?)?;

	res.set("redirect", ctx.create_function(|_, (url): (String)| {
		return Ok(http::Response::redirect(&url));
	})?)?;

	http.set("res", res)?;
	ctx.add_module("http", http)?;

	return Ok(());

}

#[cfg(feature = "term")]
fn bind_term(ctx: &Context) -> Result<()> {

	let term = ctx.create_table()?;

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

	ctx.add_module("term", term)?;

	return Ok(());

}

#[cfg(feature = "ase")]
fn bind_ase(ctx: &Context) -> Result<()> {

	let ase = ctx.create_table()?;

	impl UserData for ase::AnimDir {}
	impl UserData for ase::Anim {}
	impl UserData for ase::SpriteData {}

	ase.set("from_file", ctx.create_function(|_, (path): (String)| {
		return Ok(ase::SpriteData::from_file(path)?);
	})?)?;

	ase.set("from_json", ctx.create_function(|_, (data): (String)| {
		return Ok(ase::SpriteData::from_json(&data)?);
	})?)?;

	ctx.add_module("ase", ase)?;

	return Ok(());

}

#[cfg(feature = "col")]
fn bind_col(ctx: &Context) -> Result<()> {

	use math::*;

	let col = ctx.create_table()?;

	col.set("rect_rect", ctx.create_function(|_, (r1, r2): (Quad, Quad)| {
		return Ok(col::rect_rect(r1, r2));
	})?)?;

	col.set("line_line", ctx.create_function(|_, (p1, p2, p3, p4): (Vec2, Vec2, Vec2, Vec2)| {
		return Ok(col::line_line((p1, p2), (p3, p4)));
	})?)?;

	col.set("line_poly", ctx.create_function(|_, (p1, p2, poly): (Vec2, Vec2, Vec<Vec2>)| {
		return Ok(col::line_poly((p1, p2), &poly));
	})?)?;

	col.set("poly_poly", ctx.create_function(|_, (p1, p2): (Vec<Vec2>, Vec<Vec2>)| {
		return Ok(col::poly_poly(&p1, &p2));
	})?)?;

	col.set("point_rect", ctx.create_function(|_, (pt, r): (Vec2, Quad)| {
		return Ok(col::point_rect(pt, r));
	})?)?;

	col.set("point_poly", ctx.create_function(|_, (pt, p): (Vec2, Vec<Vec2>)| {
		return Ok(col::point_poly(pt, &p));
	})?)?;

	col.set("sat", ctx.create_function(|_, (p1, p2): (Vec<Vec2>, Vec<Vec2>)| {
		return Ok(col::sat(&p1, &p2));
	})?)?;

	return Ok(());

}

fn bind_vec(ctx: &Context) -> Result<()> {

	use math::*;

	let globals = ctx.globals();

	impl UserData for Vec2 {

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

	impl UserData for Vec3 {}
	impl UserData for Vec4 {}
	impl UserData for Mat4 {}
	impl UserData for Color {}
	impl UserData for Quad {}

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

	globals.set("quad", ctx.create_function(|_, (x, y, w, h): (f32, f32, f32, f32)| {
		return Ok(quad!(x, y, w, h));
	})?)?;

	return Ok(());

}

fn bind_thread(ctx: &Context) -> Result<()> {

	let globals = ctx.globals();

	impl<'a, T: Send + Clone + 'static + for<'lua> ToLua<'lua>> UserData for thread::Task<T> {

		fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

			methods.add_method_mut("poll", |_, t: &mut thread::Task<T>, (): ()| {
				return Ok(t.poll());
			});

		}

	}

	globals.set("sleep", ctx.create_function(|_, (t): (u64)| {
		return Ok(std::thread::sleep(std::time::Duration::from_millis(t)));
	})?)?;

	return Ok(());

}

fn bind(ctx: &Context) -> Result<()> {

	bind_vec(&ctx)?;
	bind_thread(&ctx)?;

	#[cfg(feature = "fs")]
	bind_fs(&ctx)?;

	#[cfg(feature = "app")]
	bind_app(&ctx)?;

	#[cfg(feature = "img")]
	bind_img(&ctx)?;

	#[cfg(feature = "audio")]
	bind_audio(&ctx)?;

	#[cfg(feature = "http")]
	bind_http(&ctx)?;

	#[cfg(feature = "term")]
	bind_term(&ctx)?;

	#[cfg(feature = "ase")]
	bind_ase(&ctx)?;

	#[cfg(feature = "col")]
	bind_col(&ctx)?;

	ctx.add_module_from_lua("json", include_str!("res/json.lua"))?;

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

