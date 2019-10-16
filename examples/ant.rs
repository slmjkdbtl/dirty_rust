// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::app::*;
use input::Key;

const GRID_SIZE: i32 = 4;

const GRIDS: [Color; 6] = [
	Color::RED,
	Color::GREEN,
	Color::CYAN,
	Color::YELLOW,
	Color::PURPLE,
	Color::BLUE,
];

#[derive(Clone, Copy)]
enum Dir {
	Left,
	Bottom,
	Right,
	Up,
}

impl Dir {
	fn turn_left(&self) -> Self {
		return match self {
			Self::Left => Self::Bottom,
			Self::Bottom => Self::Right,
			Self::Right => Self::Up,
			Self::Up => Self::Left,
		};
	}
	fn turn_right(&self) -> Self {
		return match self {
			Self::Left => Self::Up,
			Self::Bottom => Self::Left,
			Self::Right => Self::Bottom,
			Self::Up => Self::Right,
		};
	}
	fn forward(&self, (x, y): (i32, i32)) -> (i32, i32) {
		return match self {
			Self::Left => (x - 1, y),
			Self::Bottom => (x, y + 1),
			Self::Right => (x + 1, y),
			Self::Up => (x, y - 1),
		};
	}
}

struct Ant {
	dir: Dir,
	x: i32,
	y: i32,
}

struct Sim {
	grids: Vec<Vec<Option<u8>>>,
	ant: Ant,
	count: usize,
}

impl Sim {

	fn next(&mut self) {

		let x = self.ant.x as usize;
		let y = self.ant.y as usize;

		let cols = match self.grids.get(x).map(Clone::clone) {
			Some(c) => c,
			None => return,
		};

		let cur = match cols.get(y).map(Clone::clone) {
			Some(g) => g,
			None => return,
		};

		let dir = &self.ant.dir;

		if let Some(cur) = cur {
			self.grids[x][y] = Some((cur + 1) % GRIDS.len() as u8);
			if cur % 2 == 0 {
				self.ant.dir = dir.turn_left();
			} else {
				self.ant.dir = dir.turn_right();
			}
		} else {
			self.grids[x][y] = Some(0);
			self.ant.dir = dir.turn_right();
		};

		let (x, y) = self.ant.dir.forward((self.ant.x, self.ant.y));

		self.ant.x = x;
		self.ant.y = y;
		self.count += 1;

	}

}

impl app::State for Sim {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let x = ctx.width() / GRID_SIZE;
		let y = ctx.height() / GRID_SIZE;
		let ax = x / 2;
		let ay = y / 2;
		let grids = vec![vec![None; y as usize]; x as usize];

		return Ok(Self {
			grids: grids,
			count: 0,
			ant: Ant {
				dir: Dir::Up,
				x: ax,
				y: ay,
			},
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.next();
		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		use shapes::*;

		for (i, row) in self.grids.iter().enumerate() {
			for (j, g) in row.iter().enumerate() {
				if let Some(c) = g {
					ctx.draw(
						rect(vec2!(i as i32 * GRID_SIZE, j as i32 * GRID_SIZE), vec2!((i + 1) as i32 * GRID_SIZE, (j + 1) as i32 * GRID_SIZE))
							.fill(GRIDS[*c as usize])
					)?;
				}
			}
		}

		ctx.push(&gfx::t()
			.translate(vec2!(16))
		, |ctx| {
			ctx.draw(
				shapes::text(&format!("{}", self.count))
					.align(gfx::Origin::TopLeft)
			)?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::launcher()
		.vsync(false)
		.fps_cap(None)
		.origin(gfx::Origin::TopLeft)
		.run::<Sim>() {
		println!("{}", err);
	}
}

