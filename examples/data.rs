// wengwengweng

use serde::Serialize;
use serde::Deserialize;
use dirty::*;
use gfx::shapes;
use input::Key;

#[derive(Default, Serialize, Deserialize)]
struct Data {
	count: usize,
}

struct Game {
	data: Data,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let data = d.app.get_data::<Data>("counter");

		if data.is_err() {
			d.app.save_data("counter", Data::default())?;
		}

		let data = d.app.get_data::<Data>("counter")?;

		return Ok(Self {
			data: data,
		});

	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::Space => {
						self.data.count += 1;
						d.app.save_data("counter", &self.data)?;
					},
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw(
			&shapes::text(&format!("{}", self.data.count))
				.size(48.0)
		)?;

		let lines = [
			"press [SPACE] to increase",
			"the data will preserve",
		];

		for (i, l) in lines.iter().enumerate() {
			d.gfx.draw_t(
				mat4!()
					.ty(-64.0 - i as f32 * 24.0)
					,
				&shapes::text(l)
					.size(12.0)
					,
			)?;
		}

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.data_path("dirty_data_example")
		.run::<Game>() {
		elog!("{}", e);
	}
}



