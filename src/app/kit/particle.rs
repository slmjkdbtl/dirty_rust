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
	color: (Color, Color),
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

		ctx.draw(
			&shapes::rect(self.pos - vec2!(1), self.pos + vec2!(1))
				.fill(self.color.0)
		)?;

		return Ok(());

	}

}

pub struct ParticleConf {
	pos: Vec2,
	offset: (Vec2, Vec2),
	life: (f32, f32),
	color: (Color, Color),
	speed: (f32, f32),
	acc: (Vec2, Vec2),
	vel: (Vec2, Vec2),
	rate: (f32, f32),
}

impl Default for ParticleConf {
	fn default() -> Self {
		return Self {
			pos: vec2!(),
			offset: (vec2!(0), vec2!(0)),
			life: (1.6, 2.4),
			color: (rgba!(1), rgba!(1, 1, 1, 0)),
			speed: (96.0, 120.0),
			acc: (vec2!(0, 2), vec2!(0, 2)),
			vel: (vec2!(-1, -1), vec2!(1, -1)),
			rate: (0.02, 0.05),
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
	pub fn acc(mut self, min: Vec2, max: Vec2) -> Self {
		self.conf.acc = (min, max);
		return self;
	}
	pub fn speed(mut self, min: f32, max: f32) -> Self {
		self.conf.speed = (min, max);
		return self;
	}
	pub fn color(mut self, c1: Color, c2: Color,) -> Self {
		self.conf.color = (c1, c2);
		return self;
	}
	pub fn build(self) -> ParticleSystem {
		return ParticleSystem {
			particles: Vec::with_capacity(256),
			spawn_timer: Timer::new(rand(self.conf.rate)),
			conf: self.conf,
			emitting: true,
		};
	}
}

pub struct ParticleSystem {
	particles: Vec<Particle>,
	conf: ParticleConf,
	spawn_timer: Timer,
	emitting: bool,
}

impl ParticleSystem {

	pub fn builder() -> ParticleSystemBuilder {
		return ParticleSystemBuilder {
			conf: ParticleConf::default(),
		};
	}

	pub fn update(&mut self, dt: f32) {

		if self.emitting {
			if self.spawn_timer.tick(dt) {
				self.spawn_timer.reset_to(rand(self.conf.rate));
				self.spawn();
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

	pub fn pause(&mut self) {
		self.emitting = false;
	}

	pub fn start(&mut self) {
		self.emitting = true;
	}

	pub fn set_pos(&mut self, p: Vec2) {
		self.conf.pos = p;
	}

	fn spawn(&mut self) {

		let p = Particle {
			timer: Timer::new(rand(self.conf.life)),
			pos: self.conf.pos + rand(self.conf.offset),
			acc: rand(self.conf.acc),
			vel: rand(self.conf.vel),
			speed: rand(self.conf.speed),
			color: self.conf.color,
			dead: false,
		};

		self.particles.push(p);

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

