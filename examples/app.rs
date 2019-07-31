// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

use gfx::UniformValue;

struct Sprite {

	tex: gfx::Texture,
	frames: Vec<Quad>,
	cur_frame: usize,
	looping: bool,

}

impl Sprite {

	pub fn new(tex: gfx::Texture) -> Self {
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
	canvas: gfx::Canvas,
	effect: gfx::Shader,
// 	track: audio::Track,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let tex = gfx::Texture::from_bytes(ctx, include_bytes!("res/car.png"))?;
		let mut sprite = Sprite::new(tex);
// 		let track = audio::track(&audio::Sound::from_bytes(include_bytes!("res/yo.ogg"))?)?;

// 		track.play();
		sprite.slice(4, 1);

		return Ok(Self {
			sprite: sprite,
			canvas: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
			effect: gfx::Shader::effect(ctx, include_str!("res/noise.frag"))?,
// 			track: track,
		});

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.sprite.next();

		ctx.draw_on(&self.canvas, |ctx| {
			ctx.draw(&self.sprite)?;
			return Ok(());
		})?;

// 		ctx.draw_with(&self.effect, |ctx| {
			ctx.draw(shapes::canvas(&self.canvas))?;
// 			return Ok(());
// 		})?;

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		if ctx.key_pressed(Key::F) {
			ctx.toggle_fullscreen();
		}

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher().origin(gfx::Origin::Center).run::<Game>() {
		println!("{}", err);
	}

}

