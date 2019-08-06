// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Game {
	model: gfx::Model,
	pos: Vec3,
	rx: f32,
	ry: f32,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		ctx.cam_pos(vec3!(0, 0, -60));

		return Ok(Self {
			model: gfx::Model::from_obj(ctx, include_str!("res/cow.obj"))?,
			pos: vec3!(0, 0, -60),
			rx: 0.0,
			ry: 0.0,
		});

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let move_speed = 60.0;
		let rot_speed = 0.15;

		ctx.translate3d(vec3!(0, 0, 0));
		ctx.rotate3d(ctx.time(), vec3!(0, 1, 0));
		ctx.scale3d(vec3!(4, 4, 4));
		ctx.draw(shapes::model(&self.model).color(color!(0, 0, 1, 1)))?;
		ctx.cam_pos(self.pos);
		ctx.cam_look(self.rx.to_radians(), self.ry.to_radians());
		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		if let Some(delta) = ctx.mouse_delta() {

			let md: Vec2 = delta.into();

			self.rx -= md.x * rot_speed;
			self.ry -= md.y * rot_speed;

			if self.ry > 48.0 {
				self.ry = 48.0;
			}

			if self.ry < -48.0 {
				self.ry = -48.0;
			}

		}

		if ctx.key_down(Key::W) {
			self.pos += ctx.cam_front() * ctx.dt() * move_speed;
		}

		if ctx.key_down(Key::S) {
			self.pos -= ctx.cam_front() * ctx.dt() * move_speed;
		}

		if ctx.key_down(Key::A) {
			self.pos += ctx.cam_front().cross(vec3!(0, 1, 0)).unit() * ctx.dt() * move_speed;
		}

		if ctx.key_down(Key::D) {
			self.pos -= ctx.cam_front().cross(vec3!(0, 1, 0)).unit() * ctx.dt() * move_speed;
		}

		if ctx.key_pressed(Key::F) {
			ctx.toggle_fullscreen();
		}

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
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

