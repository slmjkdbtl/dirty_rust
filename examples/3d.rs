// wengwengweng

#![feature(clamp)]

use dirty::*;
use dirty::app::*;
use input::Key;

struct ObjViewer {
	shader: gfx::Shader3D<()>,
	mesh: gfx::Mesh,
	size: f32,
	cam: gfx::PerspectiveCam,
	dis: f32,
	rot_x: f32,
	rot_y: f32,
}

fn get_mesh_size(m: &gfx::Mesh) -> f32 {

	let (min, max) = m.bbox();
	let size = (max - min).mag();

	return size;

}

impl app::State for ObjViewer {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mesh = gfx::Mesh::from_obj(ctx, include_str!("res/ok.obj"), None)?;
		let size = get_mesh_size(&mesh);

		let viewer = Self {
			size: size,
			mesh: mesh,
			dis: size,
			cam: gfx::PerspectiveCam::new(60.0, ctx.width() as f32 / ctx.height() as f32, 0.01, 1024.0, vec3!(0, 0, -size), 0.0, 0.0),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			rot_x: 0.0,
			rot_y: 0.0,
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
				if k == Key::Esc {
					ctx.quit();
				}
			},

			Scroll(s) => {
				self.dis += s.y * (self.size / 240.0);
				self.dis = self.dis.clamp(self.size * 0.3, self.size * 3.0);
				self.cam.set_pos(vec3!(0, 0, -self.dis));
			},

			MouseMove(delta) => {
				if ctx.mouse_down(input::Mouse::Left) {
					self.rot_x += delta.x;
					self.rot_y += delta.y;
				}
			},

			FileDrop(path) => {

				let content = fs::read_str(&path)?;

				if let Ok(mesh) = gfx::Mesh::from_obj(ctx, &content, None) {

					self.mesh = mesh;
					self.size = get_mesh_size(&self.mesh);
					self.dis = self.size;
					self.cam.set_pos(vec3!(0, 0, -self.dis));
					self.rot_x = 0.0;
					self.rot_y = 0.0;

				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let center = self.mesh.center();

		ctx.use_cam(&self.cam, |ctx| {

			ctx.draw_3d_with(&self.shader, &(), |ctx| {

// 				ctx.draw(&shapes::rect3d(min, max))?;
// 				ctx.draw(&shapes::circle3d(center, 3.0))?;

				ctx.push(&gfx::t()
					.rotate_y(-self.rot_x.to_radians())
					.rotate_x(-self.rot_y.to_radians())
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

