// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Game {
	model: gfx::Model,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			model: gfx::Model::from_obj(ctx, include_str!("res/cow.obj"))?,
		});

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.translate3d(vec3!(0, 0, 80));
		ctx.rotate_y(ctx.time());
		ctx.scale3d(vec3!(7, 7, 7));
		ctx.draw(shapes::model(&self.model))?;
		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

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
		.run::<Game>() {
		println!("{}", err);
	}

}

