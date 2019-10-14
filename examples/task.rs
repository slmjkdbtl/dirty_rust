// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::img::Image;
use dirty::task::Task;
use input::Key;

struct Game {
	task: Task<Result<Image>>,
	tex: Option<gfx::Texture>,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		let task = Task::exec(|| {
			return Image::from_bytes(&fs::read("examples/res/dedede.png")?);
		});

		return Ok(Self {
			task: task,
			tex: None,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
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

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if self.tex.is_none() {
			if let Some(data) = self.task.poll() {
				self.tex = Some(gfx::Texture::from_img(ctx, data?)?);
			}
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		if let Some(tex) = &self.tex {
			ctx.draw(shapes::sprite(tex))?;
		} else {
			ctx.draw(shapes::text("loading"))?;
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

