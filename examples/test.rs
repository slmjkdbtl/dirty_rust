use dirty::*;

fn main() {

	let mut a = window::Window::new(window::Conf {
		title: "test",
		width: 120,
		height: 120,
		.. Default::default()
	});

	a.run(|ctx| {
		println!("{}", ctx.time());
		if ctx.time() >= 3.0 {
			ctx.close();
		}
	});

}
