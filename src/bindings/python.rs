// wengwengweng

use std::rc::Rc;
use std::path::Path;

use rustpython_vm::*;
use rustpython_vm::pyobject::*;
use rustpython_vm::function::*;

use crate::Error;
use crate::Result;

// fn vec2(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {

// 	arg_check!(
// 		vm,
// 		args,
// 		required = [(x, Some(vm.ctx.float_type())), (y, Some(vm.ctx.float_type()))]
// 	);

// 	let ctx = &vm.ctx;
// 	let d = ctx.new_dict();

// 	d.set_item("x", ctx.new_float(0.0), vm)?;
// 	d.set_item("y", ctx.new_float(0.0), vm)?;

// 	return d.into_pyobject(vm);

// }

// fn make_modules(ctx: &PyContext) -> PyObjectRef {
// 	return py_module!(ctx, "app", {
// 		"set_anim_duration" => ctx.new_rustfunc(set_view),
//     });
// }

pub fn run(code: &str, path: Option<impl AsRef<Path>>, args: Option<&[String]>) -> Result<()> {

	let vm = VirtualMachine::new();

	return Ok(());

}

