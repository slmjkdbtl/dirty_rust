// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Game {
	tex: gfx::Texture,
	tex2: gfx::Texture,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			tex: gfx::Texture::from_bytes(ctx, include_bytes!("res/blob.png"))?,
			tex2: gfx::Texture::from_bytes(ctx, include_bytes!("res/gradient.png"))?,
		});

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_masked(|ctx| {
			ctx.draw(shapes::sprite(&self.tex))?;
			return Ok(());
		}, |ctx| {
			ctx.draw(shapes::sprite(&self.tex2))?;
			return Ok(());
		})?;

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


