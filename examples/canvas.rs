// wengwengweng

use dirty::*;
use app::*;
use kit::*;
use input::Key;

struct Game {
	sprite: Sprite,
	canvas: gfx::Canvas,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut sprite = Sprite::from_bytes(ctx, include_bytes!("res/car.png"))?;

		sprite.slice(4, 1);
		sprite.add_anim("run", 0, 3, true);
		sprite.play("run");

		return Ok(Self {
			canvas: gfx::Canvas::builder()
				.origin(gfx::Origin::Center)
				.size(320, 480)
				.build(ctx)?,
			sprite: sprite,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match *e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
				if k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));
		self.sprite.update(ctx.dt());

		ctx.draw_on(&self.canvas, |ctx| {
			ctx.clear();
			ctx.draw(&shapes::rect(vec2!(-1000), vec2!(1000)).fill(rgba!(0, 0, 1, 1)))?;
			ctx.draw(&shapes::rect(vec2!(-100), vec2!(100)).fill(rgba!(1, 0, 1, 1)))?;
			return Ok(());
		})?;

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw(&shapes::canvas(&self.canvas))?;
		ctx.draw(&self.sprite)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}

}

