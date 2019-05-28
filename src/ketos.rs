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

type KetosResult<T> = ::std::result::Result<T, ketos::Error>;

fn bind(interp: &Interpreter) {
	// ...
}

pub fn run(code: &str) -> Result<()> {

	let interp = Interpreter::new();

	bind(&interp);
	interp.run_code(code, None)?;

	return Ok(());

}
