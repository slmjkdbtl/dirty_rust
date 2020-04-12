// wengwengweng

use super::*;

const A4_FREQ: f32 = 440.0;
const MIDI_A4_NOTE: i32 = 69;

#[derive(Clone, Debug)]
pub struct Voice {
	pub(super) life: Life,
	pub(super) waveform: Waveform,
	pub(super) note: i32,
	pub(super) volume: f32,
}

impl Voice {

	pub fn builder(note: i32) -> VoiceBuilder {
		return VoiceBuilder {
			volume: 1.0,
			note: note,
			waveform: Waveform::Sine,
			envelope: Envelope {
				attack: 0.01,
				decay: 0.01,
				sustain: 1.0,
				release: 1.0,
			},
		};
	}

	pub(super) fn tick(&mut self, dt: f32) {
		self.life.update(dt);
	}

	pub(super) fn voice(&self, time: f32) -> f32 {

		let volume = self.life.volume() * self.volume;
		let wav = self.waveform.osc(note_to_freq(self.note) as f32, time);

		return volume * wav;

	}

	pub(super) fn dead(&self) -> bool {
		return self.life.dead();
	}

	pub(super) fn release(&mut self) {
		self.life.release();
	}

}

fn note_to_freq(note: i32) -> i32 {
	return (A4_FREQ * f32::powi(f32::powf(2.0, 1.0 / 12.0), note - MIDI_A4_NOTE)) as i32;
}

#[derive(Clone, Copy, Debug)]
pub struct VoiceBuilder {
	note: i32,
	envelope: Envelope,
	waveform: Waveform,
	volume: f32,
}

impl VoiceBuilder {

	pub fn envelope(mut self, e: Envelope) -> Self {
		self.envelope = e;
		return self;
	}

	pub fn attack(mut self, a: f32) -> Self {
		self.envelope.attack = a;
		return self;
	}

	pub fn decay(mut self, d: f32) -> Self {
		self.envelope.decay = d;
		return self;
	}

	pub fn sustain(mut self, s: f32) -> Self {
		self.envelope.sustain = s;
		return self;
	}

	pub fn release(mut self, r: f32) -> Self {
		self.envelope.release = r;
		return self;
	}

	pub fn waveform(mut self, w: Waveform) -> Self {
		self.waveform = w;
		return self;
	}

	pub fn volume(mut self, v: f32) -> Self {
		self.volume = v;
		return self;
	}

	pub fn build(self) -> Voice {

		return Voice {
			volume: self.volume,
			note: self.note,
			waveform: self.waveform,
			life: Life::new(self.envelope),
		};

	}

}

