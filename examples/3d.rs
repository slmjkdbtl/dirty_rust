// wengwengweng

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;

mod pix;
use pix::*;

struct Game {
	model: gfx::Model,
	pix_effect: PixEffect,
	shader: gfx::Shader3D<LightUniform>,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
	skybox: gfx::Skybox,
}

#[derive(Clone)]
struct LightUniform {
	pos: Vec3,
	color: Vec3,
	mix: f32,
}

impl gfx::Uniform for LightUniform {
	fn values(&self) -> gfx::UniformValues {
		return hashmap![
			"u_light_pos" => &self.pos,
			"u_light_color" => &self.color,
			"u_light_mix" => &self.mix,
		];
	}
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut model = gfx::Model::from_obj(
			ctx,
			include_str!("res/kart.obj"),
			Some(include_str!("res/kart.mtl")),
			None,
		)?;

// 		model.update(|data| {
// 			for m in data {
// 				for v in &mut m.vertices {
// 					v.color = color!(rand!(), rand!(), rand!(), 1);
// 				}
// 			}
// 		});

// 		let mut model = gfx::Model::from_gltf(ctx, "examples/res/Duck.gltf")?;
// 		let mut model = gfx::Model::from_glb(ctx, include_bytes!("res/duck.glb"))?;

		let skybox = gfx::Skybox::from_bytes(
			&ctx,
			include_bytes!("res/forest_rt.png"),
			include_bytes!("res/forest_lf.png"),
			include_bytes!("res/forest_up.png"),
			include_bytes!("res/forest_dn.png"),
			include_bytes!("res/forest_bk.png"),
			include_bytes!("res/forest_ft.png"),
		)?;

		return Ok(Self {
			model: model,
			pix_effect: PixEffect::new(ctx)?,
			cam: gfx::PerspectiveCam::new(60.0, ctx.width() as f32 / ctx.height() as f32, 0.1, 1024.0, vec3!(0, 0, 12), 0.0, 0.0),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/light.frag"))?,
			move_speed: 12.0,
			eye_speed: 0.16,
			skybox: skybox,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
				if k == Key::F {
					ctx.toggle_fullscreen();
				}
			},

			MouseMove(delta) => {

				if ctx.is_cursor_locked() {

					let mut rx = self.cam.yaw();
					let mut ry = self.cam.pitch();
					let dead = 48.0f32.to_radians();

					rx += delta.x * self.eye_speed * ctx.dt();
					ry -= delta.y * self.eye_speed * ctx.dt();

					if ry > dead {
						ry = dead;
					}

					if ry < -dead {
						ry = -dead;
					}

					self.cam.set_angle(rx, ry);

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if ctx.key_down(Key::W) {
			self.cam.set_pos(self.cam.pos() + self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::S) {
			self.cam.set_pos(self.cam.pos() - self.cam.front() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::A) {
			self.cam.set_pos(self.cam.pos() - self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
		}

		if ctx.key_down(Key::D) {
			self.cam.set_pos(self.cam.pos() + self.cam.front().cross(vec3!(0, 1, 0)).normalize() * ctx.dt() * self.move_speed);
		}

		self.pix_effect.render(ctx, |ctx| {

			ctx.clear();
// 			ctx.clear_ex(gfx::Surface::Depth);

			let light_pos = vec3!(ctx.time().sin() * 12.0, 12, ctx.time().cos() * 12.0);

			ctx.use_cam(&self.cam, |ctx| {

				ctx.draw(&shapes::skybox(&self.skybox))?;

				ctx.draw_t(&gfx::t().t3(light_pos), &shapes::cube())?;

				ctx.draw_3d_with(&self.shader, &LightUniform {
					pos: light_pos,
					color: vec3!(1, 1, 1),
					mix: 0.4,
				}, |ctx| {
					ctx.draw(&shapes::model(&self.model))?;
					return Ok(());
				})?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		ctx.set_title(&format!("{}", ctx.fps()));

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.pix_effect.draw(ctx, &PixUniform {
			resolution: vec2!(ctx.gwidth(), ctx.gheight()),
			size: 4.0,
		})?;

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

