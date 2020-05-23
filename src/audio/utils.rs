// wengwengweng

pub fn i16_to_f32(n: i16) -> f32 {
	return n as f32 / i16::MAX as f32;
}

pub fn f32_to_i16(n: f32) -> i16 {
	return (n * i16::MAX as f32) as i16;
}

pub fn f32_to_u16(n: f32) -> u16 {
	return ((n * 0.5 + 0.5) * u16::MAX as f32) as u16;
}

