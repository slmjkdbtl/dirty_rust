// wengwengweng

const N_FRAMES: usize = 10;

use dirty::*;
use dirty::math::*;
use gfx::shapes;
use input::Key;

struct Polyline {
	line: Vec<Vec2>,
	color: Color,
}

impl Polyline {
	fn new(line: Vec<Vec2>, color: Color) -> Self {
		Self {
			line,
			color,
		}
	}

	fn push(&mut self, p: Vec2) {
		self.line.push(p);
	}

	fn clear(&mut self) {
		self.line.clear();
	}

	fn draw(&self, gfx: &mut gfx::Gfx) -> Result<()> {
		gfx.draw(
			&shapes::lines(&self.line)
				.width(2.0)
				.color(self.color)
		)?;
		Ok(())
	}

	fn render(&self, gfx: &mut gfx::Gfx, offset: Vec2) -> Result<()> {
		for (p0, p1) in self.line.iter().zip(self.line.iter().skip(1)) {
			gfx.draw(
				&shapes::line(*p0 + offset, *p1 + offset)
					.width(2.0)
					.color(self.color)
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
			frames.push(Polyline::new(frame, buf.color));
		}
		Self {
			frames,
		}
	}

	fn draw(&self, t: usize, gfx: &mut gfx::Gfx) -> Result<()> {
		let f = &self.frames[t % self.frames.len()];
		f.draw(gfx)?;
		Ok(())
	}

	fn render(&self, gfx: &mut gfx::Gfx, sz: isize) -> Result<()> {
		for (i, frame) in self.frames.iter().enumerate() {
			let off = vec2!(sz / 2, 0) +
					  vec2!(-sz * (N_FRAMES as isize) / 2, 0) +
					  vec2!((i as isize) * sz as isize, 0);
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

	fn init(d: &mut Ctx) -> Result<Self> {
		Ok(Self {
			key_down: false,
			lines: vec![],
			buf: Polyline::new(vec![], rgba!(1.)),
			t: 0,
			ui: ui::UI::new(d)?,
			tol: 3,
			density: 3.,
			sz: 200,
		})
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {
		use input::Event::*;
		if self.ui.event(d, &e) {
			return Ok(());
		}
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
						if Vec2::dist(pos, *last) > self.density {
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

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		self.t += 1;
		self.t %= 60;

		let top_left = d.gfx.coord(gfx::Origin::TopLeft);
		let top_right = d.gfx.coord(gfx::Origin::TopRight);

		let mut tol = 0;
		let mut density = 0.;
		let mut sz = 0;
		let mut save = false;
		let mut fname = String::new();
		let mut color = None;

		self.ui.frame(d, |mut ui| {

			ui.window("options", top_left + vec2!(60, -60), 240.0, 360.0, |mut p| {

				tol = p.slider::<usize>("tol", 3, 1, 10)?;
				density = p.slider::<f32>("d", 3., 1.0, 10.0)?;
				sz = p.slider::<isize>("sz", 200, 10, 500)?;
				fname = p.input("filename")?;
				p.text(".png")?;
				save = p.button("save")?;

				Ok(())

			})?;

			ui.window("color", top_right + vec2!(-60.0 - 240.0, -60), 240.0, 360.0, |mut p| {
				if p.button("red")? { color = Some(rgba!(1,0,0,1)); }
				if p.button("green")? { color = Some(rgba!(0,1,0,1)); }
				if p.button("blue")? { color = Some(rgba!(0,0,1,1)); }
				if p.button("white")? { color = Some(rgba!(1)); }
				if p.button("black")? { color = Some(rgba!(0,0,0,1)); }
				Ok(())
			})?;

			Ok(())

		})?;

		self.tol = tol;
		self.density = density;
		self.sz = sz;

		if save {
			self.save(d.gfx, &fname)?;
		}

		if let Some(col) = color {
			self.buf.color = col;
		}

		return Ok(());

	}

	fn draw(&self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw(
			&shapes::rect(vec2!(-self.sz, self.sz) * 0.5, vec2!(self.sz, -self.sz) * 0.5)
				.fill(rgba!(0.5, 0.5, 0.5, 1)),
		)?;

		self.buf.draw(d.gfx)?;

		for line in &self.lines {
			line.draw(self.t, d.gfx)?;
		}

		d.gfx.draw(&shapes::canvas(self.ui.canvas()))?;

		Ok(())

	}

}

impl Game {

	fn save(&self, ctx: &mut gfx::Gfx, fname: &str) -> Result<()> {

		if fname.is_empty() {
			return Ok(());
		}

		let fbuf = gfx::Canvas::new(ctx, self.sz as i32 * N_FRAMES as i32, self.sz as i32)?;

		ctx.draw_on(&fbuf, |gfx| {
			for line in &self.lines {
				line.render(gfx, self.sz)?;
			}
			Ok(())
		})?;

		let img = fbuf.capture()?;
		let img = img.resize(fbuf.width(), fbuf.height(), img::FilterType::Nearest)?;

		img.save(format!("{}.png", fname))?;

		Ok(())

	}

}

fn main() {
	if let Err(e) = launcher()
		.size(1024, 768)
		.run::<Game>() {
		elog!("{}", e);
	}
}
