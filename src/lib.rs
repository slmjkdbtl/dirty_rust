// wengwengweng

//! a cross-platform toolkit for making games / interactive experiences
//!
//! ## Basic Setup
//!
//! to create a window, make a struct that implements the [`State`](trait.State.html) trait, then use [`run`](fn.run.html) to run
//! ```no_run
//! use dirty::*;
//!
//! struct Game;
//!
//! impl State for Game {
//!     fn init(_: &mut Ctx) -> Result<Self> {
//!         return Ok(Self);
//!     }
//! }
//!
//! fn main() {
//!     run::<Game>();
//! }
//! ```
//! for more information checkout the doc of [`State`](trait.State.html), and [`launcher`](fn.launcher.html) if you wish to have more start up options
//!
//! ## Context
//! each method under [`State`](trait.State.html) takes a [`Ctx`](struct.Ctx.html) as parameter, which represents the application context, it has 4 fields (modules):
//!  - [`ctx.window`](window/struct.Window.html)
//!    provides ways to interact with the window, like toggling fullscreen
//!  - [`ctx.gfx`](gfx/struct.Gfx.html)
//!    provides everything you need with rendering stuff on screen
//!  - [`ctx.app`](app/struct.App.html)
//!    provides methods that relates to the application lifecycle, like time
//!  - [`ctx.audio`](audio/struct.Audio.html)
//!    provides everything you need to play audio
//!
//! for more information checkout each indivisual docs

#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![deny(clippy::implicit_return)]

#[macro_use]
pub mod utils;
pub mod res;

pub mod conf;
pub mod app;
pub mod window;
pub mod gfx;
pub mod audio;
pub mod input;

mod run;
pub use run::*;
mod state;
pub use state::*;
mod ctx;
pub use ctx::*;

pub mod fs;
pub mod geom;
pub mod img;
pub mod math;
pub mod physics;
pub mod term;
pub mod ui;
pub mod kit;
pub mod data;
pub mod task;
pub mod ase;

#[cfg(feature = "midi")]
pub mod midi;

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;

