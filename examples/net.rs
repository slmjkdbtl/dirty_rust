// wengwengweng

use std::thread;
use std::time::Duration;

use dirty::*;
use net::*;

const HOST: &str = "0.0.0.0";

fn main() {

	thread::spawn(move || {

		let bob = Client::new((HOST, 1111)).expect("failed to bind socket");

		loop {
			bob.send((HOST, 1234), b"I'm Bob").expect("failed to send");
			thread::sleep(Duration::from_secs_f32(1.0));
		}

	});

	thread::spawn(move || {

		let alice = Client::new((HOST, 2222)).expect("failed to bind socket");

		loop {
			alice.send((HOST, 1234), b"I'm Alice").expect("failed to send");
			thread::sleep(Duration::from_secs_f32(2.0));
		}

	});

	thread::spawn(move || {

		let mut client = Client::new((HOST, 1234)).expect("failed to bind socket");

		loop {
			match client.recv() {
				Ok((buf, src)) => {
					println!(r#"{}: "{}""#, src, String::from_utf8_lossy(&buf));
					client.send(src, b"oh hi").expect("failed to send");
				},
				Err(e) => {
					eprintln!("{}", e);
				}
			}
		}

	});

	loop {}

}

