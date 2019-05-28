// wengwengweng

use ketos::Interpreter;
use ketos::ModuleLoader;
use ketos::Name;
use ketos::BuiltinModuleLoader;
use ketos::ModuleBuilder;
use ketos::Context;
use ketos::Module;
use ketos::Scope;
use ketos::CompileError;
use ketos::ketos_fn;

use crate::Result;
use crate::math::*;

type KetosResult<T> = ::std::result::Result<T, ketos::Error>;

// fn vec2_new(x: f32, y: f32) -> Vec2 {
// 	return Vec2::new(x, y);
// }

fn bind(interp: &Interpreter) {

	let scope = interp.scope();

// 	ketos_fn! { scope => "vec2" =>
// 		fn vec2_new(x: f32, y: f32) -> Vec2 };

}

pub fn run(code: &str) -> Result<()> {

	let interp = Interpreter::new();

	bind(&interp);
	interp.run_code(code, None)?;

	return Ok(());

}
