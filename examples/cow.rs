// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Game {
	model: gfx::Model,
	pix_shader: gfx::Shader2D,
	canvas: gfx::Canvas,
	cam: gfx::Camera,
	move_speed: f32,
	eye_speed: f32,
}

struct PixUniform {
	resolution: Vec2,
	size: f32,
}

impl gfx::Uniform for PixUniform {
	fn values(&self) -> gfx::UniformValues {
		return vec![
			("resolution", gfx::UniformType::F2(self.resolution.as_arr())),
			("size", gfx::UniformType::F1(self.size)),
		];
	}
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			model: gfx::Model::from_obj(ctx, include_str!("res/cow.obj"))?,
			pix_shader: gfx::Shader2D::effect(ctx, include_str!("res/pix.frag"))?,
			canvas: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
			cam: gfx::Camera::new(vec3!(0, 0, -12), 0.0, 0.0),
			move_speed: 16.0,
			eye_speed: 0.16,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.toggle_cursor_hidden();
					ctx.toggle_cursor_locked()?;
				}
				if *k == Key::F {
					ctx.toggle_fullscreen();
				}
			},

			MouseMove(delta) => {

				if ctx.is_cursor_locked() {

					let md: Vec2 = (*delta).into();
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

		if ctx.key_down(Key::W) {
			self.cam.set_pos(self.cam.pos() + self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::S) {
			self.cam.set_pos(self.cam.pos() - self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::A) {
			self.cam.set_pos(self.cam.pos() + self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::D) {
			self.cam.set_pos(self.cam.pos() - self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_on(&self.canvas, |ctx| {

			ctx.clear_ex(gfx::Surface::Depth);

			ctx.use_cam(&self.cam, |ctx| {

				ctx.push(&gfx::t()
					.rotate_y(ctx.time())
				, |ctx| {
					ctx.draw(shapes::model(&self.model))?;
					return Ok(());
				})?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		ctx.draw_2d_with(&self.pix_shader, &PixUniform {
			resolution: vec2!(ctx.width(), ctx.height()),
			size: 6.0,
		}, |ctx| {
			return ctx.draw(shapes::canvas(&self.canvas));
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

