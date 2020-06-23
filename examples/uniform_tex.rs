// wengwengweng

use dirty::*;
use gfx::*;
use input::*;

struct TexUniform {
	tex: Texture,
}

impl UniformLayout for TexUniform {
	fn textures(&self) -> Vec<&Texture> {
		return vec![&self.tex];
	}
}

struct Game {
	tex: Texture,
	shader: Shader<TexUniform>,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			tex: Texture::from_bytes(d.gfx, include_bytes!("res/acid2.png"))?,
			shader: Shader::from_frag(d.gfx, include_str!("res/uniform_tex.frag"))?,
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

		d.gfx.draw_with(&self.shader, &TexUniform {
			tex: self.tex.clone(),
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

