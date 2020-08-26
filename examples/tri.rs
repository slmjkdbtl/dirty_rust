// wengwengweng

use dirty::*;
use gfx::*;
use input::*;

const VERTS: &[Vertex] = &[
	Vertex {
		pos: vec3!(0, 96, 0),
		normal: vec3!(0, 0, 1),
		color: rgba!(1, 0, 0, 1),
		uv: vec2!(0),
	},
	Vertex {
		pos: vec3!(-120, -96, 0),
		normal: vec3!(0, 0, 1),
		color: rgba!(0, 1, 0, 1),
		uv: vec2!(0),
	},
	Vertex {
		pos: vec3!(120, -96, 0),
		normal: vec3!(0, 0, 1),
		color: rgba!(0, 0, 1, 1),
		uv: vec2!(0),
	},
];

const INDICES: &[u32] = &[0, 1, 2];

struct Tri;

impl State for Tri {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		match e {
			Event::KeyPress(k) => {
				match k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {
		d.gfx.draw(&shapes::raw(&VERTS, &INDICES))?;
		return Ok(());
	}

}

fn main() {
	if let Err(e) = launcher()
		.run::<Tri>() {
		elog!("{}", e);
	}
}

