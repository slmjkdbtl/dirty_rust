// wengwengweng

use dirty::*;
use app::*;
use kit::*;
use particle::*;
use math::*;
use input::Key;

struct Game {
	fire: ParticleSystem,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let fire = ParticleSystem::from_conf(ParticleConf {
			offset: (vec2!(-32, -16), vec2!(32, 16)),
			life: (1.0, 3.0),
			color_start: (rgba!(0.9, 0.3, 0, 0.4), rgba!(1, 0.3, 0, 0.5)),
			color_end: rgba!(0.2, 0.2, 1, 0),
			speed: (96.0, 240.0),
			acc: (vec2!(-0.1, -0.5), vec2!(0.1, -0.5)),
			vel: (vec2!(-0.2, -0.5), vec2!(0.2, -0.5)),
			rate: (0.02, 0.05),
			size_start: (vec2!(12), vec2!(36)),
			size_end: (vec2!(0), vec2!(0)),
			num: (16, 24),
			max: 1024,
		});

		return Ok(Self {
			fire: fire,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
				if k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.set_title(&format!("FPS: {} DCS: {} PTC: {}", ctx.fps(), ctx.draw_calls(), self.fire.count()));
		self.fire.update(ctx.dt());
		self.fire.set_pos(ctx.mouse_pos());

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.use_blend(gfx::Blend::Add, |ctx| {
			ctx.draw(&self.fire)?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

