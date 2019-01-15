// wengwengweng

//! Ketos Bindings

use ketos::Interpreter;

use crate::fs;

/// run from ketos code
pub fn run_code(code: &str) {

	// Create an interpreter.
	let interp = Interpreter::new();

	interp.run_code(code, None).expect("failed to run code");

}

/// run from ketos file
pub fn run(fname: &str) {
	run_code(&fs::read_str(fname));
}

