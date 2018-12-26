// wengwengweng

#![windows_subsystem = "windows"]
#[macro_use]

extern crate image;
extern crate gl;
extern crate sdl2;
extern crate rodio;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use gl::types::*;
use std::ffi::CString;
use std::io::Cursor;
use std::os::raw::c_void;
use std::thread;
use std::time;
use std::ptr;
use std::ptr::null;
use std::ptr::null_mut;
use std::str;
use std::fs::File;
use std::io::BufReader;
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

	let mut vert_buf: GLuint = 0;
	let mut uv_buf: GLuint = 0;
	let mut index_buf: GLuint = 0;

	let vertices: [f32; 8] = [
		-0.5,  0.5,
		 0.5,  0.5,
		 0.5, -0.5,
		-0.5, -0.5,
	];

	let uv: [f32; 8] = [
		0.0, 1.0,
		1.0, 1.0,
		1.0, 0.0,
		0.0, 0.0
	];

	let indices: [u32; 6] = [
		0, 1, 3,
		1, 2, 3,
	];

	let image = image::load(Cursor::new(&include_bytes!("car.png")[..]), image::PNG)
		.unwrap()
		.to_rgba();
	let width: GLint = image.width() as GLint;
	let height: GLint = image.height() as GLint;
	let pixels: Vec<u8> = image.into_raw();

	let mut texture_id: GLuint = 0;
	let mut vert_attr: GLint;

	let program = create_program(
		include_str!("quad.vert").to_owned(),
		include_str!("quad.frag").to_owned()
	);

	unsafe {

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		gl::ClearColor(0.0, 0.0, 1.0, 1.0);

		gl::GenBuffers(1, &mut vert_buf);
		gl::GenBuffers(1, &mut uv_buf);
		gl::GenBuffers(1, &mut index_buf);

		gl::BindBuffer(gl::ARRAY_BUFFER, vert_buf);

		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
			&vertices[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW
		);

		gl::BindBuffer(gl::ARRAY_BUFFER, uv_buf);

		gl::BufferData(
			gl::ARRAY_BUFFER,
			(uv.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
			&uv[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW
		);

		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buf);

		gl::BufferData(
			gl::ELEMENT_ARRAY_BUFFER,
			(indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
			&indices[0] as *const u32 as *const c_void,
			gl::STATIC_DRAW
		);

		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

		gl::BindBuffer(gl::ARRAY_BUFFER, vert_buf);
		gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
		gl::EnableVertexAttribArray(0);
		gl::BindBuffer(gl::ARRAY_BUFFER, uv_buf);
		gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
		gl::EnableVertexAttribArray(1);

		gl::GenTextures(1, &mut texture_id);
		gl::BindTexture(gl::TEXTURE_2D, texture_id);

		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGBA8 as GLint,
			width,
			height,
			0,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			pixels.as_ptr() as *const _
		);

		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
		gl::GenerateMipmap(gl::TEXTURE_2D);
		gl::BindTexture(gl::TEXTURE_2D, 0);

	}

	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut index = 0;

	'running: loop {

		let trans = math::mat4()
			.translate(240.0, 240.0)
			.scale((width as f32) * 0.25 * 2.0, (height as f32) * 2.0);

		if (index < 3) {
			index += 1;
		} else {
			index = 0;
		}

		let proj = math::ortho(0.0, 640.0, 480.0, 0.0, -1.0, 1.0);
		let quad = math::vec4((index as f32) * 0.25, 0.0, 0.25, 1.0);
		let tint = math::vec4(1.0, 1.0, 1.0, 1.0);

		unsafe {

			gl::Clear(gl::COLOR_BUFFER_BIT);
			gl::Viewport(0, 0, 640, 480);
			uniform_vec4(program, "tint", tint.as_array());
			uniform_vec4(program, "quad", quad.as_array());
			uniform_mat4(program, "proj", proj.as_array());
			uniform_mat4(program, "trans", trans.as_array());
			gl::UseProgram(program);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buf);
			gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

		}

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

fn create_program(vs_src: String, fs_src: String) -> GLuint {

	unsafe {

		let vs: GLuint = compile_shader(gl::VERTEX_SHADER, vs_src);
		let fs: GLuint = compile_shader(gl::FRAGMENT_SHADER, fs_src);
		let program: GLuint = gl::CreateProgram();

		gl::AttachShader(program, vs);
		gl::AttachShader(program, fs);
		gl::BindAttribLocation(program, 0, CString::new("pos").unwrap().as_ptr());
		gl::BindAttribLocation(program, 1, CString::new("uv").unwrap().as_ptr());
		gl::LinkProgram(program);

		return program;

	}

}

fn compile_shader(shader_type: GLenum, src: String) -> GLuint {

	unsafe {

		let id: GLuint = gl::CreateShader(shader_type);
		let src_cstr = CString::new(src).unwrap();

		gl::ShaderSource(id, 1, &src_cstr.as_ptr(), ptr::null());
		gl::CompileShader(id);

		let mut status: GLint = gl::FALSE as GLint;

		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);

		if status != (gl::TRUE as GLint) {

			let mut log_length: GLint = std::mem::uninitialized();

			gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length);

			let mut log: Vec<u8> = Vec::with_capacity(log_length as usize);

			gl::GetShaderInfoLog(
				id,
				log_length,
				&mut log_length,
				log.as_mut_ptr() as *mut GLchar
			);

			log.set_len(log_length as usize);
			panic!("{}", String::from_utf8(log).unwrap());

		}

		return id;

	}

}

fn uniform_vec4(id: GLuint, name: &str, value: [f32; 4]) {

	unsafe {
		gl::Uniform4f(
			gl::GetUniformLocation(id, CString::new(name).unwrap().as_ptr()),
			value[0],
			value[1],
			value[2],
			value[3],
		);
	}

}

fn uniform_mat4(id: GLuint, name: &str, value: [[f32; 4]; 4]) {

	unsafe {
		gl::UniformMatrix4fv(
			gl::GetUniformLocation(id, CString::new(name).unwrap().as_ptr()),
			1,
			gl::FALSE,
			&value[0][0]
		);
	}

}

fn cstr_ptr(data: &str) -> *const GLchar {
	return CString::new(data).unwrap().as_ptr();
}


