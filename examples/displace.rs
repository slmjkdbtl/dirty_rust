// wengwengweng

use dirty::*;
use gfx::*;
use input::*;

struct DisplaceUniform {
	tex: Texture,
	time: f32,
}

impl UniformLayout for DisplaceUniform {
	fn data(&self) -> Vec<(&'static str, UniformData)> {
		return vec![
			("u_time", UniformData::Float(self.time)),
			("u_effect_tex", UniformData::Texture(self.tex.clone())),
		];
	}
}

struct Game {
	tex: Texture,
	effect_tex: Texture,
	shader: Shader<DisplaceUniform>,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			tex: Texture::from_bytes(d.gfx, include_bytes!("res/acid2.png"))?,
			effect_tex: Texture::from_bytes_with_conf(d.gfx, include_bytes!("res/displace.png"), TextureConf {
				filter: FilterMode::Linear,
				wrap: WrapMode::Repeat,
				..TextureConf::default()
			})?,
			shader: Shader::from_frag(d.gfx, include_str!("res/displace.frag"))?,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		match e {
			Event::KeyPress(k) => {
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

		d.gfx.draw_with(&self.shader, &DisplaceUniform {
			tex: self.effect_tex.clone(),
			time: d.app.time().as_secs_f32(),
		}, |gfx| {
			gfx.draw(&shapes::sprite(&self.tex))?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = run::<Game>() {
		elog!("{}", e);
	}
}

