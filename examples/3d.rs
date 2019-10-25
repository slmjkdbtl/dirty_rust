// wengwengweng

#![feature(clamp)]
#![feature(option_flattening)]

use std::path::Path;

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
	mesh: Option<gfx::Mesh>,
	show_normal: bool,
	size: f32,
	cam: gfx::PerspectiveCam,
	rot: Vec2,
	pos: Vec2,
	dis: f32,
	scale: f32,
	resetting: bool,
}

impl ObjViewer {

	fn load_file(&mut self, ctx: &Ctx, path: impl AsRef<Path>) -> Result<()> {

		let mut path = path.as_ref().to_owned();

		self.resetting = false;

		let obj_src = fs::read_str(&path)?;

		path.set_extension("mtl");

		let mtl_src = fs::read_str(&path).ok();
		let mtl_src = mtl_src.as_ref().map(|s| s.as_str());

		path.set_extension("png");

		let tex_src = fs::read(&path)
			.ok()
			.map(|b| gfx::Texture::from_bytes(ctx, &b).ok())
			.flatten();

		self.show_normal = tex_src.is_none() && mtl_src.is_none();

		if let Ok(mesh) = gfx::Mesh::from_obj(ctx, &obj_src, mtl_src, tex_src) {
			self.size = get_mesh_size(&mesh);
			self.dis = self.size;
			self.scale = 0.0;
			self.mesh = Some(mesh);
			self.resetting = true;
		}

		return Ok(());

	}

}

impl app::State for ObjViewer {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut viewer = Self {
			size: 0.0,
			mesh: None,
			pos: vec2!(0),
			dis: 0.0,
			show_normal: true,
			cam: gfx::PerspectiveCam::new(60.0, ctx.width() as f32 / ctx.height() as f32, 0.01, 2048.0, vec3!(), 0.0, 0.0),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			rot: vec2!(0),
			resetting: false,
			scale: 0.0,
		};

		viewer.load_file(ctx, "examples/res/kart.obj");

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

			Scroll(s, phase) => {

				if let input::ScrollPhase::Solid = phase {
					self.resetting = false;
				}

				if !self.resetting {
					self.dis -= s.y * (self.size / 240.0);
					self.dis = self.dis.clamp(self.size * 0.2, self.size * 3.0);
				}

			},

			MouseMove(delta) => {

				if ctx.mouse_down(Mouse::Left) {

					// TODO: correctly handle rotation
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

			FileDrop(mut path) => {
				self.load_file(ctx, &path);
			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let move_speed = f32::sqrt(self.dis) * 2.0;

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

		let range = f32::sqrt(self.dis);

		self.pos = self.pos.clamp(-vec2!(range), vec2!(range));

		if self.resetting {

			let dest_rot = vec2!(0);
			let dest_pos = vec2!(0);
			let dest_scale = 1.0;
			let dest_dis = self.size;
			let t = ctx.dt() * 6.0;

			self.rot = self.rot.lerp(dest_rot, t);
			self.pos = self.pos.lerp(dest_pos, t);
			self.scale = self.scale.lerp(dest_scale, t);
			self.dis = self.dis.lerp(dest_dis, t);

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if let Some(mesh) = &self.mesh {

			let (max, min) = mesh.bbox();
			let center = mesh.center();

			self.cam.set_pos(vec3!(self.pos.x, self.pos.y, self.dis));

			ctx.use_cam(&self.cam, |ctx| {

				ctx.push(&gfx::t()
					.scale_3d(vec3!(self.scale))
					.rotate_y(self.rot.x.to_radians())
					.rotate_x(self.rot.y.to_radians())
					.translate_3d(-center)
				, |ctx| {

					if self.show_normal {
						ctx.draw_3d_with(&self.shader, &(), |ctx| {
							ctx.draw(&shapes::mesh(&mesh))?;
							return Ok(());
						})?;
					} else {
						ctx.draw(&shapes::mesh(&mesh))?;
					}

					return Ok(());

				})?;

				return Ok(());

			})?;

		}

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

