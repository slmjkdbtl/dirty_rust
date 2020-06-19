// wengwengweng

use dirty::*;
use input::Key;

struct Game {
	imgui: imgui::Imgui,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			imgui: imgui::Imgui::new(d)?,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		self.imgui.event(d, e);

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		use imgui::*;

		self.imgui.render(d, |ui| {

			Window::new(im_str!("test"))
				.size([320.0, 240.0], Condition::FirstUseEver)
				.build(&ui, || {
					ui.text(im_str!("yo"));
				});

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = run::<Game>() {
		elog!("{}", e);
	}
}

