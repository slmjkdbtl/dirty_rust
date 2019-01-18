// wengwengweng

//! Common Math Functions

pub(crate) mod vec;
pub(crate) mod mat;

/// get a random number from 0-1
pub fn rand() -> f32 {
	return rand::random::<f32>();
}

