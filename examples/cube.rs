// wengwengweng

use dirty::*;
use gfx::*;
use input::Key;

struct Game {
	cube: Mesh,
}

const VERTICES: [Vertex; 8] = [
	Vertex {
		pos: vec3!(1, 1, 1),
		normal: vec3!(0, 0, 0),
		uv: vec2!(0, 0),
		color: rgba!(1, 1, 1, 1),
	},
	Vertex {
		pos: vec3!(-1, 1, 1),
		normal: vec3!(0, 0, 0),
		uv: vec2!(0, 0),
		color: rgba!(0, 1, 1, 1),
	},
	Vertex {
		pos: vec3!(-1, -1, 1),
		normal: vec3!(0, 0, 0),
		uv: vec2!(0, 0),
		color: rgba!(0, 0, 1, 1),
	},
	Vertex {
		pos: vec3!(1, -1, 1),
		normal: vec3!(0, 0, 0),
		uv: vec2!(0, 0),
		color: rgba!(1, 0, 1, 1),
	},
	Vertex {
		pos: vec3!(1, 1, -1),
		normal: vec3!(0, 0, 0),
		uv: vec2!(0, 0),
		color: rgba!(1, 1, 0, 1),
	},
	Vertex {
		pos: vec3!(-1, 1, -1),
		normal: vec3!(0, 0, 0),
		uv: vec2!(0, 0),
		color: rgba!(0, 1, 0, 1),
	},
	Vertex {
		pos: vec3!(-1, -1, -1),
		normal: vec3!(0, 0, 0),
		uv: vec2!(0, 0),
		color: rgba!(0, 0, 0, 1),
	},
	Vertex {
		pos: vec3!(1, -1, -1),
		normal: vec3!(0, 0, 0),
		uv: vec2!(0, 0),
		color: rgba!(1, 0, 0, 1),
	},
];

const INDICES: [u32; 36] = [
	0, 1, 2,
	0, 2, 3,
	4, 0, 3,
	4, 3, 7,
	4, 5, 1,
	4, 1, 0,
	5, 4, 7,
	5, 7, 6,
	1, 5, 6,
	1, 6, 2,
	3, 2, 6,
	3, 6, 7,
];

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			cube: Mesh::new(d.gfx, &VERTICES, &INDICES)?,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw_t(
			mat4!()
				.rx(f32::to_radians(-45.0))
				.ry(f32::to_radians(-45.0))
				.s3(vec3!(90))
				,
			&shapes::mesh(&self.cube)
		)?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = run::<Game>() {
		elog!("{}", e);
	}
}

