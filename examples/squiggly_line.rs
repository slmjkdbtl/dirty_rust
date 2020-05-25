// wengwengweng

use dirty::*;
use dirty::math::*;
use gfx::shapes;
use input::Key;

struct Polyline {
	line: Vec<Vec2>,
}

impl Polyline {
	fn new(line: Vec<Vec2>) -> Self {
		Self {
			line,
		}
	}

	fn push(&mut self, p: Vec2) {
		self.line.push(p);
	}

	fn clear(&mut self) {
		self.line.clear();
	}

	fn draw(&self, d: &mut Ctx) -> Result<()> {
		for (p0, p1) in self.line.iter().zip(self.line.iter().skip(1)) {
			d.gfx.draw(
				&shapes::line(*p0, *p1)
					.width(2.0)
					// .color()
			)?
		}
		Ok(())
	}
}

struct Squiggly {
	frames: Vec<Polyline>,
}

impl Squiggly {
	fn new(buf: &Polyline, n: usize, tol: usize) -> Self {
		let mut frames = vec![];
		for _ in 0..n {
			let frame = buf.line.iter()
				.map(|&v| v + vec2!(rand(0,tol), rand(0,tol)))
				.collect::<Vec<_>>();
			frames.push(Polyline::new(frame));
		}
		Self {
			frames,
		}
	}

	fn draw(&self, t: usize, d: &mut Ctx) -> Result<()> {
		let f = &self.frames[t % self.frames.len()];
		f.draw(d)?;
		Ok(())
	}
}

struct Game {
	key_down: bool,
	lines: Vec<Squiggly>,
	buf: Polyline,
	t: usize,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		Ok(Self {
			key_down: false,
			lines: vec![],
			buf: Polyline::new(vec![]),
			t: 0,
		})
	}

	fn update(&mut self, _: &mut Ctx) -> Result<()> {
		self.t += 1;
		self.t %= 60;
		Ok(())
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {
		use input::Event::*;

		match e {
			MousePress(_) => {
				self.key_down = true;
			}
			MouseRelease(_) => {
				self.key_down = false;
				self.lines.push(Squiggly::new(&self.buf, 10, 3));
				self.buf.clear();
			}
			MouseMove(_) => {
				let pos = d.window.mouse_pos();
				if self.key_down {
					self.buf.push(pos);
				}
			}
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		Ok(())

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {
		self.buf.draw(d)?;
		for line in &self.lines {
			line.draw(self.t, d)?;
		}

		Ok(())

	}

}

fn main() {
	if let Err(err) = launcher()
		.run::<Game>() {
		println!("{}", err);
	}
}

