// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Effect {
	name: String,
	shader: gfx::Shader,
}

impl Effect {
	pub fn new(name: &str, shader: gfx::Shader) -> Self {
		return Self {
			name: name.to_owned(),
			shader: shader,
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

		pixelate.send("size", 24.0);
		pixelate.send("dimension", vec2!(ctx.width(), ctx.height()));

		let blur = gfx::Shader::effect(ctx, include_str!("res/blur.frag"))?;

		blur.send("radius", 24.0);
		blur.send("dimension", vec2!(ctx.width(), ctx.height()));

		return Ok(Self {
			tex: gfx::Tex2D::from_bytes(ctx, include_bytes!("../icon.png"))?,
			effects: vec![Effect::new("pixlate", pixelate), Effect::new("blur", blur)],
			cur_effect: None,
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

		let draw_label = |ctx: &mut app::Ctx, txt: &str, color: math::Color| -> Result<()> {

			ctx.push();
			ctx.translate(ctx.coord(gfx::Origin::Bottom) - vec2!(0, 48));
			ctx.draw(shapes::text(txt).color(color))?;
			ctx.pop()?;

			return Ok(());

		};

		if let Some(cur_effect) = self.cur_effect {

			ctx.draw_with(&self.effects[cur_effect].shader, |ctx| {
				draw_icon(ctx)?;
				return Ok(());
			})?;

			draw_label(ctx, &format!("cur effect: {}", self.effects[cur_effect].name), color!(0, 0, 1, 1))?;

		} else {

			draw_icon(ctx)?;
			draw_label(ctx, "no effect", color!(1))?;

		}

		if ctx.key_pressed(Key::Space) {

			if let Some(cur_effect) = self.cur_effect {
				if cur_effect < self.effects.len() - 1 {
					self.cur_effect = Some(cur_effect + 1);
				} else {
					self.cur_effect = None;
				}
			} else {
				self.cur_effect = Some(0);
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

