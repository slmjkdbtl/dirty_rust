# DIRTY
personal toolkit for creating game-like experiences

### usage
add to `Cargo.toml`
```toml
[dependencies.dirty]
git = "https://github.com/slmjkdbtl/DIRTY"
```
not on crates.io yet due to duplicate crate name

### modules
- **app** Lifecycles, Time and Errors
- **window** Window & Events
- **gfx** 2D Rendering
- **audio** Sound Loading & playback
- **math** Common Math Functions & Structs
- **fs** Common File Related Functions
- **ecs** Simple ECS
- **ui** Simple UI

### example
```rust
use dirty::*;

fn main() {

	// init
	app::init();
	window::init("yo", 640, 480);
	audio::init();

	// main loop
	app::run(|| {

		// transform
		gfx::push();
		gfx::translate(vec2!(220, 120));
		gfx::scale(vec2!(12));
		gfx::color(color!(0, 1, 1, 1));

		// draw text
		gfx::text("yo");
		gfx::pop();

		// input
		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}
```
with `ecs`:

```rust
use dirty::*;
use dirty::ecs::*;
use dirty::ecs::derive::*;

#[derive(Comp, Clone)]
struct Pos {
	x: f32,
	y: f32,
}

struct MoveSys;

impl System for MoveSys {

	fn filter(&self) -> Filter {
		return filter![Pos];
	}

	fn each(&mut self, e: &mut Entity) {

		let mut pos = e.get::<Pos>();

		pos.x = pos.x + 1.0;
		println!("pos: ({}, {})", pos.x, pos.y);
		e.set::<Pos>(pos);

	}

}

fn thing(x: f32, y: f32) -> Entity {
	return entity![Pos { x, y }];
}

fn main() {

	// init modules
	app::init();

	// create new world
	let mut world = World::new();

	// add entities
	world.add(thing(0.0, 0.0));

	// run systems
	world.run(MoveSys);

	// loop
	for _ in 0..3 {
		world.update();
	}

}
```
more under `examples/`

### notes & caveats

- upon initialization each module has its hidden `static` state that dies when program ends
- `app::init()` also overrides default panic behavior (also display messages to screen if `window` is initiated)
- currently using OpenGL 2.1 for better compatibility, but might change to gfx-hal in the future
- currently no proper error handling (a lot of internal panics, but with pretty error screen)
- on Windows, `audio::init()` must be called before `window::init()`, for some reason it crashes if tried to get audio device after SDL initialization
- planning on adding scripting support
- ui style is a tribute to [MEKA](http://www.smspower.org/meka/), thanks for keep making awesome tools ocornut

### faq
- Why should I just this over ggez or amethyst?
> You should not. This is my personal toolkit, not intented to compete or relate to any other libs.
- Is it fast?
> Fast enough, not crazy fast. 2D sprite batching is done by default, no other notable performance tricks.
- Pineapple on pizza?
> Yes.

