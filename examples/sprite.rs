// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Sprite {

	tex: gfx::Texture,
	frames: Vec<Quad>,
	cur_frame: usize,
	looping: bool,
	color: Color,

}

impl Sprite {

	pub fn new(tex: gfx::Texture) -> Self {
		return Self {
			tex: tex,
			frames: vec![quad!(0, 0, 1, 1)],
			cur_frame: 0,
			looping: true,
			color: color!(),
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

	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}

}

impl gfx::DrawCmd for &Sprite {
	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {
		return ctx.draw(
			shapes::sprite(&self.tex)
				.quad(self.frames[self.cur_frame])
				.color(self.color)
		);
	}
}

struct Game {
	sprite: Sprite,
	canvas: gfx::Canvas,
	effect: gfx::Shader,
	pix_size: f32,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let tex = gfx::Texture::from_bytes(ctx, include_bytes!("res/car.png"))?;
		let mut sprite = Sprite::new(tex);

		sprite.slice(4, 1);

		let effect = gfx::Shader::effect(ctx, include_str!("res/pix.frag"))?;

		effect.send("size", 0.0);
		effect.send("dimension", vec2!(ctx.width(), ctx.height()));

		return Ok(Self {
			sprite: sprite,
			canvas: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
			effect: effect,
			pix_size: 0.0,
		});

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.sprite.next();

		ctx.draw_on(&self.canvas, |ctx| {
			ctx.clear();
			ctx.draw(&self.sprite)?;
			return Ok(());
		})?;

		ctx.draw_with(&self.effect, |ctx| {
			ctx.draw(shapes::canvas(&self.canvas))?;
			return Ok(());
		})?;

// 		ctx.draw_masked(|ctx| {
// 			ctx.draw(shapes::rect(100.0, 100.0))?;
// 			return Ok(());
// 		}, |ctx| {
// 			ctx.draw(&self.sprite)?;
// 			return Ok(());
// 		})?;

		self.pix_size = rand!() * 6.0;
		self.effect.send("size", self.pix_size);

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

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

