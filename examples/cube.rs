// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game;

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.push();
		ctx.translate_3d(vec3!(0, 0, 3));
		ctx.rotate_y(ctx.time());
		ctx.rotate_z(ctx.time());
		ctx.draw(shapes::cube())?;
		ctx.pop()?;

		ctx.draw(shapes::text("yo"))?;

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::run::<Game>() {
		println!("{}", err);
	}
}

