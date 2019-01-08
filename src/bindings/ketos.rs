// wengwengweng

//! Ketos Bindings

use ketos::Interpreter;

/// run from ketos code
pub fn run_code(code: &str) {

	// Create an interpreter.
	let interp = Interpreter::new();

	interp.run_code(code, None).unwrap();

}

