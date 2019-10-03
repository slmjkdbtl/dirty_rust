// wengwengweng

use std::thread;

use dirty::*;
use dirty::app::*;
use dirty::task::Task;
use input::Key;

struct Game {
	task: Task<Result<Vec<u8>>>,
	tex: Option<gfx::Texture>,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			task: task!(fs::read("examples/res/dedede.png")),
			tex: None,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
				if *k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if self.tex.is_none() {
			if let Some(data) = self.task.poll() {
				self.tex = Some(gfx::Texture::from_bytes(ctx, &data?)?);
			}
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		if let Some(tex) = &self.tex {
			ctx.draw(&shapes::sprite(tex))?;
		} else {
			ctx.draw(&shapes::text("loading"))?;
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

