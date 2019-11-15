// wengwengweng

use std::collections::HashMap;

use crate::*;
use app::*;

#[derive(Clone)]
pub struct Sprite {
	tex: gfx::Texture,
	frames: Vec<Quad>,
	cur_frame: usize,
	offset: Vec2,
	anims: HashMap<String, Anim>,
	cur_anim: Option<String>,
	timer: f32,
	speed: f32,
}

impl Sprite {

	pub fn from_tex(ctx: &app::Ctx, tex: gfx::Texture) -> Self {
		return Self {
			tex: tex,
			frames: vec![quad!(0, 0, 1, 1)],
			cur_frame: 0,
			offset: ctx.conf().origin.as_pt(),
			anims: HashMap::new(),
			cur_anim: None,
			timer: 0.0,
			speed: 0.1,
		};
	}

	pub fn from_bytes(ctx: &app::Ctx, b: &[u8]) -> Result<Self> {
		return Ok(Self::from_tex(ctx, gfx::Texture::from_bytes(ctx, b)?));
	}

	#[cfg(feature = "ase")]
	pub fn load_ase(&mut self, json: &str) -> Result<()> {

		let data = ase::SpriteData::from_json(json)?;

		self.frames = data.frames;

		for (name, anim) in data.anims {
			self.add_anim(&name, anim.from, anim.to, true);
		}

		return Ok(());

	}

	#[cfg(feature = "ase")]
	pub fn from_ase(ctx: &app::Ctx, tex: gfx::Texture, json: &str) -> Result<Self> {

		let mut sprite = Self::from_tex(ctx, tex);

		sprite.load_ase(json)?;

		return Ok(sprite);

	}

	#[cfg(feature = "ase")]
	pub fn from_bytes_ase(ctx: &app::Ctx, img: &[u8], json: &str) -> Result<Self> {
		return Self::from_ase(ctx, gfx::Texture::from_bytes(ctx, img)?, json);
	}

	pub fn width(&self) -> f32 {
		return self.frames[self.cur_frame].w * self.tex.width() as f32;
	}

	pub fn height(&self) -> f32 {
		return self.frames[self.cur_frame].h * self.tex.height() as f32;
	}

	pub fn tex_width(&self) -> i32 {
		return self.tex.width();
	}

	pub fn tex_height(&self) -> i32 {
		return self.tex.height();
	}

	pub fn slice(&mut self, x: u8, y: u8) {

		let w = 1.0 / x as f32;
		let h = 1.0 / y as f32;

		self.frames.clear();

		for i in 0..x as usize {
			for j in 0..y as usize {
				self.frames.push(quad!(i as f32 * w, j as f32 * h, w, h));
			}
		}

	}

	pub fn add_anim(&mut self, name: &str, from: usize, to: usize, looping: bool) {
		self.anims.insert(name.to_owned(), Anim {
			from: from,
			to: to,
			looping: looping,
		});
	}

	pub fn next(&mut self) {
		if self.cur_frame < self.frames.len() - 1 {
			self.cur_frame += 1;
		}
	}

	pub fn prev(&mut self) {
		if self.cur_frame > 0 {
			self.cur_frame -= 1;
		}
	}

	pub fn set_offset(&mut self, o: Vec2) {
		self.offset = o;
	}

	pub fn set_speed(&mut self, s: f32) {
		self.speed = s;
	}

	pub fn verts(&self) -> Vec<Vec2> {

		let w = self.width();
		let h = self.height();
		let offset = self.offset * vec2!(w, h) * -0.5;

		return vec![
			vec2!(-w / 2.0, -h / 2.0) + offset,
			vec2!(w / 2.0, -h / 2.0) + offset,
			vec2!(w / 2.0, h/ 2.0) + offset,
			vec2!(-w / 2.0, h/ 2.0) + offset,
		];

	}

	pub fn play(&mut self, name: &str) {

		self.cur_anim = Some(name.to_owned());

		if let Some(anim) = self.anims.get(name) {
			self.cur_frame = anim.from;
		}

	}

	pub fn update(&mut self, dt: f32,) {

		let anim = match &self.cur_anim {
			Some(cur_anim) => {
				match self.anims.get(cur_anim) {
					Some(anim) => anim,
					None => return
				}
			},
			None => return
		};

		self.timer += dt;

		if self.timer >= self.speed {
			if anim.from < anim.to {
				if self.cur_frame >= anim.to {
					if anim.looping {
						self.cur_frame = anim.from;
					} else {
						self.cur_anim = None;
					}
				} else {
					self.cur_frame += 1;
				}
			}
			self.timer = 0.0;
		}

	}

	pub fn shape(&self) -> shapes::Sprite {
		return shapes::sprite(&self.tex)
			.quad(self.frames[self.cur_frame])
			.offset(self.offset)
			;
	}

}

impl gfx::Drawable for Sprite {
	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {
		return ctx.draw(&self.shape());
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Anim {
	pub from: usize,
	pub to: usize,
	pub looping: bool,
}

