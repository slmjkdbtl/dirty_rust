// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	font: gfx::TTF,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {
		return Ok(Self {
			font: gfx::TTF::new(ctx, include_bytes!("res/Zpix.ttf"))?,
		});
	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

// 		ctx.draw(shapes::text("yo"))?;
		ctx.scale(vec2!(20));
		self.font.draw(ctx, "请问企鹅我");

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


