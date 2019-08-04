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

		return Ok(Self {
			model: gfx::Model::from_obj(ctx, include_str!("res/cow.obj"))?,
			pos: vec3!(0, 0, 60),
			rx: 0.0,
			ry: 0.0,
		});

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let speed = 60.0;
		let rot_speed = 0.2;

		ctx.translate3d(vec3!(0, 0, 0));
		ctx.rotate_y(ctx.time());
		ctx.scale3d(vec3!(4, 4, 4));
		ctx.draw(shapes::model(&self.model))?;
		ctx.pos(self.pos);
		ctx.look(self.rx.to_radians(), self.ry.to_radians());
		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		if ctx.key_down(Key::W) {
			self.pos -= vec3!(0, 0, ctx.dt() * speed);
		}

		if ctx.key_down(Key::S) {
			self.pos += vec3!(0, 0, ctx.dt() * speed);
		}

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

