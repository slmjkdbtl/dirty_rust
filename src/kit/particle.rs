// wengwengweng

use std::time::Duration;

use crate::*;
use super::*;
use timer::*;
use math::*;

#[derive(Clone)]
pub struct Particle {
	timer: Timer,
	pos: Vec2,
	color: Color,
	size: Vec2,
	acc: Vec2,
	vel: Vec2,
	speed: f32,
	color_start: Color,
	color_end: Color,
	size_start: Vec2,
	size_end: Vec2,
	dead: bool,
}

impl Particle {

	fn update(&mut self, dt: Duration) {

		if self.timer.tick(dt) {
			self.dead = true;
			return;
		}

		let t = self.timer.progress();
		let dt = dt.as_secs_f32();

		self.vel += self.acc * dt;
		self.pos += self.vel * self.speed * dt;
		self.size = self.size_start.lerp(self.size_end, t);
		self.color = self.color_start.lerp(self.color_end, t);

	}

	pub fn pos(&self) -> Vec2 {
		return self.pos;
	}

	pub fn size(&self) -> Vec2 {
		return self.size;
	}

	pub fn color(&self) -> Vec2 {
		return self.pos;
	}

}

#[derive(Clone)]
pub struct ParticleConf {
	pub offset: (Vec2, Vec2),
	pub life: (f32, f32),
	pub color_start: (Color, Color),
	pub color_end: Color,
	pub speed: (f32, f32),
	pub acc: (Vec2, Vec2),
	pub vel: (Vec2, Vec2),
	pub rate: (f32, f32),
	pub size_start: (Vec2, Vec2),
	pub size_end: (Vec2, Vec2),
	pub num: (usize, usize),
	pub max: usize,
}

#[derive(Clone)]
pub struct ParticleSystem {
	pos: Vec2,
	particles: Vec<Particle>,
	conf: ParticleConf,
	spawn_timer: Option<Timer>,
	paused: bool,
}

impl ParticleSystem {

	pub fn from_conf(conf: ParticleConf) -> Self {

		let rate = rand_t(conf.rate);
		let timer = if rate == 0.0 {
			None
		} else {
			Some(Timer::from_secs(1.0 / rate))
		};

		return Self {
			pos: vec2!(),
			spawn_timer: timer,
			particles: Vec::with_capacity(256),
			paused: false,
			conf: conf,
		};

	}

	pub fn update(&mut self, dt: Duration) {

		if let Some(timer) = &mut self.spawn_timer {
			if !self.paused {
				if timer.tick(dt) {
					let rate = rand_t(self.conf.rate);
					if rate == 0.0 {
						self.spawn_timer = None;
					} else {
						timer.reset_to_secs(1.0 / rate)
					};
					self.emit();
				}
			}
		} else {
			let rate = rand_t(self.conf.rate);
			if rate != 0.0 {
				self.spawn_timer = Some(Timer::from_secs(1.0 / rate));
			}
		}

		for p in &mut self.particles {
			p.update(dt);
		}

		self.particles.retain(|p| !p.dead);

	}

	pub fn particles(&self) -> &[Particle] {
		return &self.particles;
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

	pub fn conf_mut(&mut self) -> &mut ParticleConf {
		return &mut self.conf;
	}

	pub fn conf(&self) -> &ParticleConf {
		return &self.conf;
	}

	pub fn set_pos(&mut self, p: Vec2) {
		self.pos = p;
	}

	pub fn pos(&self) -> Vec2 {
		return self.pos;
	}

	pub fn emit(&mut self) {

		for _ in 0..rand_t(self.conf.num) {

			if self.count() <= self.conf.max {

				let color_start = rand_t(self.conf.color_start);
				let size_start = rand_t(self.conf.size_start);

				let p = Particle {
					timer: Timer::from_secs(rand_t(self.conf.life)),
					pos: self.pos + rand_t(self.conf.offset),
					color: color_start,
					size: size_start,
					acc: rand_t(self.conf.acc),
					vel: rand_t(self.conf.vel),
					speed: rand_t(self.conf.speed),
					color_start: color_start,
					color_end: self.conf.color_end,
					size_start: size_start,
					size_end: rand_t(self.conf.size_end),
					dead: false,
				};

				self.particles.push(p);

			}

		}

	}

}

