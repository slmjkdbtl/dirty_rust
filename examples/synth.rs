// wengwengweng

use std::sync::Arc;
use std::sync::Mutex;
use std::f32::consts::PI;

use dirty::*;
use app::*;
use input::Key;
use input::Mouse;

struct Game {
	a: Arc<Mutex<Synth>>,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		let a = Arc::new(Mutex::new(Synth {
			freq: 0.0,
			waveform: synth::Waveform::Square,
		}));

		synth::run(a.clone());

		return Ok(Self {
			a: a,
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let mut a = self.a.lock().unwrap();
		let keys = ctx.down_keys();

		let offset = 9;

		use Key::*;

		let kk = vec![
			A, W, S, E, D, F, T, G, Y, H, U, J, K,
		];

		a.freq = 0.0;

		for k in keys {
			if let Some(index) = kk.iter().position(|&x| x == k) {
				a.freq = synth::get_note_freq(index as i32 - offset);
			}
		}

		return Ok(());
	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_t(&gfx::t()
			.t3(vec3!(0, 0, -6))
			.ry(ctx.time())
			.rz(ctx.time())
		, &shapes::cube())?;

		return Ok(());

	}

}

struct Synth {
	freq: f32,
	waveform: synth::Waveform,
}

use audio::synth;

impl synth::Stream for Synth {

	fn data(&self, dt: f32) -> f32 {

		use synth::Waveform;

		match self.waveform {
			Waveform::Sine => {
				return f32::sin(self.freq * 2.0 * PI * dt);
			},
			Waveform::Square => {
				let o = f32::sin(self.freq * 2.0 * PI * dt);
				if o >= 0.0 {
					return 0.2;
				} else {
					return -0.2;
				}
			},
			_ => return 0.0,
		}

	}

}

fn main() -> Result<()> {
	return app::run::<Game>();
}

