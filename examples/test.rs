// wengwengweng

use dirty::*;
use app::*;
use input::Key;

use kit::ui::*;

struct Game {
	panel: Panel,
	input: Input,
	select: Select<i32>,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			panel: Panel::new("test", vec2!(100), 240.0, 320.0),
			input: Input::new(),
			select: Select::new(vec![1, 2, 3], 0),
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				let mods = ctx.key_mods();
				match *k {
					Key::F if mods.meta => ctx.toggle_fullscreen(),
					Key::Q if mods.meta => ctx.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&shapes::checkerboard(ctx.gwidth(), ctx.gheight(), 32.0))?;
		self.panel.draw(ctx, &[
			&self.select,
			&self.input,
		])?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
		.default_font(res::font::PROGGY)
		.scale_mode(gfx::ScaleMode::Letterbox)
		.resizable(true)
		.run::<Game>();
}

