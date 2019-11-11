// wengwengweng

use std::f32::consts::PI;
use crate::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Waveform {
	Sine,
	Square,
	Triangle,
	Saw,
	Noise,
}

fn w(f: f32) -> f32 {
	return f * 2.0 * PI;
}

impl Waveform {
	pub fn osc(&self, freq: f32, dt: f32) -> f32 {
		return match self {
			Waveform::Sine => f32::sin(w(freq) * dt),
			Waveform::Square => {
				if Waveform::Sine.osc(freq, dt) > 0.0 {
					1.0
				} else {
					-1.0
				}
			},
			Waveform::Triangle => f32::asin(Waveform::Sine.osc(freq, dt)) * 2.0 / PI,
			Waveform::Saw => (2.0 / PI) * (freq * PI * (dt % (1.0 / freq)) - PI / 2.0),
			Waveform::Noise => {
				if freq == 0.0 {
					0.0
				} else {
					rand!(-1.0, 1.0)
				}
			},
		};
	}
}

pub fn osc(wav: Waveform, freq: f32, dt: f32) -> f32 {
	return wav.osc(freq, dt);
}

