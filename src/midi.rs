// wengwengweng

//! MIDI Types

// https://ccrma.stanford.edu/~craig/articles/linuxmidi/misc/essenmidi.html

use std::sync::mpsc;
use std::thread;
use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
	NoteOn(i32, f32),
	NoteOff(i32, f32),
	Control(i32, f32),
	Pitch(f32, f32),
	Unknown(Vec<u8>),
}

// TODO: wait for Option::zip to be stable
fn option_zip<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
	return match (a, b) {
		(Some(a), Some(b)) => Some((a, b)),
		_ => None,
	};
}

impl Msg {

	pub fn from(msg: &[u8]) -> Msg {

		let first = match msg.get(0) {
			Some(b) => b,
			None => return Msg::Unknown(msg.to_vec()),
		};

		let second = msg.get(1);
		let third = msg.get(2);

		match first {
			// note off
			0x80..=0x8f => {
				if let Some((note, vel)) = option_zip(second, third) {
					return Msg::NoteOff(*note as i32, *vel as f32 / 127.0);
				}
			},
			// note on
			0x90..=0x9f => {
				if let Some((note, vel)) = option_zip(second, third) {
					return Msg::NoteOn(*note as i32, *vel as f32 / 127.0);
				}
			},
			// continuous
			0xb0..=0xbf => {
				if let Some((id, val)) = option_zip(second, third) {
					return Msg::Control(*id as i32, *val as f32 / 127.0);
				}
			},
			// pitch
			0xe0..=0xef => {
				if let Some((lsb, msb)) = option_zip(second, third) {
					return Msg::Pitch(*lsb as f32 / 127.0, *msb as f32 / 127.0);
				}
			},
			_ => {},
		}

		return Msg::Unknown(msg.to_vec());

	}

}

pub(crate) fn listen() -> Result<mpsc::Receiver<Msg>> {

	let (midi_tx, midi_rx) = mpsc::channel();

	thread::Builder::new()
		.name(format!("dirty_midi"))
		.spawn(move || {

		// TODO: extremely slow
		if let Ok(midi_in) = midir::MidiInput::new("dirty_midi") {

			if let Some(port) = midi_in.ports().last() {

				let port_name = midi_in.port_name(port).unwrap_or(format!("unknown"));

				let _conn = midi_in.connect(port, &format!("dirty_midi - {}", port_name), move |_, msg, _| {
					if let Err(e) = midi_tx.send(Msg::from(&msg)) {
						elog!("failed to send midi msg");
					}
				}, ()).map_err(|_| format!("failed to read midi input"));

				loop {}

			}

		} else {
			elog!("failed to init midi input");
		}

	}).map_err(|_| format!("failed to spawn midi thread"))?;

	return Ok(midi_rx);

}

