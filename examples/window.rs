// wengwengweng

use dirty::*;
use window::Key;

struct Game {
	tex: gfx::Texture,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {
		return Ok(Self {
			tex: gfx::Texture::from_bytes(ctx, include_bytes!("res/car.png"))?,
		});
	}

	fn run(&mut self, ctx: &mut app::Ctx, dt: f32) -> Result<()> {

		if window::key_pressed(ctx, Key::F) {
			window::toggle_fullscreen(ctx);
		}

		if window::key_pressed(ctx, Key::Escape) {
			app::quit(ctx);
		}

		for _ in 0..10000 {
			gfx::draw(ctx, &self.tex, vec2!(window::width(ctx) / 2, window::height(ctx) / 2), 0.0, vec2!(1), rect!(0, 0, 0.25, 1), color!());
		}
		dbg!(gfx::draw_calls(ctx));
		dbg!(app::fps(ctx));

		return Ok(());

	}

}

fn main() {

	let result = app::run::<Game>(app::Conf {
		clear_color: color!(0, 0, 1, 1),
		..Default::default()
	});

	if let Err(err) = result {
		println!("{}", err);
	}

}

