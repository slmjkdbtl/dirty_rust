// wengwengweng

use super::*;
use crate::app::*;

pub enum Primitive<'a> {
	Texture(&'a gfx::Texture),
}

pub struct Particle {
	timer: Timer,
	pos: Vec2,
	acc: Vec2,
	vel: Vec2,
	speed: f32,
	color_start: Color,
	color_end: Color,
	size_start: f32,
	size_end: f32,
	dead: bool,
}

impl Particle {

	fn update(&mut self, dt: f32) {

		if self.timer.tick(dt) {
			self.dead = true;
			return;
		}

		self.vel += self.acc * dt;
		self.pos += self.vel * self.speed * dt;

	}

}

impl gfx::Drawable for Particle {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let t = self.timer.progress();
		let size = self.size_start.lerp(self.size_end, t);
		let color = self.color_start.lerp(self.color_end, t);

		ctx.draw(
			&shapes::rect(self.pos - vec2!(size) / 2.0, self.pos + vec2!(size) / 2.0)
				.fill(color)
		)?;

		return Ok(());

	}

}

pub struct ParticleConf {
	pos: Vec2,
	offset: (Vec2, Vec2),
	life: (f32, f32),
	color_start: (Color, Color),
	color_end: Color,
	speed: (f32, f32),
	acc: (Vec2, Vec2),
	vel: (Vec2, Vec2),
	rate: (f32, f32),
	size_start: (f32, f32),
	size_end: (f32, f32),
	num: (usize, usize),
}

impl Default for ParticleConf {
	fn default() -> Self {
		return Self {
			pos: vec2!(),
			offset: (vec2!(0), vec2!(0)),
			life: (3.0, 5.0),
			color_start: (rgba!(0.9, 0.3, 0, 0.4), rgba!(1, 0.3, 0, 0.5)),
			color_end: rgba!(0.2, 0.2, 1, 0),
			speed: (96.0, 240.0),
			acc: (vec2!(0, 2), vec2!(0, 2)),
			vel: (vec2!(-0.5, -1.2), vec2!(0.5, -1.2)),
			rate: (0.02, 0.05),
			size_start: (12.0, 36.0),
			size_end: (0.0, 0.0),
			num: (12, 16),
		};
	}
}

pub struct ParticleSystemBuilder {
	conf: ParticleConf,
}

impl ParticleSystemBuilder {
	pub fn pos(mut self, p: Vec2) -> Self {
		self.conf.pos = p;
		return self;
	}
	pub fn offset(mut self, min: Vec2, max: Vec2) -> Self {
		self.conf.offset = (min, max);
		return self;
	}
	pub fn life(mut self, min: f32, max: f32) -> Self {
		self.conf.life = (min, max);
		return self;
	}
	pub fn color_start(mut self, min: Color, max: Color) -> Self {
		self.conf.color_start = (min, max);
		return self;
	}
	pub fn color_end(mut self, c: Color) -> Self {
		self.conf.color_end = c;
		return self;
	}
	pub fn speed(mut self, min: f32, max: f32) -> Self {
		self.conf.speed = (min, max);
		return self;
	}
	pub fn acc(mut self, min: Vec2, max: Vec2) -> Self {
		self.conf.acc = (min, max);
		return self;
	}
	pub fn vel(mut self, min: Vec2, max: Vec2) -> Self {
		self.conf.vel = (min, max);
		return self;
	}
	pub fn rate(mut self, min: f32, max: f32) -> Self {
		self.conf.rate = (min, max);
		return self;
	}
	pub fn size_start(mut self, min: f32, max: f32) -> Self {
		self.conf.size_start = (min, max);
		return self;
	}
	pub fn size_end(mut self, min: f32, max: f32) -> Self {
		self.conf.size_end = (min, max);
		return self;
	}
	pub fn num(mut self, min: usize, max: usize) -> Self {
		self.conf.num = (min, max);
		return self;
	}
	pub fn build(self) -> ParticleSystem {
		return ParticleSystem {
			particles: Vec::with_capacity(256),
			spawn_timer: Timer::new(rand_t(self.conf.rate)),
			conf: self.conf,
			paused: false,
		};
	}
}

pub struct ParticleSystem {
	particles: Vec<Particle>,
	conf: ParticleConf,
	spawn_timer: Timer,
	paused: bool,
}

impl ParticleSystem {

	pub fn builder() -> ParticleSystemBuilder {
		return ParticleSystemBuilder {
			conf: ParticleConf::default(),
		};
	}

	pub fn update(&mut self, dt: f32) {

		if !self.paused {
			if self.spawn_timer.tick(dt) {
				self.spawn_timer.reset_to(rand_t(self.conf.rate));
				self.emit();
			}
		}

		for p in &mut self.particles {
			p.update(dt);
		}

		self.particles.retain(|p| !p.dead);

	}

	pub fn count(&self) -> usize {
		return self.particles.len();
	}

	pub fn active(&self) -> bool {
		return !self.paused;
	}

	pub fn pause(&mut self) {
		self.paused = true;
	}

	pub fn start(&mut self) {
		self.paused = false;
	}

	pub fn set_pos(&mut self, p: Vec2) {
		self.conf.pos = p;
	}

	pub fn emit(&mut self) {

		for _ in 0..rand_t(self.conf.num) {


			let p = Particle {
				timer: Timer::new(rand_t(self.conf.life)),
				pos: self.conf.pos + rand_t(self.conf.offset),
				acc: rand_t(self.conf.acc),
				vel: rand_t(self.conf.vel),
				speed: rand_t(self.conf.speed),
				color_start: rand_t(self.conf.color_start),
				color_end: self.conf.color_end,
				size_start: rand_t(self.conf.size_start),
				size_end: rand_t(self.conf.size_end),
				dead: false,
			};

			self.particles.push(p);

		}

	}

}

impl gfx::Drawable for ParticleSystem {
	fn draw(&self, ctx: &mut Ctx) -> Result<()> {
		for p in &self.particles {
			ctx.draw(p)?;
		}
		return Ok(());
	}
}

