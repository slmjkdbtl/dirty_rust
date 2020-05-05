// wengwengweng

use dirty::*;
use math::*;
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
	fn load_more(&mut self) {
		for _ in 0..LOAD_COUNT {
			self.tasks.exec(|| {
				return gfx::Model::load_obj(&fs::read_str("examples/res/teapot.obj")?, None, None);
			});
		}
	}
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let mut tasks = TaskQueue::new(THREAD_COUNT);

		for _ in 0..LOAD_COUNT {
			tasks.exec(|| {
				return gfx::Model::load_obj(&fs::read_str("examples/res/teapot.obj")?, None, None);
			});
		}

		let cw = (ctx.width() as f32 / SCALE) as i32;
		let ch = (ctx.height() as f32 / SCALE) as i32;

		return Ok(Self {
			tasks: tasks,
			teapots: vec![],
			shader: gfx::Shader::from_frag(ctx, include_str!("res/blue.frag"))?,
			canvas: gfx::Canvas::new(ctx, cw, ch)?,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			Resize(w, h) => {

				let cw = (*w as f32 / SCALE) as i32;
				let ch = (*h as f32 / SCALE) as i32;

				self.canvas.resize(ctx, cw, ch)?;

			},

			KeyPress(k) => {
				match *k {
					Key::F => ctx.toggle_fullscreen(),
					Key::Esc => ctx.quit(),
					Key::Space => self.load_more(),
					_ => {},
				}
			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		for m in self.tasks.poll() {
			let modeldata = m?;
			self.teapots.push(Teapot {
				transform: mat4!()
					.t3(vec3!(rand(-320, 320), rand(-320, 320), rand(-640, -240)))
					.rx(rand(0f32, 360f32).to_radians())
					.ry(rand(0f32, 360f32).to_radians())
					.rz(rand(0f32, 360f32).to_radians())
					,
				model: gfx::Model::from_data(ctx, modeldata)?,
			});
		}

		for t in &mut self.teapots {
			t.transform = t.transform
				.rx(ctx.dt())
				.ry(ctx.dt())
				.rz(ctx.dt())
				;
		}

		ctx.draw_on(&self.canvas, |ctx| {

			ctx.clear_ex(gfx::Surface::Depth);

			ctx.draw_with(&self.shader, &(), |ctx| {
				for t in &self.teapots {
					ctx.draw_t(t.transform, &shapes::model(&t.model))?;
				}
				return Ok(());
			})?;

			return Ok(());

		})?;

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_t(
			mat4!()
				.s2(vec2!(SCALE))
				,
			&shapes::canvas(&self.canvas)
		)?;

		ctx.draw_t(
			mat4!()
				.t2(ctx.coord(gfx::Origin::TopLeft) + vec2!(24, -24))
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

