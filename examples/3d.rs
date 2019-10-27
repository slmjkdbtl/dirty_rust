// wengwengweng

#![feature(clamp)]
#![feature(option_flattening)]

use std::path::Path;

use dirty::*;
use dirty::task::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;
use input::Mouse;

type LoadResult = Result<gfx::ModelData>;

struct ObjViewer {
	shader: gfx::Shader3D<()>,
	model: Option<DisplayedModel>,
	cam: gfx::PerspectiveCam,
	rot: Vec2,
	pos: Vec2,
	dis: f32,
	scale: f32,
	resetting: bool,
	loader: Task<LoadResult>,
}

fn load_file(path: impl AsRef<Path>) -> Task<LoadResult> {

	let mut path = path.as_ref().to_owned();

	return Task::exec(move || {

		let obj_src = fs::read_str(&path)?;

		path.set_extension("mtl");

		let mtl_src = fs::read_str(&path).ok();
		let mtl_src = mtl_src.as_ref().map(|s| s.as_str());

		path.set_extension("png");

		let img_src = fs::read(&path).ok();
		let img_src = img_src.as_ref().map(|i| i.as_slice());

		let data = gfx::Model::load_obj(&obj_src, mtl_src, img_src)?;

		return Ok(data);

	});
}

impl ObjViewer {

	fn update_model(&mut self, model: gfx::Model) {

		self.resetting = true;
		let model = DisplayedModel::from(model);
		self.dis = model.size;
		self.model = Some(model);

	}

	fn load_file(&mut self, path: impl AsRef<Path>) {

		self.loader = load_file(path);
		self.model = None;
		self.scale = 0.0;
		self.resetting = false;

	}

}

struct DisplayedModel {
	size: f32,
	show_normal: bool,
	model: gfx::Model,
}

impl DisplayedModel {

	fn from(model: gfx::Model) -> Self {

		let (min, max) = model.bound();
		let size = (max - min).mag();

		return Self {
			model: model,
			show_normal: true,
			size: size,
		};

	}

}

impl app::State for ObjViewer {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			model: None,
			pos: vec2!(0),
			dis: 0.0,
			cam: gfx::PerspectiveCam::new(60.0, ctx.width() as f32 / ctx.height() as f32, 0.01, 2048.0, vec3!(), 0.0, 0.0),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			rot: vec2!(0),
			resetting: false,
			scale: 0.0,
			loader: load_file("examples/res/kart.obj"),
		});

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

				if let Some(model) = &self.model {
					if !self.resetting {
						self.dis -= s.y * (model.size / 240.0);
						self.dis = self.dis.clamp(model.size * 0.2, model.size * 3.0);
					}
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

			FileDrop(path) => {
				self.load_file(&path);
			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if let Some(data) = self.loader.poll() {
			if let Ok(data) = data {
				if let Ok(model) = gfx::Model::from_data(ctx, data) {
					self.update_model(model);
				}
			}
		}

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

		if let Some(model) = &self.model {

			if self.resetting {

				let dest_rot = vec2!(0);
				let dest_pos = vec2!(0);
				let dest_scale = 1.0;
				let dest_dis = model.size;
				let t = ctx.dt() * 4.0;

				self.rot = self.rot.lerp(dest_rot, t);
				self.pos = self.pos.lerp(dest_pos, t);
				self.scale = self.scale.lerp(dest_scale, t);
				self.dis = self.dis.lerp(dest_dis, t);

			}

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if let Some(model) = &self.model {

			let (max, min) = model.model.bound();
			let center = model.model.center();

			self.cam.set_pos(vec3!(self.pos.x, self.pos.y, self.dis));

			ctx.use_cam(&self.cam, |ctx| {

				ctx.push(&gfx::t()
					.scale_3d(vec3!(self.scale))
					.rotate_y(self.rot.x.to_radians())
					.rotate_x(self.rot.y.to_radians())
					.translate_3d(-center)
				, |ctx| {

					if model.show_normal {
						ctx.draw_3d_with(&self.shader, &(), |ctx| {
							ctx.draw(&shapes::model(&model.model))?;
							return Ok(());
						})?;
					} else {
						ctx.draw(&shapes::model(&model.model))?;
					}

					return Ok(());

				})?;

				return Ok(());

			})?;

		} else {

			ctx.push(&gfx::t()
				.translate(ctx.coord(gfx::Origin::BottomLeft) + vec2!(24, -24))
			, |ctx| {
				ctx.draw(
					&shapes::text("loading...")
						.align(gfx::Origin::BottomLeft)
				)?;
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

