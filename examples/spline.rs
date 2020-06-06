// wengwengweng

use dirty::*;
use math::*;
use geom::*;
use gfx::shapes;
use input::Key;

const SPEED: f32 = 120.0;

struct Game {
	pts: Vec<Vec2>,
	cur_pt: usize,
	draggin: bool,
	looping: bool,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			pts: vec![
				vec2!(-240, -160),
				vec2!(-120, 120),
				vec2!(0),
				vec2!(120),
				vec2!(240, 0),
			],
			cur_pt: 0,
			draggin: false,
			looping: true,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::Backspace => {
						self.pts.remove(self.cur_pt);
						if self.cur_pt >= self.pts.len() {
							self.cur_pt = self.pts.len() - 1;
						}
					},
					Key::Space => {
						self.pts.insert(self.cur_pt + 1, d.window.mouse_pos());
						self.cur_pt += 1;
					},
					Key::L => self.looping = !self.looping,
					_ => {},
				}
			},
			KeyPressRepeat(k) => {
				match *k {
					Key::Left => {
						if self.cur_pt == 0 {
							self.cur_pt = self.pts.len() - 1;
						} else {
							self.cur_pt -= 1;
						}
					},
					Key::Right => self.cur_pt = (self.cur_pt + 1) % self.pts.len(),
					Key::Tab => self.cur_pt = (self.cur_pt + 1) % self.pts.len(),
					_ => {},
				}
			},
			MousePress(_) => {
				let mpos = d.window.mouse_pos();
				for (i, p) in self.pts.iter().enumerate() {
					let area = Rect::new(*p - vec2!(8), *p + vec2!(8));
					if col::intersect2d(mpos, area) {
						self.cur_pt = i;
						self.draggin = true;
						break;
					}
				}
			},
			MouseRelease(_) => {
				self.draggin = false;
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		let dt = d.app.dt().as_secs_f32();
		let mut pt = self.pts[self.cur_pt];

		if d.window.key_down(Key::W) {
			pt.y += dt * SPEED;
		}

		if d.window.key_down(Key::S) {
			pt.y -= dt * SPEED;
		}

		if d.window.key_down(Key::D) {
			pt.x += dt * SPEED;
		}

		if d.window.key_down(Key::A) {
			pt.x -= dt * SPEED;
		}

		self.pts[self.cur_pt] = pt;

		if self.draggin {
			self.pts[self.cur_pt] = d.window.mouse_pos();
		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		let spts = if self.looping {
			geom::meshgen::spline_loop(&self.pts)
		} else {
			geom::meshgen::spline(&self.pts)
		};

		d.gfx.draw(
			&shapes::lines(&spts)
				.cap(shapes::LineCap::Round)
				.width(4.0)
		)?;

		d.gfx.draw(
			&shapes::points(&self.pts)
				.color(rgba!(1, 0, 0, 1))
				.size(16.0)
		)?;

		d.gfx.draw(
			&shapes::points(&[self.pts[self.cur_pt]])
				.color(rgba!(0, 0, 1, 1))
				.size(16.0)
		)?;

		for (i, pt) in self.pts.iter().enumerate() {
			d.gfx.draw_t(
				mat4!()
					.t2(*pt)
					,
				&shapes::text(&format!("{}", i))
					.size(12.0)
			)?;
		}

		return Ok(());

	}

}

fn main() {
	if let Err(e) = run::<Game>() {
		elog!("{}", e);
	}
}

