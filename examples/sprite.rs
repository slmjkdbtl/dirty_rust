// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Sprite {

	tex: gfx::Tex2D,
	frames: Vec<Quad>,
	cur_frame: usize,
	looping: bool,

}

impl Sprite {

	pub fn new(tex: gfx::Tex2D) -> Self {
		return Self {
			tex: tex,
			frames: vec![quad!(0, 0, 1, 1)],
			cur_frame: 0,
			looping: true,
		};
	}

	pub fn slice(&mut self, x: u8, y: u8) {

		let w = 1.0 / x as f32;
		let h = 1.0 / y as f32;

		self.frames.clear();

		for i in 0..x as usize {
			for j in 0..y as usize {
				self.frames.push(quad!(i as f32 * w, j as f32 * h, w, h));
			}
		}

	}

	pub fn next(&mut self) {
		if self.cur_frame < self.frames.len() - 1 {
			self.cur_frame += 1;
		} else {
			if self.looping {
				self.cur_frame = 0;
			}
		}
	}

	pub fn prev(&mut self) {
		if self.cur_frame > 0 {
			self.cur_frame -= 1;
		} else {
			if self.looping {
				self.cur_frame = self.frames.len() - 1;
			}
		}
	}

}

impl gfx::Drawable for &Sprite {
	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {
		return ctx.draw(
			shapes::sprite(&self.tex)
				.quad(self.frames[self.cur_frame])
		);
	}
}

struct Game {
	sprite: Sprite,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let tex = gfx::Tex2D::from_bytes(ctx, include_bytes!("res/car.png"))?;
		let mut sprite = Sprite::new(tex);

		sprite.slice(4, 1);

		return Ok(Self {
			sprite: sprite,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
				if *k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.sprite.next();
		ctx.draw(&self.sprite)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

