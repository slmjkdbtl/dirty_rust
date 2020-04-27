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
			cam: gfx::PerspectiveCam::new(
				60f32.to_radians(),
				ctx.width() as f32 / ctx.height() as f32,
				0.1,
				1024.0,
				vec3!(0, 2, 6),
				0.0,
				0.0,
			),
			move_speed: 12.0,
			eye_speed: 0.16,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			Resize(w, h) => {
// 				self.cam = gfx::PerspectiveCam::new(
// 					60f32.to_radians(),
// 					ctx.width() as f32 / ctx.height() as f32,
// 					0.1,
// 					1024.0,
// 					self.cam.pos(),
// 					-0.92,
// 					-0.56
// 				);
			},

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
					let dead = f32::to_radians(60.0);

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

		let dt = ctx.dt();

		if ctx.key_down(Key::W) {
			self.cam.set_pos(self.cam.pos() + self.cam.front() * dt * self.move_speed);
		}

		if ctx.key_down(Key::S) {
			self.cam.set_pos(self.cam.pos() - self.cam.front() * dt * self.move_speed);
		}

		if ctx.key_down(Key::A) {
			self.cam.set_pos(self.cam.pos() - self.cam.front().cross(vec3!(0, 1, 0)).unit() * dt * self.move_speed);
		}

		if ctx.key_down(Key::D) {
			self.cam.set_pos(self.cam.pos() + self.cam.front().cross(vec3!(0, 1, 0)).unit() * dt * self.move_speed);
		}

		ctx.set_title(&format!("{}", ctx.fps()));

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let p = vec3!(0);
		let origin = self.cam.to_screen(ctx, p);
		let mray = self.cam.mouse_ray(ctx);

		ctx.use_cam(&self.cam, |ctx| {

			let bbox = self.model.bbox().transform(mat4!());

			let cray = Ray3::new(self.cam.pos(), self.cam.front());

			let c = if kit::geom::intersect3d(mray, bbox) {
				rgba!(0, 0, 1, 1)
			} else {
				rgba!(1)
			};

			ctx.draw(&shapes::model(&self.model))?;

			ctx.draw(
				&shapes::Rect3D::from_bbox(bbox)
					.line_width(3.0)
					.color(c)
			)?;

// 			let ground = Plane::new(vec3!(0, 1, 0), 0.0);

// 			if let Some(pt) = kit::geom::ray_plane(mray, ground) {
// 				ctx.draw_t(mat4!().t3(pt), &shapes::cube())?;
// 			}

			return Ok(());

		})?;

		ctx.draw(&shapes::circle(vec2!(0), 2.0))?;

		ctx.draw_t(
			mat4!()
				.t2(origin)
				,
			&shapes::text("car")
				.size(16.0)
				,
		)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.cursor_hidden(true)
		.cursor_locked(true)
		.resizable(true)
		.run::<Game>() {
		println!("{}", err);
	}

}

