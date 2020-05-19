// wengwengweng

use std::rc::Rc;
use std::io::Cursor;
use std::time::Duration;

use rodio::Source;
use rodio::Decoder;
use rodio::Sink;
use rodio::source::Buffered;

use crate::*;

pub struct Audio {
	device: Rc<rodio::Device>,
}

impl Audio {
	pub fn new() -> Result<Self> {
		let device = rodio::default_output_device().ok_or(format!("failed to get audio device"))?;
		return Ok(Self {
			device: Rc::new(device),
		});
	}
}

#[derive(Clone)]
pub struct Sound {
	buffer: Buffered<Decoder<Cursor<Vec<u8>>>>,
	effect: Effect,
	device: Rc<rodio::Device>,
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

impl Sound {

	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor)
			.map_err(|_| format!("failed to parse sound from file"))?;

		return Ok(Self {
			buffer: source.buffered(),
			effect: Effect::default(),
			device: ctx.device.clone(),
		});

	}

	pub fn as_track(self) -> Result<Track> {
		return Track::from_sound(self);
	}

	pub fn speed(&self, s: f32) -> Self {
		return Self {
			effect: Effect {
				speed: s,
				.. self.effect
			},
			device: self.device.clone(),
			buffer: self.buffer.clone(),
		}
	}

	pub fn volume(&self, v: f32) -> Self {
		return Self {
			effect: Effect {
				volume: v,
				.. self.effect
			},
			device: self.device.clone(),
			buffer: self.buffer.clone(),
		}
	}

	pub fn repeat(&self) -> Self {
		return Self {
			effect: Effect {
				repeat: true,
				.. self.effect
			},
			device: self.device.clone(),
			buffer: self.buffer.clone(),
		}
	}

	pub fn fadein(&self, time: f32) -> Self {
		return Self {
			effect: Effect {
				fadein: time,
				.. self.effect
			},
			device: self.device.clone(),
			buffer: self.buffer.clone(),
		}
	}

	pub fn play(&self) -> Result<()> {
		rodio::play_raw(&self.device, self.apply().convert_samples());
		return Ok(());
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

pub struct Track {
	sink: Sink,
}

impl Track {

	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {
		return Self::from_sound(Sound::from_bytes(ctx, data)?);
	}

	pub fn from_sound(sound: Sound) -> Result<Self> {

		let device = rodio::default_output_device().ok_or(format!("failed to get audio device"))?;
		let sink = Sink::new(&device);

		sink.append(sound.buffer);
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

	pub fn free(self) {
		self.sink.stop();
		self.sink.detach();
	}

}

