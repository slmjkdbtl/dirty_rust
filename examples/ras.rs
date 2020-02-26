// wengwengweng

use rayon::prelude::*;
use dirty::*;
use math::*;
use input::Key;

struct Canvas {
	buf: Vec<Color>,
	width: u32,
	height: u32,
}

impl Canvas {

	pub fn new(w: u32, h: u32) -> Self {
		return Self {
			width: w,
			height: h,
			buf: vec![rgba!(0); w as usize * h as usize],
		};
	}

	pub fn width(&self) -> u32 {
		return self.width;
	}

	pub fn height(&self) -> u32 {
		return self.height;
	}

	pub fn buf(&self) -> &[Color] {
		return &self.buf;
	}

	pub fn as_u8(&self) -> Vec<u8> {

		let mut buf = Vec::with_capacity(self.width as usize * self.height as usize * 4);

		for c in &self.buf {
			buf.push((c.r * 255.0) as u8);
			buf.push((c.g * 255.0) as u8);
			buf.push((c.b * 255.0) as u8);
			buf.push((c.a * 255.0) as u8);
		}

		return buf;

	}

	pub fn clear(&mut self) {
		self.buf = vec![rgba!(0); self.width as usize * self.height as usize];
	}

	pub fn put(&mut self, x: usize, y: usize, c: Color) {

		let i = y * self.width as usize + x;

		self.buf.get_mut(i).map(|i| *i = c);

	}

	pub fn shade(&mut self, f: impl Fn(usize, usize, Color) -> Color + Sync + Send) {

		let w = self.width as usize;

		self.buf.par_iter_mut().enumerate().for_each(|(i, c)| {

			let x = i % w;
			let y = i / w;

			*c = f(x, y, *c);

		});

	}

	pub fn rect(&mut self, p1: Vec2, p2: Vec2, col: Color) {

		let p1 = p1.clamp(vec2!(0), vec2!(self.width, self.height));
		let p2 = p2.clamp(vec2!(0), vec2!(self.width, self.height));

		let x1 = f32::min(p1.x, p2.x) as usize;
		let x2 = f32::max(p1.x, p2.x) as usize;
		let y1 = f32::min(p1.y, p2.y) as usize;
		let y2 = f32::max(p1.y, p2.y) as usize;

		for x in x1..x2 {
			for y in y1..y2 {
				self.put(x, y, col);
			}
		}

	}

	pub fn line(&mut self, p1: Vec2, p2: Vec2, col: Color) {

		let p1 = p1.clamp(vec2!(0), vec2!(self.width, self.height));
		let p2 = p2.clamp(vec2!(0), vec2!(self.width, self.height));

		let mut x0 = p1.x as i32;
		let mut x1 = p2.x as i32;
		let mut y0 = p1.y as i32;
		let mut y1 = p2.y as i32;

		let mut steep = false;

		if (i32::abs(x0 - x1) < i32::abs(y0 - y1)) {
			std::mem::swap(&mut x0, &mut y0);
			std::mem::swap(&mut x1, &mut y1);
			steep = true;
		}

		if (x0 > x1) {
			std::mem::swap(&mut x0, &mut x1);
			std::mem::swap(&mut y0, &mut y1);
		}

		let dx = x1 - x0;
		let dy = y1 - y0;
		let derr = i32::abs(dy) * 2;
		let mut err = 0;
		let mut y = y0;

		for x in x0..x1 {

			if (steep) {
				self.put(y as usize, x as usize, col);
			} else {
				self.put(x as usize, y as usize, col);
			}

			err += derr;

			if (err > dx) {
				y += i32::signum(dy);
				err -= dx * 2;
			}

		}

	}

}

struct Game {
	canvas: Canvas,
	tex: gfx::Texture,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let w = 320;
		let h = 240;

		return Ok(Self {
			canvas: Canvas::new(w, h),
			tex: gfx::Texture::new(ctx, w as i32, h as i32)?,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

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

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.set_title(&format!("{}", ctx.fps()));
		self.canvas.clear();

		let w = self.canvas.width() as f32;
		let h = self.canvas.height() as f32;
		let t = ctx.time();

		self.canvas.shade(|x, y, c| {

			let uv = vec2!(x, y) / vec2!(w, h);
			let angle = f32::atan2(uv.y, uv.x);
			let dis = Vec2::dis(uv, vec2!(0.5));

			let time = t * 4.0;

			let c1 = f32::sin(dis * 50.0 + time + angle);
			let c2 = f32::sin(dis * 50.0 + time + angle + (1.0 / 3.0) * 3.14 * 2.0);
			let c3 = f32::sin(dis * 50.0 + time + angle + (2.0 / 3.0) * 3.14 * 2.0);

			return rgba!(c1, c2, c3, 1);

		});

		let m = ctx.mouse_pos() * vec2!(0.5, -0.5) + vec2!(160, 120);

// 		self.canvas.rect(vec2!(100), m, rgba!(1));
// 		self.canvas.line(vec2!(100), m, rgba!(1));

		self.tex.data(&self.canvas.as_u8());

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&shapes::sprite(&self.tex).width(640.0).height(480.0))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.fps_cap(None)
		.vsync(false)
		.run::<Game>() {
		println!("{}", err);
	}

}

