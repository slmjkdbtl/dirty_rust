// wengwengweng

#![feature(clamp)]

use dirty::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;
use input::Mouse;

fn get_mesh_size(m: &gfx::Mesh) -> f32 {

	let (min, max) = m.bbox();
	let size = (max - min).mag();

	return size;

}

struct ObjViewer {
	shader: gfx::Shader3D<()>,
	mesh: gfx::Mesh,
	size: f32,
	cam: gfx::PerspectiveCam,
	rot: Vec2,
	pos: Vec2,
	dis: f32,
	resetting: bool,
}

impl app::State for ObjViewer {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mesh = gfx::Mesh::from_obj(ctx, include_str!("res/ok.obj"), None)?;
		let size = get_mesh_size(&mesh);
		let dis = size;
		let pos = vec2!(0);

		let viewer = Self {
			size: size,
			mesh: mesh,
			pos: pos,
			dis: dis,
			cam: gfx::PerspectiveCam::new(60.0, ctx.width() as f32 / ctx.height() as f32, 0.01, 2048.0, vec3!(pos.x, pos.y, -dis), 0.0, 0.0),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			rot: vec2!(0),
			resetting: false,
		};

		return Ok(viewer);

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				if k == Key::F {
					ctx.toggle_fullscreen();
				}

				if k == Key::Space {
					self.resetting = true;
				}

				if k == Key::Esc {
					ctx.quit();
				}

			},

			Scroll(s) => {

				self.resetting = false;
				self.dis -= s.y * (self.size / 240.0);
				self.dis = self.dis.clamp(self.size * 0.3, self.size * 3.0);

			},

			MouseMove(delta) => {

				if ctx.mouse_down(Mouse::Left) {

					self.resetting = false;
					self.rot += delta;

					if self.rot.x >= 360.0 {
						self.rot.x = self.rot.x - 360.0;
					}

					if self.rot.x <= -360.0 {
						self.rot.x = self.rot.x + 360.0;
					}

					if self.rot.y >= 360.0 {
						self.rot.y = self.rot.y - 360.0;
					}

					if self.rot.y <= -360.0 {
						self.rot.y = self.rot.y + 360.0;
					}

				}

			},

			FileDrop(path) => {

				self.resetting = false;

				let content = fs::read_str(&path)?;

				if let Ok(mesh) = gfx::Mesh::from_obj(ctx, &content, None) {
					self.mesh = mesh;
					self.resetting = true;
				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let move_speed = self.dis;

		if ctx.key_down(Key::A) {
			self.resetting = false;
			self.pos.x -= move_speed * ctx.dt();
		}

		if ctx.key_down(Key::D) {
			self.resetting = false;
			self.pos.x += move_speed * ctx.dt();
		}

		if ctx.key_down(Key::W) {
			self.resetting = false;
			self.pos.y += move_speed * ctx.dt();
		}

		if ctx.key_down(Key::S) {
			self.resetting = false;
			self.pos.y -= move_speed * ctx.dt();
		}

		self.pos = self.pos.clamp(-vec2!(self.dis), vec2!(self.dis));

		if self.resetting {

			self.size = get_mesh_size(&self.mesh);

			let dest_rot = vec2!(0);
			let dest_pos = vec2!(0);
			let dest_dis = self.size;
			let t = ctx.dt() * 6.0;

			self.rot = self.rot.lerp(dest_rot, t);
			self.pos = self.pos.lerp(dest_pos, t);
			self.dis = self.dis.lerp(dest_dis, t);

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let center = self.mesh.center();

		self.cam.set_pos(vec3!(self.pos.x, self.pos.y, -self.dis));

		ctx.use_cam(&self.cam, |ctx| {

			ctx.draw_3d_with(&self.shader, &(), |ctx| {

// 				ctx.draw(&shapes::rect3d(min, max))?;
// 				ctx.draw(&shapes::circle3d(center, 3.0))?;

				ctx.push(&gfx::t()
					.rotate_y(-self.rot.x.to_radians())
					.rotate_x(-self.rot.y.to_radians())
					.translate_3d(-center)
				, |ctx| {

					ctx.draw(&shapes::mesh(&self.mesh))?;

					return Ok(());

				})?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		ctx.push(&gfx::t()
			.translate(vec2!(24))
		, |ctx| {

			ctx.draw(&shapes::text("drag .obj files into this window"))?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<ObjViewer>() {
		println!("{}", err);
	}

}

