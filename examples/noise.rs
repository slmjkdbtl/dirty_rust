// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

use math::noise::*;

struct Game {
	tex: gfx::Texture,
	noise_type: NoiseType,
	seed: u8,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum NoiseType {
	Perlin,
	OpenSimplex,
	SuperSimplex,
	Fbm,
	Billow,
	Worley,
	RidgedMulti,
	Turbulence,
}

impl NoiseType {
	fn as_str(&self) -> &'static str {
		return match self {
			NoiseType::Perlin => "perlin",
			NoiseType::OpenSimplex => "open simplex",
			NoiseType::SuperSimplex => "super simplex",
			NoiseType::Fbm => "fbm",
			NoiseType::Billow => "billow",
			NoiseType::Worley => "worley",
			NoiseType::RidgedMulti => "ridged multi",
			NoiseType::Turbulence => "turbulence (fbm)",
		};
	}
}

impl Game {

	fn gen(&mut self, d: &Ctx) -> Result<()> {

		let seed = self.seed as u32;

		let noise: Box<dyn NoiseFn<[f64; 2]>> = match self.noise_type {
			NoiseType::Perlin => Box::new(Perlin::new().set_seed(seed)),
			NoiseType::OpenSimplex => Box::new(OpenSimplex::new().set_seed(seed)),
			NoiseType::SuperSimplex => Box::new(SuperSimplex::new().set_seed(seed)),
			NoiseType::Fbm => Box::new(Fbm::new().set_seed(seed)),
			NoiseType::Billow => Box::new(Billow::new().set_seed(seed)),
			NoiseType::Worley => Box::new(Worley::new().enable_range(true).set_seed(seed)),
			NoiseType::RidgedMulti => Box::new(RidgedMulti::new().set_seed(seed)),
			NoiseType::Turbulence => Box::new(Turbulence::new(Fbm::new()).set_seed(seed)),
		};

		let w = d.gfx.width() as i32;
		let h = d.gfx.height() as i32;
		let scale = 0.3;

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

		let img = img::Image::from_raw(w, h, buf)?;

		self.tex = gfx::Texture::from_img(d.gfx, img)?;

		return Ok(());

	}

}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let mut g = Self {
			tex: gfx::Texture::new(d.gfx, d.gfx.width(), d.gfx.height())?,
			noise_type: NoiseType::Perlin,
			seed: 0,
		};

		g.gen(d)?;

		return Ok(g);

	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				let mods = d.window.key_mods();

				match *k {
					Key::Esc => d.window.quit(),
					Key::Q if mods.meta => d.window.quit(),
					Key::F if mods.meta => d.window.toggle_fullscreen(),
					_ => {},
				}

			},

			KeyPressRepeat(k) => {

				match *k {

					Key::Up => {
						self.noise_type = match self.noise_type {
							NoiseType::OpenSimplex => NoiseType::Perlin,
							NoiseType::SuperSimplex => NoiseType::OpenSimplex,
							NoiseType::Fbm => NoiseType::SuperSimplex,
							NoiseType::Billow => NoiseType::Fbm,
							NoiseType::Worley => NoiseType::Billow,
							NoiseType::RidgedMulti => NoiseType::Worley,
							NoiseType::Turbulence => NoiseType::RidgedMulti,
							NoiseType::Perlin => NoiseType::Turbulence,
						};
						self.gen(d)?;
					},

					Key::Down => {
						self.noise_type = match self.noise_type {
							NoiseType::Perlin => NoiseType::OpenSimplex,
							NoiseType::OpenSimplex => NoiseType::SuperSimplex,
							NoiseType::SuperSimplex => NoiseType::Fbm,
							NoiseType::Fbm => NoiseType::Billow,
							NoiseType::Billow => NoiseType::Worley,
							NoiseType::Worley => NoiseType::RidgedMulti,
							NoiseType::RidgedMulti => NoiseType::Turbulence,
							NoiseType::Turbulence => NoiseType::Perlin,
						};
						self.gen(d)?;
					},

					Key::Left => {
						self.seed = self.seed.wrapping_sub(1);
						self.gen(d)?;
					},

					Key::Right => {
						self.seed = self.seed.wrapping_add(1);
						self.gen(d)?;
					},

					_ => {},

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		let top_left = d.gfx.coord(gfx::Origin::TopLeft);

		d.gfx.draw(&shapes::sprite(&self.tex))?;

		d.gfx.draw_t(
			mat4!()
				.t2(top_left + vec2!(24, -24))
				,
			&shapes::text(&format!("type: {}", self.noise_type.as_str()))
				.align(gfx::Origin::TopLeft)
				.size(12.0)
				,
		)?;

		d.gfx.draw_t(
			mat4!()
				.t2(top_left + vec2!(24, -44))
				,
			&shapes::text(&format!("seed: {}", self.seed))
				.align(gfx::Origin::TopLeft)
				.size(12.0)
				,
		)?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
		.run::<Game>();
}

