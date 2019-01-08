// wengwengweng

//! Common Math Functions

pub(crate) mod vec;
pub(crate) mod mat;

use crate::*;

/// get a random number from 0-1
pub fn rand() -> f32 {
	return rand::random::<f32>();
}

/// get a random Vec2 from 0-1
pub fn rand_vec2() -> Vec2 {
	return vec2!(rand(), rand());
}

/// get a random element from a Vec
pub fn rand_from<T>(v: &Vec<T>) -> &T {
	return &v[(rand() * v.len() as f32) as usize];
}

