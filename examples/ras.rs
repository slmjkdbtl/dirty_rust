// wengwengweng

use dirty::*;
use app::*;
use math::*;
use input::Key;

struct Canvas {
	buf: Vec<u8>,
	width: u32,
	height: u32,
}

impl Canvas {

	pub fn new(w: u32, h: u32) -> Self {
		return Self {
			width: w,
			height: h,
			buf: vec![0; w as usize * h as usize * 4],
		};
	}

	pub fn clear(&mut self) {
		self.buf = vec![0; self.width as usize * self.height as usize * 4];
	}

	pub fn put(&mut self, p: Vec2, c: Color) {

		let i = p.y as usize * self.width as usize * 4 + p.x as usize * 4;

		self.buf.get_mut(i + 0).map(|i| *i = (c.r * 255.0) as u8);
		self.buf.get_mut(i + 1).map(|i| *i = (c.g * 255.0) as u8);
		self.buf.get_mut(i + 2).map(|i| *i = (c.b * 255.0) as u8);
		self.buf.get_mut(i + 3).map(|i| *i = (c.a * 255.0) as u8);

	}

	pub fn rect(&mut self, p1: Vec2, p2: Vec2) {

		// ...

	}

	pub fn line(&mut self, p1: Vec2, p2: Vec2, c: Color) {

		let mut x0 = f32::round(p1.x) as i32;
		let mut y0 = f32::round(p1.y) as i32;
		let mut x1 = f32::round(p2.x) as i32;
		let mut y1 = f32::round(p2.y) as i32;

		if x0 > x1 {
			std::mem::swap(&mut x0, &mut x1);
			std::mem::swap(&mut y0, &mut y1);
		}

		let dx = x1 - x0;
		let dy = y1 - y0;

		let derr = i32::abs(dy / dx);
		let mut err = 0;
		let mut y = y0;

		for x in x0..x1 {

// 			if (steep) {
// 				image.set(y, x, color);
// 			} else {
				self.put(vec2!(x, y), c);
// 			}
			err += derr;

			if (err > dx) {

				y += if (y1 > y0) {
					1
				} else {
					-1
				};

				err -= dx * 2;

			}

		}

	}

}

struct Game {
	canvas: Canvas,
	tex: gfx::Texture,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			canvas: Canvas::new(640, 480),
			tex: gfx::Texture::new(ctx, 640, 480)?,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					_ => {},
				}
			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.canvas.clear();
		self.canvas.line(vec2!(100), ctx.mouse_pos(), rgba!(1));
		self.tex.data(&self.canvas.buf);

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw(&shapes::sprite(&self.tex))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

