// wengwengweng

use dirty::*;
use kit::*;
use sprite::*;
use input::Key;

struct Game {
	sprite: Sprite,
	canvas: gfx::Canvas,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let mut sprite = Sprite::from_bytes(ctx, include_bytes!("res/car.png"))?;

		sprite.slice(4, 1);
		sprite.add_anim("run", 0, 3, true);
		sprite.play("run");

		return Ok(Self {
			canvas: gfx::Canvas::from_conf(ctx, &gfx::CanvasConf {
				size: (240, 240),
				origin: gfx::Origin::Center,
			})?,
			sprite: sprite,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match *e {
			KeyPress(k) => {
				match k {
					Key::Esc => ctx.quit(),
					Key::F => ctx.toggle_fullscreen(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));
		self.sprite.update(ctx.dt());

		ctx.draw_on(&self.canvas, |ctx| {
			ctx.clear();
			ctx.draw(&shapes::rect(vec2!(-1000, 1000), vec2!(1000, -1000)).fill(rgba!(0, 1, 1, 1)))?;
			ctx.draw(&self.sprite)?;
			return Ok(());
		})?;

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&shapes::canvas(&self.canvas))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}

}

