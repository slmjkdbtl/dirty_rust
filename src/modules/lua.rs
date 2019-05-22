// wengwengweng

//! Lua Bindings

use std::path::Path;
use crate::*;
use crate::err::Error;

use rlua::Lua;
use rlua::Table;
use rlua::UserData;
use rlua::Result;

impl From<Error> for rlua::Error {
	fn from(err: Error) -> rlua::Error {
		return match err {
			Error::IO => rlua::Error::RuntimeError(String::from("io error")),
			Error::Net => rlua::Error::RuntimeError(String::from("network error")),
		}
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

		globals.set("arg", args);

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

		win.set("init", ctx.create_function(|_, (conf): (rlua::Value)| {
			return Ok(window::init(window::Conf::default()));
		})?)?;

		win.set("run", ctx.create_function(|_, (f): (rlua::Function)| {
			return Ok(window::run(|| {
				if f.call::<_, ()>(()).is_err() {
					panic!("failed to run");
				}
			}));
		})?)?;

		win.set("fps", ctx.create_function(|_, (): ()| {
			return Ok(window::fps());
		})?)?;

		win.set("dt", ctx.create_function(|_, (): ()| {
			return Ok(window::dt());
		})?)?;

		http.set("get", ctx.create_function(|_, (uri): (String)| {
			return Ok(http::get(&uri)?);
		})?)?;

		http.set("get_bytes", ctx.create_function(|_, (uri): (String)| {
			return Ok(http::get_bytes(&uri)?);
		})?)?;

		globals.set("fs", fs)?;
		globals.set("win", win)?;
		globals.set("http", http)?;

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

