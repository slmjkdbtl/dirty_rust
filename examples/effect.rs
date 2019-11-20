// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::app::*;
use input::Key;

// struct Effect<U> {
// 	name: String,
// 	shader: gfx::Shader2D<U>,
// }

// impl<U> Effect<U> {
// 	pub fn new(name: &str, shader: gfx::Shader2D<U>) -> Self {
// 		return Self {
// 			name: name.to_owned(),
// 			shader: shader,
// 		};
// 	}
// }

#[derive(Clone)]
struct PixUniform {
	resolution: Vec2,
	size: f32,
}

impl gfx::Uniform for PixUniform {
	fn values(&self) -> gfx::UniformValues {
		return vec![
			("resolution", self.resolution.as_arr().into()),
			("size", self.size.into()),
		];
	}
}

struct Game {
	tex: gfx::Texture,
// 	effects: Box<Vec<Effect<dyn gfx::Uniform>>>,
	cur_effect: Option<usize>,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let tex = gfx::Texture::from_bytes(ctx, include_bytes!("res/dedede.png"))?;
		let pixelate = gfx::Shader::from_frag(ctx, include_str!("res/pix.frag"))?;

		pixelate.send("size", 32.0);
		pixelate.send("dimension", vec2!(tex.width(), tex.height()));

		let blur = gfx::Shader::from_frag(ctx, include_str!("res/blur.frag"))?;

		blur.send("radius", 24.0);
		blur.send("dir", vec2!(1, 0));
		blur.send("dimension", vec2!(tex.width(), tex.height()));

		let grayscale = gfx::Shader::from_frag(ctx, include_str!("res/grayscale.frag"))?;
		let invert = gfx::Shader::from_frag(ctx, include_str!("res/invert.frag"))?;

// 		let effects = vec![
// 			Effect::new("pixlate", pixelate, Some(Param::new("size", 32.0))),
// 			Effect::new("blur", blur, Some(Param::new("radius", 24.0))),
// 			Effect::new("grayscale", grayscale, None),
// 			Effect::new("invert", invert, None),
// 		];

		return Ok(Self {
			tex: tex,
			cur_effect: None,
			effects: effects,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match *e {

			KeyPress(k) => {

				if k == Key::Left || k == Key::A {
					if let Some(cur_effect) = self.cur_effect {
						if cur_effect > 0 {
							self.cur_effect = Some(cur_effect - 1);
						} else {
							self.cur_effect = None;
						}
					} else {
						if !self.effects.is_empty() {
							self.cur_effect = Some(self.effects.len() - 1);
						}
					}
				}

				if k == Key::Right || k == Key::D {
					if let Some(cur_effect) = self.cur_effect {
						if cur_effect < self.effects.len() - 1 {
							self.cur_effect = Some(cur_effect + 1);
						} else {
							self.cur_effect = None;
						}
					} else {
						if !self.effects.is_empty() {
							self.cur_effect = Some(0);
						}
					}
				}

				if k == Key::Esc {
					ctx.quit();
				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if ctx.key_down(Key::Up) || ctx.key_down(Key::W) {
			if let Some(cur_effect) = self.cur_effect {
				if let Some(effect) = self.effects.get_mut(cur_effect) {
					if let Some(param) = &mut effect.param {
						param.value = param.value + ctx.dt() * 24.0;
						effect.shader.send(&param.name, param.value);
					}
				}
			}
		}

		if ctx.key_down(Key::Down) || ctx.key_down(Key::S) {
			if let Some(cur_effect) = self.cur_effect {
				if let Some(effect) = self.effects.get_mut(cur_effect) {
					if let Some(param) = &mut effect.param {
						param.value = param.value - ctx.dt() * 24.0;
						effect.shader.send(&param.name, param.value);
					}
				}
			}
		}

		let draw_icon = |ctx: &mut app::Ctx| -> Result<()> {

			ctx.push(&gfx::t()
				.translate(vec2!(0, -24))
				.scale(vec2!(0.5))
			, |ctx| {
				return ctx.draw(shapes::sprite(&self.tex));
			})?;

			return Ok(());

		};

		if let Some(cur_effect) = self.cur_effect {
			ctx.draw_2d_with(&self.effects[cur_effect].shader, |ctx| {
				draw_icon(ctx)?;
				return Ok(());
			})?;
		} else {
			draw_icon(ctx)?;
		}

		ctx.push(&gfx::t()
			.translate(ctx.coord(gfx::Origin::Bottom) - vec2!(0, 64))
		, |ctx| {

			if let Some(cur_effect) = self.cur_effect {

				if let Some(effect) = self.effects.get(cur_effect) {

					ctx.draw(shapes::text(&format!("effect: {}", effect.name)).color(rgba!(0, 1, 1, 1)))?;

					if let Some(param) = &effect.param {

						ctx.push(&gfx::t()
							.translate(vec2!(0, 16))
							.scale(vec2!(0.8))
						, |ctx| {
							return ctx.draw(shapes::text(&format!("{}: {:.*}", param.name, 0, param.value)));
						})?;

					}

				}

			} else {
				ctx.draw(shapes::text("no effect"))?;
			}

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}
}

