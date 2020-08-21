// wengwengweng

use dirty::*;
use gfx::*;
use input::*;

struct TexUniform {
	tex: Texture,
	time: f32,
}

impl UniformLayout for TexUniform {
	fn values(&self) -> Vec<(&'static str, UniformValue)> {
		return vec![
			("u_time", UniformValue::Float(self.time)),
			("u_dtex", UniformValue::Texture(self.tex.clone())),
		];
	}
}

struct Game {
	tex: Texture,
	displace_tex: Texture,
	shader: Shader<TexUniform>,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			tex: Texture::from_bytes(d.gfx, include_bytes!("res/acid2.png"))?,
			displace_tex: Texture::from_bytes_with_conf(d.gfx, include_bytes!("res/displace.png"), TextureConf {
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

	fn draw(&self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw_with(&self.shader, &TexUniform {
			tex: self.displace_tex.clone(),
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

