// wengwengweng

use dirty::*;
use kit::*;
use sprite::*;
use gfx::*;
use input::*;

struct Game {
	tex: Texture,
	sprite: Sprite,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let tex = Texture::from_bytes(d.gfx, include_bytes!("res/car.png"))?;
		let mut sprite = Sprite::new();

		sprite.slice(4, 1);
		sprite.add("run", sprite::Anim {
			from: 0,
			to: 3,
			looping: true,
		});
		sprite.play("run");

		return Ok(Self {
			sprite: sprite,
			tex: tex,
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

		d.gfx.draw(&shapes::sprite(&self.tex).quad(self.sprite.frame()))?;

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

