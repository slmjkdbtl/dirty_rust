// wengwengweng

use dirty::*;
use app::*;
use input::Key;

use math::noise::*;

struct Game {
	tex: gfx::Texture,
}

fn gen(w: i32, h: i32, scale: f64, noise: &impl NoiseFn<[f64; 2]>) -> Vec<u8> {

	let mut buf = Vec::with_capacity((w * h * 4) as usize);

	for j in 0..h {

		for i in 0..w {

			let depth = noise.get([
				i as f64 / w as f64 / scale,
				j as f64 / h as f64 / scale,
			]);

			let c = ((depth + 1.0) / 2.0 * 255.0) as u8;

			buf.extend_from_slice(&[c, c, c, 255]);

		}

	}

	return buf;

}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let seed = math::rand(0, 65536);
		let noise = Fbm::new()
			.set_seed(seed);
		let w = ctx.width();
		let h = ctx.height();

		let img = img::Image::from_pixels(w, h, gen(w, h, 0.3, &noise))?;

		return Ok(Self {
			tex: gfx::Texture::from_img(ctx, img)?,
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				match *k {

					Key::Esc => ctx.quit(),

					Key::Space => {

						let noise = Fbm::new()
							.set_seed(math::rand(0, 65536));

						let w = ctx.width();
						let h = ctx.height();
						let img = img::Image::from_pixels(w, h, gen(w, h, 0.3, &noise))?;

						self.tex = gfx::Texture::from_img(ctx, img)?;

					},

					_ => {},

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&shapes::sprite(&self.tex))?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
// 		.scale(2.0)
		.run::<Game>();
}

