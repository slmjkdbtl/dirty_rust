// wengwengweng

use dirty::*;
use dirty::kit::*;

#[derive(Clone)]
struct Name(String);

#[derive(Clone)]
struct Age(i32);

fn make_guy(name: &str, age: i32) -> Entity {
	return entity![Name(name.to_owned()), Age(3)];
}

fn main() {

	let mut guy = make_guy("David", 8);

	for _ in 0..3 {

		grow(&mut guy);
		print_info(&mut guy);

	}

}

fn grow(e: &mut Entity) {

	let mut age = e.get::<Age>();

	age.0 = age.0 + 1;
	e.set::<Age>(age);

}

fn print_info(e: &mut Entity) {

	let name = e.get::<Name>();
	let age = e.get::<Age>();

	println!("{}, {}", name.0, age.0);

}

