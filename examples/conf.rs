// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game;

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.scale(vec2!(12));
		ctx.draw(shapes::text("yo"))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.size(640, 480)
		.title("")
		.hidpi(true)
		.resizable(false)
		.fullscreen(false)
		.vsync(true)
		.cursor_hidden(false)
		.cursor_locked(false)
		.hide_title(false)
		.hide_titlebar_buttons(false)
		.transparent(false)
		.always_on_top(false)
		.fps_cap(Some(60))
		.origin(gfx::Origin::Center)
		.quad_origin(gfx::Origin::Center)
		.texture_filter(gfx::FilterMode::Nearest)
		.run::<Game>() {
		println!("{}", err);
	}
}

