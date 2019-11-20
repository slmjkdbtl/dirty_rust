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

		match *e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
				if k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn imgui(&self, ui: &app::imgui::Ui) -> Result<()> {

		use app::imgui::*;

		Window::new(im_str!("yo"))
			.size([320.0, 160.0], Condition::Always)
			.build(&ui, || {
				ui.text(im_str!("Hello world!"));
				ui.text(im_str!("こんにちは世界！"));
				ui.separator();
				let mouse_pos = ui.io().mouse_pos;
				ui.text(format!(
					"Mouse Position: ({:.1},{:.1})",
					mouse_pos[0], mouse_pos[1]
				));
			});

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}


