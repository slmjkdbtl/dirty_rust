// wengwengweng

#[macro_use]
mod utils;
mod addons;
mod bindings;
mod modules;

pub use crate::modules::*;
pub use crate::addons::*;
pub use crate::bindings::*;
pub use crate::math::*;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

fn main() {
	lua::run("main.lua");
}

