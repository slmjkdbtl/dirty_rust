// wengwengweng

use std::time::Duration;

use dirty::*;
use gfx::shapes;
use input::Key;

#[derive(Clone)]
pub struct RainbowUniform {
	pub time: Duration,
	pub size: f32,
}

impl gfx::CustomUniform for RainbowUniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_time" => &self.time,
			"u_size" => &self.size,
		];
	}
}

struct Game {
	rainbow_shader: gfx::Shader<RainbowUniform>,
	model: gfx::Model,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			rainbow_shader: gfx::Shader::from_frag(d.gfx, include_str!("res/rainbow.frag"))?,
			model: gfx::Model::from_glb(d.gfx, include_bytes!("res/duck.glb"))?,
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

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		let center = self.model.center();
		let time = d.app.time();

		d.gfx.draw_with(&self.rainbow_shader, &RainbowUniform {
			size: 64.0,
			time: d.app.time(),
		}, |gfx| {

			gfx.draw_t(
				mat4!()
					.s3(vec3!(160))
					.t3(-center)
					.ry(time.as_secs_f32())
					,
				&shapes::model(&self.model)
					,
			)?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.run::<Game>() {
		elog!("{}", e);
	}
}

