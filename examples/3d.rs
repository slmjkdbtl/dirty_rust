// wengwengweng

use dirty::*;
use math::*;
use input::Key;
use gfx::Camera;

struct Game {
	model: gfx::Model,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let model = gfx::Model::from_glb(
			ctx,
			include_bytes!("res/btfly.glb"),
		)?;

		let model = gfx::Model::from_obj(
			ctx,
			include_str!("res/truck.obj"),
			Some(include_str!("res/truck.mtl")),
			None,
		)?;

		return Ok(Self {
			model: model,
			cam: gfx::PerspectiveCam::new(60f32.to_radians(), ctx.width() as f32 / ctx.height() as f32, 0.1, 1024.0, vec3!(3, 3, 2), -0.92, -0.56),
			move_speed: 12.0,
			eye_speed: 0.16,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {
				let mods = ctx.key_mods();
				match *k {
					Key::Esc => {
						ctx.toggle_cursor_hidden();
						ctx.toggle_cursor_locked()?;
					},
					Key::F => ctx.toggle_fullscreen(),
					Key::Q if mods.meta => ctx.quit(),
					_ => {},
				}
			},

			MouseMove(delta) => {

				if ctx.is_cursor_hidden() {

					let mut rx = self.cam.yaw();
					let mut ry = self.cam.pitch();
					let dead = 48.0f32.to_radians();

					rx += delta.x * self.eye_speed * ctx.dt();
					ry -= delta.y * self.eye_speed * ctx.dt();

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

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		if ctx.key_down(Key::W) {
			self.cam.set_pos(self.cam.pos() + self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::S) {
			self.cam.set_pos(self.cam.pos() - self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::A) {
			self.cam.set_pos(self.cam.pos() - self.cam.front().cross(vec3!(0, 1, 0)).unit() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::D) {
			self.cam.set_pos(self.cam.pos() + self.cam.front().cross(vec3!(0, 1, 0)).unit() * ctx.dt() * self.move_speed);
		}

		ctx.set_title(&format!("{}", ctx.fps()));

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let p = vec3!(0);
		let p2 = self.cam.to_screen(ctx, p);

		ctx.use_cam(&self.cam, |ctx| {
			ctx.draw(&shapes::model(&self.model))?;
			return Ok(());
		})?;

		ctx.draw(&shapes::circle(p2, 12.0))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
// 		.cursor_relative(true)
		.cursor_hidden(true)
		.cursor_locked(true)
		.resizable(true)
		.run::<Game>() {
		println!("{}", err);
	}

}



