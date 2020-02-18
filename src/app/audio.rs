// wengwengweng

use std::io::Cursor;
use std::time::Duration;

use rodio::Source;
use rodio::Decoder;
use rodio::Sink;
use rodio::source::Buffered;

use super::*;

#[derive(Clone)]
pub struct Sound {
	buffer: Buffered<Decoder<Cursor<Vec<u8>>>>,
	effect: Effect,
}

#[derive(Clone, Copy)]
struct Effect {
	speed: f32,
	volume: f32,
	repeat: bool,
	fadein: f32,
}

impl Default for Effect {
	fn default() -> Self {
		return Self {
			speed: 1.0,
			volume: 1.0,
			repeat: false,
			fadein: 0.0,
		};
	}
}

pub struct Track {
	sink: Sink,
}

impl Sound {

	pub fn from_bytes(data: &[u8]) -> Result<Self> {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor)
			.map_err(|_| format!("failed to parse sound from file"))?;

		return Ok(Self {
			buffer: source.buffered(),
			effect: Effect::default(),
		});

	}

	pub fn as_track(&self, ctx: &Ctx) -> Result<Track> {
		return Track::from_sound(ctx, &self);
	}

	pub fn speed(&self, s: f32) -> Self {
		assert!(s > 0.0 && s <= 2.0, "invalid speed");
		return Self {
			effect: Effect {
				speed: s,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	pub fn volume(&self, v: f32) -> Self {
		assert!(v >= 0.0 && v <= 2.0, "invalid volume");
		return Self {
			effect: Effect {
				volume: v,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	pub fn repeat(&self) -> Self {
		return Self {
			effect: Effect {
				repeat: true,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	pub fn fadein(&self, time: f32) -> Self {
		return Self {
			effect: Effect {
				fadein: time,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	// TODO: clean this up
	fn apply(&self) -> Box<dyn Source<Item = i16> + Send> {

		type S = dyn Source<Item = i16> + Send;

		let s = box self.buffer.clone();
		let effect = self.effect;

		let s: Box<S> = if effect.speed != 0.0 {
			box s.speed(effect.speed)
		} else {
			s
		};

		let s: Box<S> = if effect.volume != 0.0 {
			box s.amplify(effect.volume)
		} else {
			s
		};

		let s: Box<S> = if effect.fadein != 0.0 {
			box s.fade_in(Duration::from_secs_f32(effect.fadein))
		} else {
			s
		};

		let s: Box<S> = if effect.repeat {
			box s.repeat_infinite()
		} else {
			s
		};

		return s;

	}

}

impl Track {

	pub fn from_bytes(ctx: &Ctx, data: &[u8]) -> Result<Self> {
		return Self::from_sound(ctx, &Sound::from_bytes(data)?);
	}

	pub fn from_sound(ctx: &Ctx, sound: &Sound) -> Result<Self> {

		let device = ctx.audio_device.as_ref().ok_or(format!("no audio ouput device"))?;
		let sink = Sink::new(&device);

		sink.append(sound.apply());
		sink.pause();

		return Ok(Self {
			sink,
		});

	}

	pub fn is_playing(&self) -> bool {
		return !self.sink.is_paused();
	}

	pub fn pause(&self) {
		self.sink.pause();
	}

	pub fn play(&self) {
		self.sink.play();
	}

}

pub trait Playable {
	fn play(&self, device: &rodio::Device) -> Result<()>;
}

pub trait Pausible {
	fn pausible(&self) -> Result<()>;
}

impl Playable for Sound {
	fn play(&self, device: &rodio::Device) -> Result<()> {
		return Ok(rodio::play_raw(device, self.apply().convert_samples()));
	}
}

impl Playable for Track {
	fn play(&self, _: &rodio::Device) -> Result<()> {
		return Ok(self.sink.play());
	}
}

impl Ctx {

	pub fn play(&self, thing: &impl Playable) -> Result<()> {

		let device = self.audio_device
			.as_ref()
			.ok_or(format!("no audio output device"))?;

		return thing.play(device);

	}

	pub fn audio_devices(&self) -> Vec<rodio::Device> {
		return rodio::output_devices()
			.map(|c| c.collect())
			.unwrap_or(vec![]);
	}

	pub fn set_audio_device(&mut self, d: rodio::Device) {
		self.audio_device = Some(d);
	}

}

