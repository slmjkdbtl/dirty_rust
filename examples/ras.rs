// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

fn viewport(p: Vec3, w: f32, h: f32) -> Mat4 {

	return mat4!(
		w / 2.0, 0.0, 0.0, p.x + w / 2.0,
		0.0, h / 2.0, 0.0, p.y + h / 2.0,
		0.0, 0.0, p.z / 2.0, p.z / 2.0,
		0.0, 0.0, 0.0, 1.0,
	);

}

struct Game {
	ok: gfx::Mesh,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			ok: gfx::Mesh::from_obj(ctx, include_str!("res/ok.obj"), None)?,
			cam: gfx::PerspectiveCam::new(60.0, ctx.width() as f32 / ctx.height() as f32, 0.1, 1024.0, vec3!(0, 0, -12.0), 0.0, 0.0),
			move_speed: 9.0,
			eye_speed: 0.16,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
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

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		use shapes::*;
		use gfx::Camera;

		let proj = self.cam.projection();
		let view = self.cam.lookat();
		let model = Mat4::identity()
// 			* Mat4::scale(vec3!(720))
			;

		for mesh in self.ok.meshdata() {

			for tri in mesh.indices.chunks(3) {

				let p1 = mesh.vertices[tri[0] as usize].pos;
				let p2 = mesh.vertices[tri[1] as usize].pos;
				let p3 = mesh.vertices[tri[2] as usize].pos;

				let p1 = proj * view * model * p1;
				let p2 = proj * view * model * p2;
				let p3 = proj * view * model * p3;

				let p1 = (p1 / p1.z * 0.5).xy() * vec2!(ctx.width(), -ctx.height());
				let p2 = (p2 / p2.z * 0.5).xy() * vec2!(ctx.width(), -ctx.height());
				let p3 = (p3 / p3.z * 0.5).xy() * vec2!(ctx.width(), -ctx.height());

				ctx.draw(&line(p1, p2))?;
				ctx.draw(&line(p2, p3))?;
				ctx.draw(&line(p3, p1))?;

			}

		}

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

