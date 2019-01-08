// wengwengweng

/// Common Math Functions

pub(crate) mod vec;
pub(crate) mod mat;

use crate::*;

pub fn rand() -> f32 {
	return rand::random::<f32>();
}

pub fn rand_vec2() -> Vec2 {
	return vec2!(rand(), rand());
}

pub fn rand_from<T>(v: &Vec<T>) -> &T {
	return &v[(rand() * v.len() as f32) as usize];
}

