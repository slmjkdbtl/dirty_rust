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
use std::thread;
use std::time;
use std::ptr;
use std::mem;
use rodio::Source;

mod app;
mod gfx;
mod audio;
mod math;

fn main() {

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let gl_attr = video_subsystem.gl_attr();

	gl_attr.set_context_profile(GLProfile::Compatibility);
	gl_attr.set_context_version(2, 1);

	let window = video_subsystem.window("yo", 640, 480)
		.opengl()
		.build()
		.unwrap();

	let ctx = window.gl_create_context().unwrap();

	gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

// 	let device = rodio::default_output_device().unwrap();

// 	let source = rodio::Decoder::new(Cursor::new(&include_bytes!("pop.ogg")[..])).unwrap();
// 	rodio::play_raw(&device, source.convert_samples());

	let vertices: Vec<GLfloat> = vec![
		-0.5,  0.5,
		 0.5,  0.5,
		 0.5, -0.5,
		-0.5, -0.5,
	];

	let uv: Vec<GLfloat> = vec![
		0.0, 1.0,
		1.0, 1.0,
		1.0, 0.0,
		0.0, 0.0
	];

	let indices: Vec<GLuint> = vec![
		0, 1, 3,
		1, 2, 3,
	];

	let mut mesh = gfx::make_mesh();

	mesh
		.make_buf(&vertices)
		.make_buf(&uv)
		.make_index_buf(&indices)
		.attr(0, 0)
		.attr(1, 1);

	let program = gfx::make_program(
		include_str!("quad.vert").to_owned(),
		include_str!("quad.frag").to_owned()
	);

	program
		.attr(0, "pos")
		.attr(1, "uv")
		.link();

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

	gfx::init();

	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut index = 0;

	'running: loop {

		let trans = math::mat4()
			.translate(240.0, 240.0)
			.rotate(0.0)
			.scale((tex.width as f32) * 0.25 * 2.0, (tex.height as f32) * 2.0);

		if (index < 3) {
			index += 1;
		} else {
			index = 0;
		}

		let proj = math::ortho(0.0, 640.0, 480.0, 0.0, -1.0, 1.0);
		let quad = math::vec4((index as f32) * 0.25, 0.0, 0.25, 1.0);
		let tint = math::vec4(1.0, 1.0, 1.0, 1.0);

		gfx::clear();

		program
			.uniform_vec4("tint", tint.as_arr())
			.uniform_vec4("quad", quad.as_arr())
			.uniform_mat4("proj", proj.as_arr())
			.uniform_mat4("trans", trans.as_arr())
			.bind();

		tex.bind();
		mesh.draw();

		window.gl_swap_window();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}

		thread::sleep(time::Duration::from_millis(16));

	}

}

