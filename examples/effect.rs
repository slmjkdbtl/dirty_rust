// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Effect {
	name: String,
	shader: gfx::Shader,
	param: Option<Param>,
}

struct Param {
	name: String,
	value: f32,
}

impl Param {
	pub fn new(name: &str, value: f32) -> Self {
		return Self {
			name: name.to_owned(),
			value: value,
		};
	}
}

impl Effect {
	pub fn new(name: &str, shader: gfx::Shader, param: Option<Param>) -> Self {
		return Self {
			name: name.to_owned(),
			shader: shader,
			param: param,
		};
	}
}

struct Game {
	tex: gfx::Tex2D,
	effects: Vec<Effect>,
	cur_effect: Option<usize>,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let pixelate = gfx::Shader::effect(ctx, include_str!("res/pix.frag"))?;

		pixelate.send("size", 32.0);
		pixelate.send("dimension", vec2!(ctx.width(), ctx.height()));

		let blur = gfx::Shader::effect(ctx, include_str!("res/blur.frag"))?;

		blur.send("radius", 24.0);
		blur.send("dir", vec2!(1, 0));
		blur.send("dimension", vec2!(ctx.width(), ctx.height()));

		let effects = vec![
			Effect::new("pixlate", pixelate, Some(Param::new("size", 32.0))),
			Effect::new("blur", blur, Some(Param::new("radius", 24.0))),
		];

		return Ok(Self {
			tex: gfx::Tex2D::from_bytes(ctx, include_bytes!("../icon.png"))?,
			cur_effect: None,
			effects: effects,
		});

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let draw_icon = |ctx: &mut app::Ctx| -> Result<()> {

			ctx.push();
			ctx.scale(vec2!(2));
			ctx.draw(shapes::sprite(&self.tex))?;
			ctx.pop()?;

			return Ok(());

		};

		if let Some(cur_effect) = self.cur_effect {
			ctx.draw_with(&self.effects[cur_effect].shader, |ctx| {
				draw_icon(ctx)?;
				return Ok(());
			})?;
		} else {
			draw_icon(ctx)?;
		}

		ctx.push();
		ctx.translate(ctx.coord(gfx::Origin::Bottom) - vec2!(0, 48));

		if let Some(cur_effect) = self.cur_effect {

			if let Some(effect) = self.effects.get(cur_effect) {

				ctx.draw(shapes::text(&format!("cur effect: {}", effect.name)).color(color!(0, 1, 1, 1)))?;

				if let Some(param) = &effect.param {

					ctx.translate(vec2!(0, 20));
					ctx.scale(vec2!(0.8));
					ctx.draw(shapes::text(&format!("{}: {}", param.name, param.value)))?;

				}

			}

		} else {
			ctx.draw(shapes::text("no effect"))?;
		}

		ctx.pop()?;

		if ctx.key_pressed(Key::Left) || ctx.key_pressed(Key::A) {
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

		if ctx.key_pressed(Key::Right) || ctx.key_pressed(Key::D) {
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

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}
}

