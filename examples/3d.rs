// wengwengweng

use dirty::*;
use math::*;
use geom::*;
use input::Key;
use gfx::Camera;
use gfx::shapes;

const SCALE: f32 = 4.0;

#[derive(Clone)]
struct Uniform {
	cam_pos: Vec3,
	fog_color: Color,
	fog_level: f32,
}

impl gfx::CustomUniform for Uniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_cam_pos" => &self.cam_pos,
			"u_fog_color" => &self.fog_color,
			"u_fog_level" => &self.fog_level,
		];
	}
}

struct Game {
	model: gfx::Model,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
	shader: gfx::Shader<Uniform>,
	show_ui: bool,
	floor: gfx::MeshData,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let model = gfx::Model::from_glb(
			d.gfx,
			include_bytes!("res/btfly.glb"),
		)?;

		let model = gfx::Model::from_obj(
			d.gfx,
			include_str!("res/truck.obj"),
			Some(include_str!("res/truck.mtl")),
			None,
		)?;

		let floor = meshgen::checkerboard(2.0, 9, 9);

		return Ok(Self {
			model: model,
			cam: gfx::PerspectiveCam {
				fov: f32::to_radians(60.0),
				aspect: d.gfx.width() as f32 / d.gfx.height() as f32,
				near: 0.1,
				far: 1024.0,
				pos: vec3!(0, 1, 6),
				dir: vec3!(0, 0, -1),
			},
			move_speed: 12.0,
			eye_speed: 32.0,
			shader: gfx::Shader::from_frag(d.gfx, include_str!("res/fog.frag"))?,
			show_ui: false,
			floor: floor,
		});

	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			Resize(w, h) => {
				self.cam.aspect = *w as f32 / *h as f32;
			},

			KeyPress(k) => {
				let mods = d.window.key_mods();
				match *k {
					Key::Esc => {
						d.window.toggle_cursor_hidden();
						d.window.toggle_cursor_locked();
					},
					Key::F => d.window.toggle_fullscreen(),
					Key::Q if mods.meta => d.window.quit(),
					Key::L => {
						d.window.set_cursor_hidden(self.show_ui);
						d.window.set_cursor_locked(self.show_ui);
						self.show_ui = !self.show_ui;
					}
					_ => {},
				}
			},

			MouseMove(delta) => {

				if d.window.is_cursor_hidden() {

					let mut rx = self.cam.yaw();
					let mut ry = self.cam.pitch();
					let dead = f32::to_radians(60.0);

					rx += delta.x * self.eye_speed * 0.0001;
					ry += delta.y * self.eye_speed * 0.0001;

					ry = ry.max(-dead).min(dead);

					self.cam.set_angle(rx, ry);

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		let dt = d.app.dt().as_secs_f32();

		if d.window.key_down(Key::W) {
			self.cam.pos += self.cam.front() * dt * self.move_speed;
		}

		if d.window.key_down(Key::S) {
			self.cam.pos += self.cam.back() * dt * self.move_speed;
		}

		if d.window.key_down(Key::A) {
			self.cam.pos += self.cam.left() * dt * self.move_speed;
		}

		if d.window.key_down(Key::D) {
			self.cam.pos += self.cam.right() * dt * self.move_speed;
		}

		d.window.set_title(&format!("FPS: {} DCS: {}", d.app.fps(), d.gfx.draw_calls()));

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		d.gfx.use_cam(&self.cam, |gfx| {

			gfx.draw_with(&self.shader, &Uniform {
				cam_pos: self.cam.pos,
				fog_color: rgba!(0, 0, 0, 1),
				fog_level: 3.0,
			}, |gfx| {

				let bbox = self.model.bbox().transform(mat4!());
				let mray = Ray3::new(self.cam.pos, self.cam.dir);

				let c = if col::intersect3d(mray, bbox) {
					rgba!(0, 0, 1, 1)
				} else {
					rgba!(1)
				};

				gfx.draw(&shapes::model(&self.model))?;

				gfx.draw(
					&shapes::Rect3D::from_bbox(bbox)
						.line_width(1.0)
						.color(c)
				)?;

				let ground = Plane::new(vec3!(0, 1, 0), 0.0);

				gfx.draw(&shapes::Raw::from_meshdata(&self.floor))?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		let top_left = d.gfx.coord(gfx::Origin::TopLeft);

		let lines = [
			"F:       toggle fullscreen",
			"Esc:     toggle cursor",
			"W/A/S/D: move",
		];

		for (i, l) in lines.iter().enumerate() {
			d.gfx.draw_t(
				mat4!()
					.t2(top_left + vec2!(24, -24.0 - i as f32 * 24.0))
					,
				&shapes::text(l)
					.align(gfx::Origin::TopLeft)
					.size(12.0)
					,
			)?;
		}

		d.gfx.draw(&shapes::circle(vec2!(0), 2.0))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.cursor_hidden(true)
		.cursor_locked(true)
		.resizable(true)
// 		.hidpi(false)
// 		.vsync(false)
		.run::<Game>() {
		elog!("{}", err);
	}

}

