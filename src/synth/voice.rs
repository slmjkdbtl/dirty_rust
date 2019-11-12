// wengwengweng

use super::*;

#[derive(Clone, Debug)]
pub struct Voice {
	pub(super) life: Life,
	pub(super) waveform: Waveform,
	pub(super) note: Note,
	pub(super) volume: f32,
}

impl Voice {

	pub(super) fn builder(note: Note) -> VoiceBuilder {
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
		return self.life.amp() * self.waveform.osc(self.note.to_freq() as f32, time) * self.volume;
	}

	pub(super) fn dead(&self) -> bool {
		return self.life.dead();
	}

	pub(super) fn release(&mut self) {
		self.life.release();
	}

}

#[derive(Clone, Copy, Debug)]
pub struct VoiceBuilder {
	note: Note,
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

