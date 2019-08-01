// wengwengweng

#[cfg(all(feature = "lua", feature = "python"))]
compile_error!("can only enable one scripting option");

#[cfg(feature = "lua")]
pub mod lua;
#[cfg(feature = "python")]
pub mod python;

