// wengwengweng

use dirty::*;
use math::*;
use task::TaskPool;
use input::Key;

mod pix;
use pix::*;

const THREAD_COUNT: usize = 1;
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
				return gfx::Model::load_obj(&fs::read_str("examples/res/ok.obj")?, None, None);
			});
		}
	}
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let mut tasks = TaskPool::new(THREAD_COUNT);

		for _ in 0..LOAD_COUNT {
			tasks.exec(|| {
				return gfx::Model::load_obj(&fs::read_str("examples/res/teapot.obj")?, None, None);
			});
		}

		return Ok(Self {
			tasks: tasks,
			teapots: vec![],
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			pix_effect: PixEffect::new(ctx)?,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
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
				transform: gfx::t()
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

		self.pix_effect.render(ctx, |ctx| {

			ctx.clear_ex(gfx::Surface::Depth);

			for t in &self.teapots {
				ctx.push(&t.transform, |ctx| {
					ctx.draw_3d_with(&self.shader, &(), |ctx| {
						ctx.draw(&shapes::model(&t.model))?;
						return Ok(());
					})?;
					return Ok(());
				})?;
			}

			return Ok(());

		})?;

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		self.pix_effect.draw(ctx, &PixUniform {
			resolution: vec2!(ctx.width(), ctx.height()),
			size: 6.0,
		})?;

		ctx.push(&gfx::t()
			.t2(vec2!(32))
			.s2(vec2!(2))
		, |ctx| {
			ctx.draw(
				&shapes::text(&format!("{}/{}", self.tasks.completed(), self.tasks.total())),
			)?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}

}

