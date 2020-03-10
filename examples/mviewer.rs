// wengwengweng

// TODO: handle load failure

#![feature(clamp)]

use std::path::Path;

use dirty::*;
use task::*;
use math::*;
use input::Key;
use input::Mouse;

struct Viewer {
	shader: gfx::Shader3D<()>,
	model: Option<gfx::Model>,
	rot: Vec2,
	pos: Vec2,
	scale: f32,
	resetting: bool,
	loader: Task<Result<gfx::ModelData>>,
	draw_wireframe: bool,
	run_anim: bool,
	draw_bound: bool,
	helping: bool,
}

#[derive(Clone)]
struct PixUniform {
	resolution: Vec2,
	size: f32,
}

impl gfx::Uniform for PixUniform {
	fn values(&self) -> gfx::UniformValues {
		return hmap![
			"u_resolution" => &self.resolution,
			"u_size" => &self.size,
		];
	}
}

fn load_file(path: impl AsRef<Path>) -> Task<Result<gfx::ModelData>> {

	let path = path.as_ref().to_owned();

	return Task::exec(move || {
		return gfx::Model::load_file(path);
	});

}

impl Viewer {

	fn update_model(&mut self, model: gfx::Model) {

		self.resetting = true;
		self.model = Some(model);

	}

	fn load_file(&mut self, path: impl AsRef<Path>) {

		self.loader = load_file(path);
		self.model = None;
		self.scale = 0.0;
		self.resetting = false;

	}

}

impl State for Viewer {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		return Ok(Self {
			model: None,
			pos: vec2!(0),
			shader: gfx::Shader3D::from_frag(ctx, include_str!("res/normal.frag"))?,
			rot: vec2!(0),
			resetting: false,
			loader: load_file("examples/res/truck.obj"),
			draw_wireframe: false,
			draw_bound: false,
			run_anim: true,
			helping: false,
			scale: 0.0,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				let mods = ctx.key_mods();

				match *k {
					Key::F => ctx.toggle_fullscreen(),
					Key::Esc => ctx.quit(),
					Key::Q if mods.meta => ctx.quit(),
					Key::Space => self.resetting = true,
					Key::L => self.draw_wireframe = !self.draw_wireframe,
					Key::B => self.draw_bound = !self.draw_bound,
					Key::H => self.helping = !self.helping,
					Key::T => self.run_anim = !self.run_anim,
					_ => {},
				}

			},

			Scroll(s, phase) => {

				if let input::ScrollPhase::Solid = phase {
					self.resetting = false;
				}

				if let Some(model) = &self.model {

					if !self.resetting {

						let (min, max) = model.bbox();
						let size = (max - min).mag();
						let orig_scale = 480.0 / size;

						self.scale += s.y * (1.0 / size);
						self.scale = self.scale.clamp(orig_scale * 0.1, orig_scale * 3.2);

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

	fn update(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.set_title(&format!("FPS: {} DCS: {}", ctx.fps(), ctx.draw_calls()));

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
				self.pos.x += move_speed * ctx.dt();
			}

			if ctx.key_down(Key::D) {
				self.resetting = false;
				self.pos.x -= move_speed * ctx.dt();
			}

			if ctx.key_down(Key::W) {
				self.resetting = false;
				self.pos.y -= move_speed * ctx.dt();
			}

			if ctx.key_down(Key::S) {
				self.resetting = false;
				self.pos.y += move_speed * ctx.dt();
			}

			if self.resetting {

				let (min, max) = model.bbox();
				let size = (max - min).mag();

				let dest_rot = vec2!(0);
				let dest_pos = vec2!(0);
				let dest_scale = 480.0 / size;
				let t = ctx.dt() * 4.0;

				self.rot = self.rot.lerp(dest_rot, t);
				self.pos = self.pos.lerp(dest_pos, t);
				self.scale = self.scale.lerp(dest_scale, t);

			}

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		if let Some(model) = &self.model {

			let center = model.center();

			ctx.push(mat4!()
				.t2(self.pos)
				.s3(vec3!(self.scale))
				.ry(self.rot.x.to_radians())
				.rx(self.rot.y.to_radians())
				.t3(-center)
			, |ctx| {

				let t = if self.run_anim {
					let anim_len = model.anim_len();
					let t = ctx.time();
					t - f32::floor(t / anim_len) * anim_len
				} else {
					0.0
				};

				ctx.draw_3d_with(&self.shader, &(), |ctx| {
					ctx.draw(
						&shapes::model(&model)
// 							.wireframe(self.draw_wireframe)
							.time(t)
					)?;
					return Ok(());
				})?;

				if self.draw_bound {
					let (min, max) = model.bbox();
					ctx.draw(&shapes::rect3d(min, max))?;
				}

				return Ok(());

			})?;

		}

		ctx.push(mat4!()
			.t2(ctx.coord(gfx::Origin::TopLeft) + vec2!(24, -24))
			.tz(320.0)
		, |ctx| {

			if self.loader.phase() == TaskPhase::Working {

				ctx.draw(
					&shapes::text("loading...")
						.align(gfx::Origin::TopLeft)
				)?;

			} else {

				ctx.draw(
					&shapes::text("drag 3d model files into this window")
						.align(gfx::Origin::TopLeft)
				)?;

				ctx.push(mat4!()
					.ty(-22.0)
					.s2(vec2!(0.8))
				, |ctx| {

					ctx.draw(
						&shapes::text("H: help")
							.align(gfx::Origin::TopLeft)
					)?;

					return Ok(());

				})?;

			}

			return Ok(());

		})?;

		ctx.push(mat4!()
			.t2(ctx.coord(gfx::Origin::BottomLeft) + vec2!(24, 24))
			.s2(vec2!(0.8))
			.tz(320.0)
		, |ctx| {

			if self.helping {

				let msg = [
					"W/A/S/D:  move",
					"<drag>:   rotate",
					"<scroll>: scale",
					"<space>:  reset",
					"L:        wireframe",
					"B:        bound",
					"F:        fullscreen",
					"T:        anim",
					"<esc>:    quit",
				];

				for (i, m) in msg
					.iter()
					.rev()
					.enumerate()
				{
					ctx.draw_t(mat4!()
						.t2(vec2!(0, i as i32 * 18))
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

	if let Err(err) = launcher()
		.resizable(true)
		.run::<Viewer>() {
		println!("{}", err);
	}

}

