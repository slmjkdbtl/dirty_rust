// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::task::TaskPool;
use input::Key;

mod pix;
use pix::*;

const THREAD_COUNT: u32 = 1;
const LOAD_COUNT: usize = 120;

struct Teapot {
	transform: gfx::Transform,
	model: gfx::Model,
}

struct Game {
	tasks: TaskPool<Result<gfx::ModelData>>,
	teapots: Vec<Teapot>,
	shader: gfx::Shader3D<()>,
	pix_effect: PixEffect,
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

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut tasks = TaskPool::new(THREAD_COUNT);

		for _ in 0..LOAD_COUNT {
			tasks.exec(|| {
				return gfx::Model::prepare_obj(&fs::read_str("examples/res/teapot.obj")?);
			});
		}

		return Ok(Self {
			tasks: tasks,
			teapots: vec![],
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			pix_effect: PixEffect::new(ctx)?,
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
					.rotate_x(rand!(0f32, 360f32).to_radians())
					.rotate_y(rand!(0f32, 360f32).to_radians())
					.rotate_z(rand!(0f32, 360f32).to_radians())
					,
				model: gfx::Model::from(ctx, model)?,
			});
		}

		for t in &mut self.teapots {
			t.transform = t.transform
				.rotate_x(ctx.dt())
				.rotate_y(ctx.dt())
				.rotate_z(ctx.dt())
				;
		}

		self.pix_effect.render(ctx, |ctx| {

			ctx.clear_ex(gfx::Surface::Depth);

			for t in &self.teapots {
				ctx.push(&t.transform, |ctx| {
					ctx.draw_3d_with(&self.shader, &(), |ctx| {
						ctx.draw(shapes::model(&t.model))?;
						return Ok(());
					})?;
					return Ok(());
				})?;
			}

			return Ok(());

		})?;

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.pix_effect.draw(ctx, &PixUniform {
			resolution: vec2!(ctx.width(), ctx.height()),
			size: 6.0,
		})?;

		ctx.push(&gfx::t()
			.translate(vec2!(32))
			.scale(vec2!(2))
		, |ctx| {
			ctx.draw(
				shapes::text(&format!("{}/{}", self.tasks.completed(), self.tasks.total())),
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

