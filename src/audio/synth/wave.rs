// wengwengweng

use std::f32::consts::PI;
use super::*;

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
	pub fn osc(&self, freq: f32, t: f32) -> f32 {
		return match self {
			Waveform::Sine => f32::sin(w(freq) * t),
			Waveform::Square => {
				if Waveform::Sine.osc(freq, t) > 0.0 {
					1.0
				} else {
					-1.0
				}
			},
			Waveform::Triangle => f32::asin(Waveform::Sine.osc(freq, t)) * 2.0 / PI,
			Waveform::Saw => (2.0 / PI) * (freq * PI * (t % (1.0 / freq)) - PI / 2.0),
			Waveform::Noise => {
				if freq == 0.0 {
					0.0
				} else {
					math::rand(-1.0, 1.0)
				}
			},
		};
	}
}

pub fn osc(wav: Waveform, freq: f32, t: f32) -> f32 {
	return wav.osc(freq, t);
}

