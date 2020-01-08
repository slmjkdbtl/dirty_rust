// wengwengweng

// TODO: handle load failure
// TODO: lighting

#![feature(clamp)]

use std::path::Path;

use dirty::*;
use dirty::task::*;
use dirty::app::*;
use dirty::math::*;
use input::Key;
use input::Mouse;

type LoadResult = Result<gfx::ModelData>;

struct Viewer {
	shader: gfx::Shader3D<()>,
	model: Option<DisplayedModel>,
	cam: gfx::OrthoCam,
	rot: Vec2,
	pos: Vec2,
	scale: f32,
	resetting: bool,
	loader: Task<LoadResult>,
	wireframe: bool,
	helping: bool,
	show_normal: bool,
}

fn load_file(path: impl AsRef<Path>) -> Task<LoadResult> {

	let mut path = path.as_ref().to_owned();

	return Task::exec(move || {

		// TODO: use Model::from_file
		match fs::extname(&path)?.as_ref() {

			"obj" => {

				let obj_src = fs::read_str(&path)?;

				path.set_extension("mtl");

				let mtl_src = fs::read_str(&path).ok();
				let mtl_src = mtl_src.as_ref().map(|s| s.as_str());

				path.set_extension("png");

				let img_src = fs::read(&path).ok();
				let img_src = img_src.as_ref().map(|i| i.as_slice());

				let data = gfx::Model::load_obj(&obj_src, mtl_src, img_src)?;

				return Ok(data);

			},

			"glb" => {

				let bytes = fs::read(&path)?;
				let data = gfx::Model::load_glb(&bytes)?;

				return Ok(data);

			},

			_ => {
				return Err(Error::Misc(format!("unrecognized model")));
			},

		}

	});
}

impl Viewer {

	fn update_model(&mut self, model: gfx::Model) {

		self.resetting = true;
		let model = DisplayedModel::from(model);
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
	model: gfx::Model,
}

impl DisplayedModel {

	fn from(model: gfx::Model) -> Self {

		let (min, max) = model.bound();
		let size = (max - min).mag();

		return Self {
			model: model,
			size: size,
		};

	}

}

impl app::State for Viewer {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			model: None,
			pos: vec2!(0),
			cam: gfx::OrthoCam::new(ctx.width() as f32, ctx.height() as f32, -2048.0, 2048.0),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			rot: vec2!(0),
			resetting: false,
			loader: load_file("examples/res/truck.obj"),
			wireframe: false,
			helping: false,
			scale: 0.0,
			show_normal: false,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				match *k {
					Key::F => ctx.toggle_fullscreen(),
					Key::Esc => ctx.quit(),
					Key::Space => self.resetting = true,
					Key::L => self.wireframe = !self.wireframe,
					Key::H => self.helping = !self.helping,
					Key::N => self.show_normal = !self.show_normal,
					_ => {},
				}

			},

			Scroll(s, phase) => {

				if let input::ScrollPhase::Solid = phase {
					self.resetting = false;
				}

				if let Some(model) = &self.model {

					if !self.resetting {

						let orig_scale = 480.0 / model.size;

						self.scale += s.y * (1.0 / model.size);
						self.scale = self.scale.clamp(orig_scale * 0.2, orig_scale * 2.4);

					}

				}

			},

			MouseMove(delta) => {

				if ctx.mouse_down(Mouse::Left) {

					// TODO: correctly handle rotation
					self.resetting = false;
					self.rot += *delta;

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

		if let Some(model) = &self.model {

			let move_speed = 480.0;

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

			if self.resetting {

				let dest_rot = vec2!(0);
				let dest_pos = vec2!(0);
				let dest_scale = 480.0 / model.size;
				let t = ctx.dt() * 4.0;

				self.rot = self.rot.lerp(dest_rot, t);
				self.pos = self.pos.lerp(dest_pos, t);
				self.scale = self.scale.lerp(dest_scale, t);

			}

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if let Some(model) = &self.model {

			let center = model.model.center();

			ctx.use_cam(&self.cam, |ctx| {

				ctx.push(&gfx::t()
					.t2(self.pos)
					.s3(vec3!(self.scale))
					.ry(self.rot.x.to_radians())
					.rx(self.rot.y.to_radians())
					.t3(-center)
				, |ctx| {

					let draw_model = |ctx: &mut Ctx| {
						return ctx.draw(
							&shapes::model(&model.model)
								.draw_wireframe(self.wireframe)
								.draw_bound()
						);
					};

					if self.show_normal {
						ctx.draw_3d_with(&self.shader, &(), draw_model)?;
					} else {
						draw_model(ctx)?;
					}

					return Ok(());

				})?;

				return Ok(());

			})?;

		}

		ctx.push(&gfx::t()
			.t2(vec2!(24))
		, |ctx| {

			if self.loader.phase() == TaskPhase::Working {

				ctx.draw(&shapes::text("loading..."))?;

			} else {

				ctx.draw(&shapes::text("drag 3d model files into this window"))?;

				ctx.push(&gfx::t()
					.t2(vec2!(0, 22))
					.s2(vec2!(0.8))
				, |ctx| {

					ctx.draw(&shapes::text("H: help"))?;

					return Ok(());

				})?;

			}

			return Ok(());

		})?;

		ctx.push(&gfx::t()
			.t2(ctx.coord(gfx::Origin::BottomLeft) + vec2!(24, -24))
			.s2(vec2!(0.8))
		, |ctx| {

			if self.helping {

				let msg = [
					"W/A/S/D:  move",
					"<drag>:   rotate",
					"<scroll>: scale",
					"<space>:  reset",
					"L:        wireframe",
					"F:        fullscreen",
					"N:        normal",
					"<esc>:    quit",
				];

				for (i, m) in msg
					.iter()
					.rev()
					.enumerate()
				{
					ctx.draw_t(&gfx::t()
						.t2(vec2!(0, i as i32 * -18))
					, &shapes::text(m)
						.align(gfx::Origin::BottomLeft)
					)?;
				}

			}

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Viewer>() {
		println!("{}", err);
	}

}

