// wengwengweng

use dirty::*;
use math::*;
use gfx::shapes;
use input::Key;

const LOAD_COUNT: usize = 120;
const SCALE: f32 = 9.0;

struct Teapot {
	transform: Mat4,
	model: gfx::Model,
}

struct Game {
	tasks: Vec<task::Loader<Result<gfx::ModelData>>>,
	loaded: usize,
	count: usize,
	teapots: Vec<Teapot>,
	shader: gfx::Shader<()>,
	canvas: gfx::Canvas,
}

impl Game {
	fn load_more(&mut self) -> Result<()> {
		for _ in 0..LOAD_COUNT {
			self.tasks.push(task::Loader::new(|| {
				return gfx::Model::load_obj(&fs::read_str("examples/res/teapot.obj")?, None, None);
			})?);
		}
		self.count += LOAD_COUNT;
		return Ok(());
	}
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let cw = (d.gfx.width() as f32 / SCALE) as i32;
		let ch = (d.gfx.height() as f32 / SCALE) as i32;

		let mut l = Self {
			tasks: vec![],
			teapots: vec![],
			shader: gfx::Shader::from_frag(d.gfx, include_str!("res/blue.frag"))?,
			canvas: gfx::Canvas::new(d.gfx, cw, ch)?,
			loaded: 0,
			count: 0,
		};

		l.load_more()?;

		return Ok(l);

	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			Resize(w, h) => {

				let cw = (*w as f32 / SCALE) as i32;
				let ch = (*h as f32 / SCALE) as i32;

				self.canvas.resize(d.gfx, cw, ch)?;

			},

			KeyPress(k) => {
				match *k {
					Key::F => d.window.toggle_fullscreen(),
					Key::Esc => d.window.quit(),
					Key::Space => self.load_more()?,
					_ => {},
				}
			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		let dt = d.app.dt().as_secs_f32();

		for task in &mut self.tasks {
			if let Some(data) = task.poll() {
				self.loaded += 1;
				self.teapots.push(Teapot {
					transform: mat4!()
						.t3(vec3!(rand(-320, 320), rand(-320, 320), rand(-640, -240)))
						.rx(rand(0f32, 360f32).to_radians())
						.ry(rand(0f32, 360f32).to_radians())
						.rz(rand(0f32, 360f32).to_radians())
						,
					model: gfx::Model::from_data(d.gfx, data?)?,
				});
			}
		}

		self.tasks.retain(|t| !t.done());

		for t in &mut self.teapots {
			t.transform = t.transform
				.rx(dt)
				.ry(dt)
				.rz(dt)
				;
		}

		d.gfx.draw_on(&self.canvas, |gfx| {

			gfx.clear_ex(gfx::Surface::Depth);

			gfx.push_t(mat4!().s3(vec3!(1.0 / SCALE)), |gfx| {

				gfx.draw_with(&self.shader, &(), |gfx| {
					for t in &self.teapots {
						gfx.draw_t(t.transform, &shapes::model(&t.model))?;
					}
					return Ok(());
				})?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw_t(
			mat4!()
				.s2(vec2!(SCALE))
				,
			&shapes::canvas(&self.canvas)
		)?;

		let bot_left = d.gfx.coord(gfx::Origin::BottomLeft);

		d.gfx.draw_t(
			mat4!()
				.t2(bot_left + vec2!(24, 48))
				,
			&shapes::text(
				&format!("{}/{}", self.loaded, self.count)
			)
				.align(gfx::Origin::BottomLeft)
				.size(16.0)
				,
		)?;

		d.gfx.draw_t(
			mat4!()
				.t2(bot_left + vec2!(24, 24))
				,
			&shapes::text("press SPACE to load more")
				.align(gfx::Origin::BottomLeft)
				.size(12.0)
				,
		)?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.run::<Game>() {
		elog!("{}", e);
	}
}

