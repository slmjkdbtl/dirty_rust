// wengwengweng

use serde::Serialize;
use serde::Deserialize;
use dirty::*;
use gfx::shapes;
use input::Key;

const PROJ: &str = "dirty_data_example";
const ENTRY: &str = "counter";

#[derive(Default, Serialize, Deserialize)]
struct Data {
	count: usize,
}

struct Game {
	data: Data,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {

		let data = data::load::<Data>(PROJ, ENTRY);

		if data.is_err() {
			data::save(PROJ, ENTRY, &Data::default())?;
		}

		let data = data::load::<Data>(PROJ, ENTRY)?;

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
						data::save(PROJ, ENTRY, &self.data)?;
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
		.run::<Game>() {
		elog!("{}", e);
	}
}

