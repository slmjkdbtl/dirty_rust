// wengwengweng

#![feature(box_syntax)]

use dirty::*;
use app::*;
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
			NoiseType::Turbulence => "turbulence",
		};
	}
}

impl Game {

	fn gen(&mut self, ctx: &Ctx) -> Result<()> {

		let seed = self.seed as u32;

		let noise: Box<dyn NoiseFn<[f64; 2]>> = match self.noise_type {
			NoiseType::Perlin => box Perlin::new().set_seed(seed),
			NoiseType::OpenSimplex => box OpenSimplex::new().set_seed(seed),
			NoiseType::SuperSimplex => box SuperSimplex::new().set_seed(seed),
			NoiseType::Fbm => box Fbm::new().set_seed(seed),
			NoiseType::Billow => box Billow::new().set_seed(seed),
			NoiseType::Worley => box Worley::new().enable_range(true).set_seed(seed),
			NoiseType::RidgedMulti => box RidgedMulti::new().set_seed(seed),
			NoiseType::Turbulence => box Turbulence::new(Fbm::new()).set_seed(seed),
		};

		let w = ctx.gwidth() as i32;
		let h = ctx.gheight() as i32;
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

		let img = img::Image::from_pixels(w, h, buf)?;

		self.tex = gfx::Texture::from_img(ctx, img)?;

		return Ok(());

	}

}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let mut g = Self {
			tex: gfx::Texture::new(ctx, ctx.width(), ctx.height())?,
			noise_type: NoiseType::Perlin,
			seed: 0,
		};

		g.gen(ctx)?;

		return Ok(g);

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				let mods = ctx.key_mods();

				match *k {
					Key::Esc => ctx.quit(),
					Key::Q if mods.meta => ctx.quit(),
					Key::F if mods.meta => ctx.toggle_fullscreen(),
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
						self.gen(ctx)?;
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
						self.gen(ctx)?;
					},

					Key::Left => {
						self.seed = self.seed.wrapping_sub(1);
						self.gen(ctx)?;
					},

					Key::Right => {
						self.seed = self.seed.wrapping_add(1);
						self.gen(ctx)?;
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

		ctx.draw_t(
			&gfx::t()
				.t2(vec2!(24)),
			&shapes::text(&format!("type: {}", self.noise_type.as_str()))
		)?;

		ctx.draw_t(
			&gfx::t()
				.t2(vec2!(24, 44)),
			&shapes::text(&format!("seed: {}", self.seed))
		)?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Game>();
}

