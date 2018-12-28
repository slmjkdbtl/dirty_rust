// wengwengweng

#![windows_subsystem = "windows"]
#![allow(unused_parens)]

extern crate image;
extern crate gl;
extern crate sdl2;
extern crate rodio;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use gl::types::*;
use std::io::Cursor;
use rodio::Source;
use std::thread;
use std::time;

mod app;
mod gfx;
mod audio;
mod math;

fn test(a: u8) {}

fn main() {

// 	let device = rodio::default_output_device().unwrap();

// 	let source = rodio::Decoder::new(Cursor::new(&include_bytes!("pop.ogg")[..])).unwrap();
// 	rodio::play_raw(&device, source.convert_samples());

	app::init("yo", 960, 640);
	gfx::init();

	let img = image::load(Cursor::new(&include_bytes!("car.png")[..]), image::PNG)
		.unwrap()
		.to_rgba();

	let width = img.width();
	let height = img.height();
	let pixels = img.into_raw();

	let tex = gfx::make_texture(
		&pixels,
		width,
		height,
	);

	let mut index = 0;

	app::run(|| {
		gfx::clear();
		gfx::draw(&tex, math::vec2(240.0, 240.0), 0.0, math::vec2(2.0, 2.0), math::vec4((index as f32) * 0.25, 0.0, 0.25, 1.0));
	})

}

