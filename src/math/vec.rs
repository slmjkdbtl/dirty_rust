// wengwengweng

//! Vectors, Matrixes

use core::ops;
use std::fmt;

macro_rules! gen_vec {

	($name:ident($sname:ident) -> ($($member:ident),+): $type:ty) => {

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

		/// $name
		#[derive(Debug, Copy, Clone, Default, PartialEq)]
		pub struct $name {
			$(
			/// $member
			pub $member: $type
			),+
		}

		impl $name {

			/// initialize $name specifying all fields
			pub fn new($($member: $type,)+) -> Self {
				return Self {
					$($member: $member,)+
				};
			}

			/// initialize $name with same values in all fields
			pub fn all(x: $type) -> Self {
				return Self {
					$($member: x,)+
				}
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

gen_vec!(Vec2(vec2) -> (x, y): f32);
gen_vec!(Vec3(vec3) -> (x, y, z): f32);
gen_vec!(Vec4(vec4) -> (x, y, z, w): f32);
gen_vec!(Color(color) -> (r, g, b, a): f32);
gen_vec!(Rect(rect) -> (x, y, w, h): f32);

