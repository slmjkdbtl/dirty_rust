// wengwengweng

use dirty::*;
use app::*;
use input::Key;
use gfx::Origin;

use kit::ui::*;

struct Game {
	panel: Panel,
	input: Input,
	select: Select<i32>,
	c1: CheckBox,
	c2: CheckBox,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			panel: Panel::new("test", vec2!(100), 240.0, 320.0),
			input: Input::new("name"),
			select: Select::new(vec![1, 2, 3], 0),
			c1: CheckBox::new("draw bbox"),
			c2: CheckBox::new("profiler"),
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		self.panel.event(ctx, e, &mut [
// 			&self.select,
			&mut self.input,
			&mut self.c1,
			&mut self.c2,
		])?;

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

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {
		self.panel.update(ctx)?;
		return Ok(());
	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&shapes::checkerboard(ctx.coord(Origin::BottomLeft), ctx.coord(Origin::TopRight), 32.0))?;

		self.panel.draw(ctx, &[
// 			&self.select,
			&self.input,
			&self.c1,
			&self.c2,
		])?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
		.default_font(res::font::UNSCII)
		.scale_mode(gfx::ScaleMode::Letterbox)
		.resizable(true)
		.run::<Game>();
}

