// wengwengweng

use dirty::*;
use kit::*;
use sprite::*;
use input::Key;

struct Game {
	sprite: Sprite,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let mut sprite = Sprite::from_bytes(d.gfx, include_bytes!("res/car.png"))?;

		sprite.slice(4, 1);
		sprite.add_anim("run", 0, 3, true);
		sprite.play("run");

		return Ok(Self {
			sprite,
		});

	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::F => d.window.toggle_fullscreen(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		d.window.set_title(&format!("FPS: {} DCS: {}", d.app.fps(), d.gfx.draw_calls()));
		self.sprite.update(d.app.dt());

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw(&self.sprite)?;

		return Ok(());

	}

}

fn main() {

	if let Err(e) = launcher()
		.resizable(true)
		.run::<Game>() {
		log!("{}", e);
	}

}

