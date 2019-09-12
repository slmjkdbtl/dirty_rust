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

	pub fn width(&self) -> f32 {
		return self.frames[self.cur_frame].w * self.tex.width() as f32;
	}

	pub fn height(&self) -> f32 {
		return self.frames[self.cur_frame].h * self.tex.height() as f32;
	}

	pub fn tex_width(&self) -> i32 {
		return self.tex.width();
	}

	pub fn tex_height(&self) -> i32 {
		return self.tex.height();
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
	shader: gfx::Shader,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let tex = gfx::Texture::from_bytes(ctx, include_bytes!("res/car.png"))?;
		let mut sprite = Sprite::new(tex);

		sprite.slice(4, 1);

		let shader = gfx::Shader::effect(ctx, include_str!("res/outline.frag"))?;

		shader.send("size", 2.0);
		shader.send("color", color!(0, 0, 0, 1));
		shader.send("filter", color!(1));
		shader.send("resolution", vec2!(sprite.tex_width(), sprite.tex_height()));

		return Ok(Self {
			sprite: sprite,
			shader: shader,
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

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.sprite.next();
		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw(shapes::gradient(
			ctx.coord(gfx::Origin::Top),
			ctx.coord(gfx::Origin::Bottom),
			&[
				(color!(0.4, 1, 1, 1), 0.0),
				(color!(1, 1, 0.6, 1), 0.5),
				(color!(1, 0.4, 0.8, 1), 1.0),
			],
		).width(640.0))?;

		ctx.draw_with(&self.shader, |ctx| {
			ctx.draw(&self.sprite)?;
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

