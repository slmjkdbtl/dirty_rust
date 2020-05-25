// wengwengweng

const N_FRAMES: usize = 10;

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

	fn render(&self, gfx: &mut gfx::Gfx, offset: Vec2) -> Result<()> {
		for (p0, p1) in self.line.iter().zip(self.line.iter().skip(1)) {
			gfx.draw(
				&shapes::line(*p0 + offset, *p1 + offset)
					.width(2.0)
					// .color()
			)?;
		}
		Ok(())
	}
}

struct Squiggly {
	frames: Vec<Polyline>,
}

impl Squiggly {
	fn new(buf: &Polyline, n_frames: usize, tol: usize) -> Self {
		let mut frames = vec![];
		for _ in 0..n_frames {
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

	fn render(&self, gfx: &mut gfx::Gfx, sz: usize, offset: Vec2) -> Result<()> {
		for (i, frame) in self.frames.iter().enumerate() {
			let off = offset + vec2!(-(i as isize) * sz as isize, 0);
			frame.render(gfx, off)?;
		}
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
				self.lines.push(Squiggly::new(&self.buf, N_FRAMES, self.tol));
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
		let tl = vec2!(0, 240);

		d.gfx.draw(
			&shapes::rect(tl, tl + vec2!(self.sz, -self.sz))
				.fill(rgba!(0.1, 0.1, 0.1, 1)),
		)?;


		self.buf.draw(d)?;
		for line in &self.lines {
			line.draw(self.t, d)?;
		}


		let mut tol = 0;
		let mut density = 0.;
		let mut sz = 0;
		let mut save = false;
		let mut fname = String::new();
		self.ui.window(d, "options", top_left, 240.0, 360.0, |ctx, p| {

			tol = p.slider(ctx, "tol", 3., 1.0, 10.0)? as usize;
			density = p.slider(ctx, "d", 3., 1.0, 10.0)?;
			sz = p.slider(ctx, "sz", 30., 10., 360.)? as isize;
			fname = p.input(ctx, "filename")?;
			p.text(ctx, ".png")?;
			save = p.button(ctx, "save")?;

			Ok(())

		})?;
		self.tol = tol;
		self.density = density;
		self.sz = sz;
		if save{
			self.save(d.gfx, &fname);
		}

		Ok(())

	}

}

impl Game {
	fn save(&self, ctx: &mut gfx::Gfx, fname: &str) {
		let fbuf = gfx::Canvas::new(ctx, self.sz as i32 * N_FRAMES as i32, self.sz as i32).unwrap();
		ctx.draw_on(&fbuf, |gfx| {
			for line in &self.lines {
				line.render(gfx, self.sz as usize, vec2!(0, 240))?;
			}
			Ok(())
		});
		fbuf.capture(format!("{}.png", fname));
	}
}

fn main() {
	if let Err(err) = launcher()
		.size(1024, 768)
		.run::<Game>() {
		println!("{}", err);
	}
}

