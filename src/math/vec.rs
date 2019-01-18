// wengwengweng

//! Vectors, Matrixes

use core::ops;
use std::fmt;

use crate::*;

macro_rules! gen_vec {

	($name:ident($sname:ident) -> ($($member:ident),+): $type:ty, ($($default:expr),+)) => {

		nested_macro! {

			($d:tt) => {

				#[macro_export]
				macro_rules! $sname {

					() => {
						$name::default();
					};

					($v:expr) => {
						$name::all($v as $type);
					};

					($d($v:expr),*) => {
						$name::new($d($v as $type),*)
					};

				}

			}

		}

		#[allow(missing_docs)]
		#[derive(Debug, Copy, Clone, PartialEq)]
		pub struct $name {
			$(
			pub $member: $type
			),+
		}

		impl $name {

			/// initialize with specifying all fields
			pub fn new($($member: $type,)+) -> Self {
				return Self {
					$($member: $member),+
				};
			}

			/// initialize with same value on all fields
			pub fn all(x: $type) -> Self {
				return Self {
					$($member: x),+
				}
			}

			/// initialize with random values
			pub fn rand() -> Self {
				return Self {
					$($member: math::rand()),+
				}
			}

		}

		impl Default for $name {
			fn default() -> Self {
				return $sname!($($default),+);
			}
		}

		impl fmt::Display for $name {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				return write!(f, "{}({})", stringify!($sname), vec![$(format!("{}", self.$member)),+].join(", "));
			}
		}

		impl ops::Add for $name {
			type Output = $name;
			fn add(self, other: $name) -> $name {
				return $name {
					$($member: self.$member + other.$member,)+
				}
			}
		}

		impl ops::Sub for $name {
			type Output = $name;
			fn sub(self, other: $name) -> $name {
				return $name {
					$($member: self.$member - other.$member,)+
				}
			}
		}

		impl ops::Mul<$name> for $name {
			type Output = $name;
			fn mul(self, other: $name) -> $name {
				return $name {
					$($member: self.$member * other.$member,)+
				}
			}
		}

		impl ops::Mul<f32> for $name {
			type Output = $name;
			fn mul(self, other: f32) -> $name {
				return $name {
					$($member: self.$member * other,)+
				}
			}
		}

		impl ops::Mul<i32> for $name {
			type Output = $name;
			fn mul(self, other: i32) -> $name {
				return self * (other as f32);
			}
		}

		impl ops::Div<$name> for $name {
			type Output = $name;
			fn div(self, other: $name) -> $name {
				return $name {
					$($member: self.$member / other.$member,)+
				}
			}
		}

		impl ops::Div<f32> for $name {
			type Output = $name;
			fn div(self, other: f32) -> $name {
				return $name {
					$($member: self.$member / other,)+
				}
			}
		}

		impl ops::Div<i32> for $name {
			type Output = $name;
			fn div(self, other: i32) -> $name {
				return self / (other as f32);
			}
		}

	}

}

gen_vec!(Vec2(vec2) -> (x, y): f32, (0, 0));
gen_vec!(Vec3(vec3) -> (x, y, z): f32, (0, 0, 0));
gen_vec!(Vec4(vec4) -> (x, y, z, w): f32, (0, 0, 0, 0));
gen_vec!(Color(color) -> (r, g, b, a): f32, (1, 1, 1, 1));
gen_vec!(Rect(rect) -> (x, y, w, h): f32, (0, 0, 0, 0));

impl Vec2 {

	/// get a vector from given angle
	pub fn from_angle(angle: f32) -> Self {
		return vec2!(angle.cos(), angle.sin());
	}

	/// normalize vector
	pub fn unit(&self) -> Self {
		return self.clone() / self.mag();
	}

	/// get vector normal
	pub fn normal(&self) -> Self {
		return vec2!(self.y, -self.x);
	}

	/// dot product of 2 vectors
	pub fn dot(&self, other: Vec2) -> f32 {
		return self.x * other.x + self.y * other.y;
	}

	/// get angle between 2 vectors
	pub fn angle(&self, other: Vec2) -> f32 {
		return (other.y - self.y).atan2(other.x - self.x);
	}

	/// get vector magnitude
	pub fn mag(&self) -> f32 {
		return (self.x * self.x + self.y * self.y).sqrt();
	}

}

