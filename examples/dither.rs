// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	tex: gfx::Tex2D,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut img = img::Image::from_bytes(include_bytes!("res/dedede.png"))?;

		for y in 0..img.height() {
			for x in 0..img.width() {

				let b = img
					.get(x, y)
					.expect("oh no")
					.brightness();

// 				let b = c.round();

				img.set(x, y, color!(b, b, b, 1))?;

			}

		}

		for y in 0..img.height() {
			for x in 0..img.width() {

				let c = img
					.get(x, y)
					.expect("oh no");

				let nc = color!(c.r.round(), c.g.round(), c.b.round(), 1);

				img.set(x, y, nc)?;

				let err = c - nc;

				let ops = [
					(x + 1, y, err * 7.0/16.0),
					(x - 1, y + 1, err * 3.0/16.0),
					(x, y + 1, err * 5.0/16.0),
					(x + 1, y + 1, err * 1.0/16.0),
				];

				for i in 0..4 {
					if let Some(c) = img.get(ops[i].0, ops[i].1) {
						img.set(ops[i].0, ops[i].1, c + ops[i].2)?;
					}
				}

			}

		}

		return Ok(Self {
			tex: gfx::Tex2D::from_img(ctx, img)?,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Escape {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.scale(vec2!(0.75));
		ctx.draw(shapes::sprite(&self.tex))?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::launcher()
		.size(480, 480)
		.run::<Game>() {
		println!("{}", err);
	}
}


