// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

comp!(T {

	pos: Vec2 (vec2!()),
	rot: f32 (0.0),
	scale: Vec2 (vec2!(1)),

});

fn main() {

	let a = Scene::new();

	a.get(&compfilter![T]);

}
