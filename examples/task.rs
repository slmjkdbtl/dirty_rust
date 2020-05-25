// wengwengweng

use dirty::*;
use math::*;
use gfx::shapes;
use task::TaskQueue;
use input::Key;

const THREAD_COUNT: usize = 1;
const LOAD_COUNT: usize = 120;
const SCALE: f32 = 9.0;

struct Teapot {
	transform: Mat4,
	model: gfx::Model,
}

struct Game {
	tasks: TaskQueue<Result<gfx::ModelData>>,
	teapots: Vec<Teapot>,
	shader: gfx::Shader<()>,
	canvas: gfx::Canvas,
}

impl Game {
	fn load_more(&mut self) -> Result<()> {
		for _ in 0..LOAD_COUNT {
			self.tasks.exec(|| {
				return gfx::Model::load_obj(&fs::read_str("examples/res/teapot.obj")?, None, None);
			})?;
		}
		return Ok(());
	}
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let mut tasks = TaskQueue::new(THREAD_COUNT);

		for _ in 0..LOAD_COUNT {
			tasks.exec(|| {
				return gfx::Model::load_obj(&fs::read_str("examples/res/teapot.obj")?, None, None);
			})?;
		}

		let cw = (d.gfx.width() as f32 / SCALE) as i32;
		let ch = (d.gfx.height() as f32 / SCALE) as i32;

		return Ok(Self {
			tasks: tasks,
			teapots: vec![],
			shader: gfx::Shader::from_frag(d.gfx, include_str!("res/blue.frag"))?,
			canvas: gfx::Canvas::new(d.gfx, cw, ch)?,
		});

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

		for m in self.tasks.poll()? {
			let modeldata = m?;
			self.teapots.push(Teapot {
				transform: mat4!()
					.t3(vec3!(rand(-320, 320), rand(-320, 320), rand(-640, -240)))
					.rx(rand(0f32, 360f32).to_radians())
					.ry(rand(0f32, 360f32).to_radians())
					.rz(rand(0f32, 360f32).to_radians())
					,
				model: gfx::Model::from_data(d.gfx, modeldata)?,
			});
		}

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

		d.gfx.draw_t(
			mat4!()
				.t2(d.gfx.coord(gfx::Origin::TopLeft) + vec2!(24, -24))
				,
			&shapes::text(
				&format!("{}/{}", self.tasks.completed_count(), self.tasks.total())
			)
				.align(gfx::Origin::TopLeft)
				.size(16.0)
				,
		)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

