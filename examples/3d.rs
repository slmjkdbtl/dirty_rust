// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Game {
	tex: gfx::Texture,
	model: gfx::Model,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			tex: gfx::Texture::from_bytes(ctx, include_bytes!("res/icon.png"))?,
			model: gfx::Model::from_obj(ctx, include_str!("res/plant.obj"))?,
			cam: gfx::PerspectiveCam::new(60.0, ctx.width() as f32 / ctx.height() as f32, 0.1, 1024.0, vec3!(0, 0, -12), 0.0, 0.0),
			move_speed: 16.0,
			eye_speed: 0.16,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {
				if k == Key::Esc {
					ctx.toggle_cursor_hidden();
					ctx.toggle_cursor_locked()?;
// 					ctx.quit();
				}
				if k == Key::F {
					ctx.toggle_fullscreen();
				}
			},

			MouseMove(delta) => {

				if ctx.is_cursor_locked() {

					let md: Vec2 = delta.into();
					let mut rx = self.cam.yaw();
					let mut ry = self.cam.pitch();
					let dead = 48.0f32.to_radians();

					rx -= md.x * self.eye_speed * ctx.dt();
					ry -= md.y * self.eye_speed * ctx.dt();

					if ry > dead {
						ry = dead;
					}

					if ry < -dead {
						ry = -dead;
					}

					self.cam.set_angle(rx, ry);

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if (ctx.key_down(Key::W)) {
			self.cam.set_pos(self.cam.pos() + self.cam.front() * ctx.dt() * self.move_speed);
		}

		if (ctx.key_down(Key::S)) {
			self.cam.set_pos(self.cam.pos() - self.cam.front() * ctx.dt() * self.move_speed);
		}

		if (ctx.key_down(Key::A)) {
			self.cam.set_pos(self.cam.pos() + self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
		}

		if (ctx.key_down(Key::D)) {
			self.cam.set_pos(self.cam.pos() - self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.use_cam(&self.cam, |ctx| {

			ctx.push(&gfx::t()
				.translate_3d(vec3!(30, 0, 0))
				.scale_3d(vec3!(3))
			, |ctx| {
				return ctx.draw(shapes::cube());
			})?;

// 			ctx.push(&gfx::t()
// 			, |ctx| {
// 				return ctx.draw(shapes::sprite3d(&self.tex));
// 			})?;

			ctx.push(&gfx::t()
			, |ctx| {
				return ctx.draw(shapes::model(&self.model));
			})?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.cursor_locked(true)
		.cursor_hidden(true)
		.run::<Game>() {
		println!("{}", err);
	}

}

