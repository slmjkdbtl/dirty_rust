// wengwengweng

use dirty::*;
use app::*;
use input::Key;
use gfx::Origin;

use kit::ui::*;

struct Game {
	ui: UI,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			ui: UI::new(),
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		self.ui.event(e);

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
		return Ok(());
	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&shapes::checkerboard(ctx.coord(Origin::BottomLeft), ctx.coord(Origin::TopRight), 32.0))?;

		self.ui.frame(ctx, |ui| {
			ui.panel(0, "test", vec2!(0), 240.0, 320.0, |p| {
				let s = p.input(1);
				p.menu(2, |m| {
					m.item(3, "1");
					m.item(4, "2");
					if m.item(5, "3") {
						println!("ok");
					}
				});
			});
		});

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
		.default_font(res::font::UNSCII)
		.resizable(true)
		.run::<Game>();
}

