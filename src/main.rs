// wengwengweng

#[macro_use]

extern crate glium;
extern crate image;

use std::io::Cursor;
use glium::glutin;
use glium::Surface;
use glium::index::PrimitiveType;

mod math;

fn main() {

	let mut events_loop = glutin::EventsLoop::new();

	let window = glutin::WindowBuilder::new()
		.with_dimensions((640, 480).into())
		.with_title("yo");

	let context = glutin::ContextBuilder::new();
	let display = glium::Display::new(window, context, &events_loop).unwrap();

	let image = image::load(Cursor::new(&include_bytes!("car.png")[..]), image::PNG)
		.unwrap()
		.to_rgba();

	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = glium::texture::Texture2d::new(&display, image).unwrap();

	#[derive(Copy, Clone)]
	struct Vertex {
		pos: [f32; 2],
		uv: [f32; 2],
	}

	implement_vertex!(Vertex, pos, uv);

	let vertex_buffer = glium::VertexBuffer::new(&display,
		&[
			Vertex { pos: [-1.0, -1.0], uv: [0.0, 0.0] },
			Vertex { pos: [-1.0,  1.0], uv: [0.0, 1.0] },
			Vertex { pos: [ 1.0,  1.0], uv: [1.0, 1.0] },
			Vertex { pos: [ 1.0, -1.0], uv: [1.0, 0.0] }
		]
	).unwrap();

	let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();

	let vs_src = r#"
		#version 100
		precision mediump float;
		attribute vec2 pos;
		attribute vec2 uv;
		varying vec2 tex_coord;
		uniform mat4 projection;
		uniform mat4 transform;
		uniform vec4 quad;
		void main() {
			tex_coord = quad.xy + uv * quad.zw;
			gl_Position = transform * projection * vec4(pos, 0.0, 1.0);
		}
	"#;

	let fs_src = r#"
		#version 100
		precision mediump float;
		varying vec2 tex_coord;
		uniform sampler2D tex;
		uniform vec4 tint;
		void main() {
			gl_FragColor = texture2D(tex, tex_coord) * tint;
		}
	"#;

	let program = glium::Program::from_source(&display, vs_src, fs_src, None).unwrap();

	let mut closed = false;

	while !closed {

		let mut target = display.draw();

		let trans = math::mat4()
			.translate(0.0, 0.0)
			.scale(1.0, 1.0);

// 		let proj = math::ortho(0.0, 640.0, 480.0, 0.0, -1.0, 1.0);
		let proj = math::mat4();

		let uniforms = uniform!{
			projection: proj.matrix(),
			transform: trans.matrix(),
			quad: math::vec4(0.0, 0.0, 0.25, 1.0).arr(),
			tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
			tint: math::vec4(1.0, 1.0, 1.0, 1.0).arr(),
		};

		target.clear_color(0.0, 0.0, 0.0, 1.0);
		target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
		target.finish().unwrap();

		events_loop.poll_events(|ev| {
			match ev {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::CloseRequested =>
						closed = true,
					_ =>
						(),
				},
				_ => (),
			}
		});
	}

}

