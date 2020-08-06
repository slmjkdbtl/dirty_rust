// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game {
	canvas: gfx::Canvas,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let model = gfx::Model::from_glb(d.gfx, include_bytes!("res/btfly.glb"))?;
		let canvas = gfx::Canvas::new(d.gfx, 160, 160)?;

		d.gfx.draw_on(&canvas, |gfx| {
			gfx.clear();
			gfx.draw_t(
				mat4!()
					.s3(vec3!(300))
					.rx(0.3)
					.ry(0.3)
					.rz(0.3)
					.t3(-model.center())
					,
				&shapes::model(&model)
			)?;
			return Ok(());
		})?;

		return Ok(Self {
			canvas: canvas,
		});

	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::Space => self.canvas.capture()?.save("test.png")?,
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, d: &mut Ctx) -> Result<()> {

		let gw = d.gfx.width();
		let gh = d.gfx.height();
		let cw = self.canvas.width();
		let ch = self.canvas.height();
		let top_left = d.gfx.coord(gfx::Origin::TopLeft);

		for i in 0..gw / cw {
			for j in 0..gh / ch {
				d.gfx.draw_t(
					mat4!()
						.t2(top_left + vec2!(i, j) * vec2!(cw, -ch)),
					&shapes::canvas(&self.canvas)
						.offset(gfx::Origin::TopLeft.as_pt())
				)?;
			}
		}

		return Ok(());
	}

}

fn main() {
	if let Err(e) = launcher()
		.resizable(true)
		.run::<Game>() {
		elog!("{}", e);
	}
}

