// wengwengweng

use dirty::*;

fn main() {

	let mut task = thread::exec(|| {
		return fs::read_str("test");
	});

	loop {
		if let Some(t) = task.data() {
			if let Ok(data) = t {
				println!("{}", &data[0..100]);
			}
		}
		std::thread::sleep(std::time::Duration::from_millis(200));
	}

// 	let mut win = window::Window::default();
// 	let mut ready = false;

// 	win.run(|ctx| {

// 		if !rx.done() {
// 			if let Some(thing) = rx.poll() {
// 				if let Ok(st) = thing {
// 					println!("{}", &st[0..100]);
// 				}
// 			} else {
// 				println!("{}", ctx.time());
// 			}
// 		}

// 	});

}
