// wengwengweng

#![feature(clamp)]

use dirty::*;
use math::*;
use geom::*;
use input::Key;
use gfx::Camera;

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

#[derive(Clone)]
pub struct PixUniform {
	pub resolution: Vec2,
	pub size: f32,
}

impl gfx::CustomUniform for PixUniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_resolution" => &self.resolution,
			"u_size" => &self.size,
		];
	}
}

struct Game {
	model: gfx::Model,
	cam: gfx::PerspectiveCam,
	move_speed: f32,
	eye_speed: f32,
	shader: gfx::Shader<Uniform>,
	pix_shader: gfx::Shader<PixUniform>,
	show_ui: bool,
	canvas: gfx::Canvas,
	floor: gfx::MeshData,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let model = gfx::Model::from_glb(
			ctx,
			include_bytes!("res/btfly.glb"),
		)?;

		let model = gfx::Model::from_obj(
			ctx,
			include_str!("res/truck.obj"),
			Some(include_str!("res/truck.mtl")),
			None,
		)?;

		let floor = meshgen::checkerboard(2.0, 9, 9);

		return Ok(Self {
			model: model,
			cam: gfx::PerspectiveCam {
				fov: f32::to_radians(60.0),
				aspect: ctx.width() as f32 / ctx.height() as f32,
				near: 0.1,
				far: 1024.0,
				pos: vec3!(0, 1, 6),
				dir: vec3!(0, 0, -1),
			},
			move_speed: 12.0,
			eye_speed: 32.0,
			shader: gfx::Shader::from_frag(ctx, include_str!("res/fog.frag"))?,
			pix_shader: gfx::Shader::from_frag(ctx, include_str!("res/pix.frag"))?,
			show_ui: false,
			canvas: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
			floor: floor,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			Resize(w, h) => {
				self.cam.aspect = *w as f32 / *h as f32;
				self.canvas.resize(ctx, *w, *h)?;
			},

			KeyPress(k) => {
				let mods = ctx.key_mods();
				match *k {
					Key::Esc => {
						ctx.toggle_cursor_hidden();
						ctx.toggle_cursor_locked()?;
					},
					Key::F => ctx.toggle_fullscreen(),
					Key::Q if mods.meta => ctx.quit(),
					Key::Tab => {
						ctx.set_cursor_hidden(self.show_ui);
						ctx.set_cursor_locked(self.show_ui)?;
						self.show_ui = !self.show_ui;
					}
					_ => {},
				}
			},

			MouseMove(delta) => {

				if ctx.is_cursor_hidden() {

					let mut rx = self.cam.yaw();
					let mut ry = self.cam.pitch();
					let dead = f32::to_radians(60.0);

					rx += delta.x * self.eye_speed * 0.0001;
					ry -= delta.y * self.eye_speed * 0.0001;

					ry = ry.clamp(-dead, dead);

					self.cam.set_angle(rx, ry);

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn imgui(&mut self, ui: &mut imgui::Ui) -> Result<()> {

		use imgui::im_str;

		if self.show_ui {

			imgui::Window::new(im_str!("test"))
				.size([320.0, 240.0], imgui::Condition::FirstUseEver)
				.build(&ui, || {

					if ui.button(im_str!("Look Center"), [0.0, 0.0]) {
						self.cam.set_dest(vec3!(0));
					}

					let mut fov = self.cam.fov.to_degrees();

					if ui.drag_float(im_str!("FOV"), &mut fov)
						.min(45.0)
						.max(90.0)
						.build()
					{
						self.cam.fov = fov.to_radians();
					}

				});

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		let dt = ctx.dt();

		if ctx.key_down(Key::W) {
			self.cam.pos += self.cam.front() * dt * self.move_speed;
		}

		if ctx.key_down(Key::S) {
			self.cam.pos += self.cam.back() * dt * self.move_speed;
		}

		if ctx.key_down(Key::A) {
			self.cam.pos += self.cam.left() * dt * self.move_speed;
		}

		if ctx.key_down(Key::D) {
			self.cam.pos += self.cam.right() * dt * self.move_speed;
		}

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

		ctx.draw_on(&self.canvas, |ctx| {

			ctx.clear();

			let p = vec3!(0);
			let origin = self.cam.to_screen(ctx, p);
			let mray = self.cam.mouse_ray(ctx);

			ctx.use_cam(&self.cam, |ctx| {

				ctx.draw_with(&self.shader, &Uniform {
					cam_pos: self.cam.pos,
					fog_color: rgba!(0, 0, 0, 1),
					fog_level: 3.0,
				}, |ctx| {

					let bbox = self.model.bbox().transform(mat4!());
					let mray = Ray3::new(self.cam.pos, self.cam.dir);

					let c = if col::intersect3d(mray, bbox) {
						rgba!(0, 0, 1, 1)
					} else {
						rgba!(1)
					};

					ctx.draw(&shapes::model(&self.model))?;

					ctx.draw(
						&shapes::Rect3D::from_bbox(bbox)
							.line_width(5.0)
							.color(c)
					)?;

					let ground = Plane::new(vec3!(0, 1, 0), 0.0);

// 					if let Some(pt) = kit::geom::ray_plane(mray, ground) {
// 						ctx.draw_t(mat4!().t3(pt), &shapes::cube())?;
// 					}

					ctx.draw(&shapes::Raw::from_meshdata(&self.floor))?;

					return Ok(());

				})?;

				return Ok(());

			})?;

			ctx.draw(&shapes::circle(vec2!(0), 2.0))?;

			ctx.draw_t(
				mat4!()
					.t2(origin)
					,
				&shapes::text("car")
					.size(16.0)
					,
			)?;

			return Ok(());

		})?;

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw_with(&self.pix_shader, &PixUniform {
			resolution: vec2!(ctx.width(), ctx.height()),
			size: 0.0,
		}, |ctx| {
			ctx.draw(&shapes::canvas(&self.canvas))?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.cursor_hidden(true)
		.cursor_locked(true)
		.resizable(true)
// 		.hidpi(false)
// 		.fps_cap(None)
// 		.vsync(false)
		.run::<Game>() {
		println!("{}", err);
	}

}

