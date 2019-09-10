struct Renderer {
	program: Box<Program<dyn Uniform>>,
}

impl Renderer {
	fn new<U: Uniform>(program: Program<U>) -> Self {
		return Self {
			program: Box::new(program)
		};
	}
	fn set_program<U: Uniform>(&mut self, program: Program<U>) {
		self.program = Box::new(program);
	}
	fn draw(&self) {
		draw(&self.program);
	}
}

trait Uniform: 'static + Clone {}

struct Program<U: Uniform + ?Sized> {
	uniform: U,
}

impl<U: Uniform + Sized> Program<U> {
	fn new(uniform: U) -> Self {
		return Self {
			uniform: uniform,
		};
	}
}

fn draw<U: Uniform + ?Sized>(program: &Program<U>) {
	// ...
}

struct Uniform1;
impl Uniform for Uniform1 {}

struct Uniform2;
impl Uniform for Uniform2 {}

fn main() {

	let p1 = Program::new(Uniform1);
	let p2 = Program::new(Uniform2);
	let mut renderer = Renderer::new(p1);

	draw(&p2);
	renderer.set_program(p2);

}

