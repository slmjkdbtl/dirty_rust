// wengwengweng

use dirty::*;
use gfx::*;
use input::*;

const GENDERS: [&str; 56] = [
	"Agender",
	"Androgyne",
	"Androgynous",
	"Bigender",
	"Cis",
	"Cisgender",
	"Cis Female",
	"Cis Male",
	"Cis Man",
	"Cis Woman",
	"Cisgender Female",
	"Cisgender Male",
	"Cisgender Man",
	"Cisgender Woman",
	"Female to Male",
	"FTM",
	"Gender Fluid",
	"Gender Nonconforming",
	"Gender Questioning",
	"Gender Variant",
	"Genderqueer",
	"Intersex",
	"Male to Female",
	"MTF",
	"Neither",
	"Neutrois",
	"Non-binary",
	"Other",
	"Pangender",
	"Trans",
	"Trans*",
	"Trans Female",
	"Trans* Female",
	"Trans Male",
	"Trans* Male",
	"Trans Man",
	"Trans* Man",
	"Trans Person",
	"Trans* Person",
	"Trans Woman",
	"Trans* Woman",
	"Transfeminine",
	"Transgender",
	"Transgender Female",
	"Transgender Male",
	"Transgender Man",
	"Transgender Person",
	"Transgender Woman",
	"Transmasculine",
	"Transsexual",
	"Transsexual Female",
	"Transsexual Male",
	"Transsexual Man",
	"Transsexual Person",
	"Transsexual Woman",
	"Two-Spirit",
];

struct Game {
	ui: ui::UI,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			ui: ui::UI::new(d)?,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &Event) -> Result<()> {

		use Event::*;

		self.ui.event(d, &e);

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		let top_left = d.gfx.coord(gfx::Origin::TopLeft);

		self.ui.frame(d, |mut m| {

			m.window("test", top_left + vec2!(64, -64), 240.0, 320.0, |mut p| {

				p.text("yo")?;
				p.input("name")?;
				p.slider::<i32>("age", 18, 0, 100)?;
				p.select("gender", &GENDERS, 0)?;
				p.checkbox("dead", false)?;
				p.color("color", rgba!(0, 0, 0, 1))?;
				p.sep()?;
				p.button("explode")?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw(&shapes::canvas(self.ui.canvas()))?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.run::<Game>() {
		elog!("{}", e);
	}
}

