// wengwengweng

#[macro_use]

extern crate image;
extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use gl::types::*;
use std::ffi::{CString};
use std::io::Cursor;
use std::os::raw::c_void;
use std::thread;
use std::time;
use std::ptr;
use std::str;

mod app;
mod gfx;
mod audio;
mod math;

fn main() {

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let gl_attr = video_subsystem.gl_attr();

	gl_attr.set_context_profile(GLProfile::Core);
	gl_attr.set_context_version(3, 3);
	gl_attr.set_context_flags()
		.forward_compatible();

	let window = video_subsystem.window("yo", 640, 480)
		.opengl()
		.build()
		.unwrap();

	let ctx = window.gl_create_context().unwrap();

	gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

	let image = image::load(Cursor::new(&include_bytes!("car.png")[..]), image::PNG)
		.unwrap()
		.to_rgba();

	let mut vao: GLuint = 0;
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

	unsafe {

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		gl::ClearColor(0.0, 0.0, 0.0, 1.0);

		gl::GenBuffers(1, &mut vert_buf);
		gl::GenBuffers(1, &mut uv_buf);
		gl::GenBuffers(1, &mut index_buf);
		gl::GenVertexArrays(1, &mut vao);

		gl::BindVertexArray(vao);

		gl::BindBuffer(gl::ARRAY_BUFFER, vert_buf);

		gl::BufferData(
			gl::ARRAY_BUFFER,
			(vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
			&vertices[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW
		);

		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buf);

		gl::BufferData(
			gl::ELEMENT_ARRAY_BUFFER,
			(indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
			&indices[0] as *const u32 as *const c_void,
			gl::STATIC_DRAW
		);

		gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
		gl::EnableVertexAttribArray(0);

		gl::BindBuffer(gl::ARRAY_BUFFER, uv_buf);

		gl::BufferData(
			gl::ARRAY_BUFFER,
			(uv.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
			&uv[0] as *const f32 as *const c_void,
			gl::STATIC_DRAW
		);

		gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<GLfloat>() as GLsizei, ptr::null());
		gl::EnableVertexAttribArray(1);

		gl::BindVertexArray(0);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

	}

	let program = create_program(
		include_str!("quad.vert").to_owned(),
		include_str!("quad.frag").to_owned()
	);

	let mut event_pump = sdl_context.event_pump().unwrap();

	'running: loop {

		unsafe {

			gl::Clear(gl::COLOR_BUFFER_BIT);
			gl::UseProgram(program);
			gl::BindVertexArray(vao);
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

			let mut log_length = 0;
			gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length);
			let log: Vec<i8> = Vec::with_capacity(log_length as usize);
			gl::GetShaderInfoLog(id, 512, ptr::null_mut(), log.as_ptr() as *mut i8);
			eprintln!("{:?}", log);
			gl::DeleteShader(id);

		}

		return id;

	}

}

// fn main() {

// 	let mut events_loop = glutin::EventsLoop::new();

// 	let window = glutin::WindowBuilder::new()
// 		.with_dimensions((640, 480).into())
// 		.with_title("yo");

// 	let context = glutin::ContextBuilder::new();
// 	let display = glium::Display::new(window, context, &events_loop).unwrap();

// 	let image = image::load(Cursor::new(&include_bytes!("car.png")[..]), image::PNG)
// 		.unwrap()
// 		.to_rgba();

// 	let image_dimensions = image.dimensions();
// 	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
// 	let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

// 	#[derive(Copy, Clone)]
// 	struct Vertex {
// 		pos: [f32; 2],
// 		uv: [f32; 2],
// 	}

// 	implement_vertex!(Vertex, pos, uv);

// 	let vertex_buffer = glium::VertexBuffer::new(&display,
// 		&[
// 			Vertex { pos: [-1.0, -1.0], uv: [0.0, 0.0] },
// 			Vertex { pos: [-1.0,  1.0], uv: [0.0, 1.0] },
// 			Vertex { pos: [ 1.0,  1.0], uv: [1.0, 1.0] },
// 			Vertex { pos: [ 1.0, -1.0], uv: [1.0, 0.0] }
// 		]
// 	).unwrap();

// 	let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();

// 	let vs_src = r#"
// 		#version 100
// 		precision mediump float;
// 		attribute vec2 pos;
// 		attribute vec2 uv;
// 		varying vec2 tex_coord;
// 		uniform mat4 projection;
// 		uniform mat4 transform;
// 		uniform vec4 quad;
// 		void main() {
// 			tex_coord = quad.xy + uv * quad.zw;
// 			gl_Position = projection * transform * vec4(pos, 0.0, 1.0);
// 		}
// 	"#;

// 	let fs_src = r#"
// 		#version 100
// 		precision mediump float;
// 		varying vec2 tex_coord;
// 		uniform sampler2D tex;
// 		uniform vec4 tint;
// 		void main() {
// 			gl_FragColor = texture2D(tex, tex_coord) * tint;
// 		}
// 	"#;

// 	let program = glium::Program::from_source(&display, vs_src, fs_src, None).unwrap();

// 	let mut closed = false;

// 	while !closed {

// 		let mut target = display.draw();

// 		let trans = math::mat4()
// 			.translate(120.0, 120.0)
// 			.scale(100.0, 100.0);

// 		let proj = math::ortho(0.0, 640.0, 480.0, 0.0, -1.0, 1.0);

// 		let uniforms = uniform!{
// 			projection: proj.matrix(),
// 			transform: trans.matrix(),
// 			quad: math::vec4(0.0, 0.0, 0.25, 1.0).arr(),
// 			tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
// 			tint: math::vec4(1.0, 1.0, 1.0, 1.0).arr(),
// 		};

// 		target.clear_color(0.0, 0.0, 0.0, 1.0);
// 		target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
// 		target.finish().unwrap();

// 		events_loop.poll_events(|ev| {
// 			match ev {
// 				glutin::Event::WindowEvent { event, .. } => match event {
// 					glutin::WindowEvent::CloseRequested =>
// 						closed = true,
// 					_ =>
// 						(),
// 				},
// 				_ => (),
// 			}
// 		});

// 		thread::sleep(time::Duration::from_millis(16));

// 	}

// }

