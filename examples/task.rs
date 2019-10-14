// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use dirty::task::TaskPool;
use input::Key;

const THREAD_COUNT: u32 = 1;
const LOAD_COUNT: usize = 120;

struct Teapot {
	transform: gfx::Transform,
	model: gfx::Model,
}

struct Game {
	tasks: TaskPool<Result<gfx::ModelLoad>>,
	teapots: Vec<Teapot>,
}

impl Game {
	fn load_more(&mut self) {
		for _ in 0..LOAD_COUNT {
			self.tasks.exec(|| {
				return gfx::Model::prepare_obj(&fs::read_str("examples/res/teapot.obj")?);
			});
		}
	}
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		let mut tasks = TaskPool::new(THREAD_COUNT);

		for _ in 0..LOAD_COUNT {
			tasks.exec(|| {
				return gfx::Model::prepare_obj(&fs::read_str("examples/res/teapot.obj")?);
			});
		}

		return Ok(Self {
			tasks: tasks,
			teapots: vec![],
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
				if k == Key::Space {
					self.load_more();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		for m in self.tasks.poll() {
			let model = m?;
			self.teapots.push(Teapot {
				transform: gfx::t()
					.translate_3d(vec3!(rand!(-320, 320), rand!(-320, 320), rand!(240, 640)))
					.rotate_x(rand!(0, 360).to_radians())
					.rotate_y(rand!(0, 360).to_radians())
					.rotate_z(rand!(0, 360).to_radians())
					,
				model: gfx::Model::from(ctx, model)?,
			});
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		for t in &self.teapots {
			ctx.push(&t.transform, |ctx| {
				ctx.draw(shapes::model(&t.model))?;
				return Ok(());
			})?;
		}

		ctx.push(&gfx::t()
			.translate(vec2!(32))
			.scale(vec2!(2))
		, |ctx| {
			ctx.draw(
				shapes::text(&format!("{}/{}", self.tasks.completed(), self.tasks.total()))
			)?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}

}

