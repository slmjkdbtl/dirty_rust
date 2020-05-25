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
	ui: ui::UI,

	tol: usize,
	density: f32,
	sz: isize,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		Ok(Self {
			key_down: false,
			lines: vec![],
			buf: Polyline::new(vec![]),
			t: 0,
			ui: ui::UI::new(),
			tol: 3,
			density: 3.,
			sz: 100,
		})
	}

	fn update(&mut self, _: &mut Ctx) -> Result<()> {
		self.t += 1;
		self.t %= 60;
		Ok(())
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {
		use input::Event::*;
		self.ui.event(d, &e);
		match e {
			MousePress(_) => {
				self.key_down = true;
			}
			MouseRelease(_) => {
				self.key_down = false;
				self.lines.push(Squiggly::new(&self.buf, 10, self.tol));
				self.buf.clear();
			}
			MouseMove(_) => {
				let pos = d.window.mouse_pos();
				if self.key_down {
					if let Some(last) = self.buf.line.last() {
						if pos.dist(*last) > self.density {
							self.buf.push(pos);
						}
					} else {
						self.buf.push(pos);
					}
				}
			}
			KeyPress(k) => {
				match *k {
					Key::Z => {
						self.lines.pop();
					}
					Key::C => {
						self.lines.clear();
						self.buf.clear();
					},
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		Ok(())

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {
		let top_left = d.gfx.coord(gfx::Origin::TopLeft);
		let bottom_right = d.gfx.coord(gfx::Origin::BottomRight);

		let orig = vec2!(0, 240);
		d.gfx.draw(
			&shapes::rect(orig, orig + vec2!(self.sz, -self.sz)).fill(rgba!(0.1, 0.1, 0.1, 1)),
		);

		self.buf.draw(d)?;
		for line in &self.lines {
			line.draw(self.t, d)?;
		}


		let mut tol = 0;
		let mut density = 0.;
		let mut sz = 0;
		self.ui.window(d, "options", top_left, 240.0, 360.0, |ctx, p| {

			tol = p.slider(ctx, "tol", 3., 1.0, 10.0)? as usize;
			density = p.slider(ctx, "d", 3., 1.0, 10.0)?;
			sz = p.slider(ctx, "sz", 30., 10., 360.)? as isize;

			Ok(())

		})?;
		self.tol = tol;
		self.density = density;
		self.sz = sz;

		Ok(())

	}

}

fn main() {
	if let Err(err) = launcher()
		.run::<Game>() {
		println!("{}", err);
	}
}

