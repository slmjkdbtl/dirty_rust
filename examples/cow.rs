// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

struct Game {
	model: gfx::Model,
	pos: Vec3,
	rx: f32,
	ry: f32,
	pixel_effect: gfx::Shader,
	blur_effect: gfx::Shader,
	canvas: gfx::Canvas,
	canvas2: gfx::Canvas,
	motion: f32,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		ctx.cam_pos(vec3!(0, 0, -320));

		let pixel_effect = gfx::Shader::effect(ctx, include_str!("res/pix.frag"))?;

		pixel_effect.send("size", 6.0);
		pixel_effect.send("dimension", vec2!(ctx.width(), ctx.height()));

		let blur_effect = gfx::Shader::effect(ctx, include_str!("res/blur.frag"))?;

		blur_effect.send("dir", vec2!(1, 0));
		blur_effect.send("radius", 12.0);
		blur_effect.send("resolution", vec2!(ctx.width(), ctx.height()));

		return Ok(Self {
			model: gfx::Model::from_obj(ctx, include_str!("res/teapot.obj"))?,
			pos: vec3!(0, 0, -320),
			pixel_effect: pixel_effect,
			blur_effect: blur_effect,
			canvas: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
			canvas2: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
			rx: 0.0,
			ry: 0.0,
			motion: 0.0,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		let move_speed = 160.0;
		let rot_speed = 0.15;

		self.motion = math::lerp(self.motion, 0.0, ctx.dt().into());
		self.blur_effect.send("radius", self.motion);
// 		self.blur_effect.send("radius", md.mag());

		match e {

			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
				if *k == Key::F {
					ctx.toggle_fullscreen();
				}
			},

			KeyDown(k) => {

				if *k == Key::W {
					self.pos += ctx.cam_front() * ctx.dt() * move_speed;
				}

				if *k == Key::S {
					self.pos -= ctx.cam_front() * ctx.dt() * move_speed;
				}

				if *k == Key::A {
					self.pos += ctx.cam_front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * move_speed;
				}

				if *k == Key::D {
					self.pos -= ctx.cam_front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * move_speed;
				}

			},

			MouseMove(delta) => {

				let md: Vec2 = (*delta).into();

				self.rx -= md.x * rot_speed;
				self.ry -= md.y * rot_speed;

				if self.ry > 48.0 {
					self.ry = 48.0;
				}

				if self.ry < -48.0 {
					self.ry = -48.0;
				}

				self.blur_effect.send("dir", md.normalize());
				self.motion = md.mag();

			},

			_ => {},

		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_on(&self.canvas, |ctx| {

			ctx.clear();
			ctx.push();
			ctx.translate_3d(vec3!(0, 0, 0));
			ctx.rotate_y(ctx.time().into());
			ctx.draw(shapes::model(&self.model))?;
			ctx.pop()?;

			return Ok(());

		})?;

// 		ctx.draw_on(&self.canvas2, |ctx| {
			ctx.draw_with(&self.pixel_effect, |ctx| {
				ctx.draw(shapes::canvas(&self.canvas))?;
				return Ok(());
			})?;
// 			return Ok(());
// 		});

// 		ctx.draw_with(&self.pixel_effect, |ctx| {
// 			ctx.draw(shapes::canvas(&self.canvas2))?;
// 			return Ok(());
// 		})?;

		ctx.cam_pos(self.pos);
		ctx.cam_look(self.rx.to_radians(), self.ry.to_radians());

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.cursor_locked(true)
		.cursor_hidden(true)
		.run::<Game>() {
		println!("{}", err);
	}

}

