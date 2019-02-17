// wengwengweng

use sock::*;
use sock::ecs::*;
use crate::comps::*;

pub struct FlowerInputSys;

impl System for FlowerInputSys {

	fn filter(&self) -> Filter {
		return filter![Flower, Vel];
	}

	fn each(&mut self, e: &mut Entity) {

		let mut f = e.get::<Flower>();
		let mut v = e.get::<Vel>();

		match f.player {

			Player::One => {

				if window::key_down(Key::W) {
					v.pos = vec2!(0, -1) * f.speed;
					f.active = true;
				} else if window::key_down(Key::S) {
					v.pos = vec2!(0, 1) * f.speed;
					f.active = true;
				} else if window::key_down(Key::A) {
					v.pos = vec2!(-1, 0) * f.speed;
					f.active = true;
				} else if window::key_down(Key::D) {
					v.pos = vec2!(1, 0) * f.speed;
					f.active = true;
				} else {
					v.pos = vec2!(0);
					f.active = false;
				}

				if window::key_down(Key::Q) {
					v.rot = -1.0 * f.rot_speed;
					f.active = true;
					f.energy += 1;
				} else if window::key_down(Key::E) {
					v.rot = 1.0 * f.rot_speed;
					f.active = true;
					f.energy += 1;
				} else {
					v.rot = 0.0;
				}

			}

			Player::Two => {

				if window::key_down(Key::I) {
					v.pos = vec2!(0, -1) * f.speed;
					f.active = true;
				} else if window::key_down(Key::K) {
					v.pos = vec2!(0, 1) * f.speed;
					f.active = true;
				} else if window::key_down(Key::J) {
					v.pos = vec2!(-1, 0) * f.speed;
					f.active = true;
				} else if window::key_down(Key::L) {
					v.pos = vec2!(1, 0) * f.speed;
					f.active = true;
				} else {
					v.pos = vec2!(0);
					f.active = false;
				}

				if window::key_down(Key::U) {
					v.rot = -1.0 * f.rot_speed;
					f.active = true;
					f.energy += 1;
				} else if window::key_down(Key::O) {
					v.rot = 1.0 * f.rot_speed;
					f.active = true;
					f.energy += 1;
				} else {
					v.rot = 0.0;
				}

			}

		}

		e.set::<Vel>(v);
		e.set::<Flower>(f);

	}

}

