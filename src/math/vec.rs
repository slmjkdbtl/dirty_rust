// wengwengweng

use std::fmt;
use std::ops;
use std::mem;

use derive_more::*;

macro_rules! nested_macro {

	($($body:tt)*) => {

		macro_rules! __nested_macro {
			$($body)*
		}

		__nested_macro!($);

	};

}

macro_rules! gen_vec {

	($name:ident($sname:ident) -> ($($member:ident),+): [$type:ty; $count:expr], ($($default:expr),+)) => {

		nested_macro! {

			($d:tt) => {

				#[macro_export]
				#[allow(missing_docs)]
				macro_rules! $sname {

					() => {
						crate::math::$name::default();
					};

					($v:expr) => {
						crate::math::$name { $($member: $v as $type),+ };
					};

					($d($v:expr),*$d(,)?) => {
						crate::math::$name::new($d($v as $type),*)
					};

				}

			}

		}

		#[allow(missing_docs)]
		#[derive(Copy, Clone, PartialEq, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, From, Into, Debug)]
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

		}

		impl ops::Mul<$name> for $name {

			type Output = Self;

			fn mul(self, other: Self) -> Self {
				return Self {
					$($member: self.$member * other.$member),+
				};
			}

		}

		impl Into<[$type; $count]> for $name {
			fn into(self) -> [$type; $count] {
				return [
					$(
						self.$member,
					)+
				];
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

	}

}

macro_rules! mix {

	($one:ident($($mem_one:ident),+): $type_one:ty, $two:ident($($mem_two:ident),+): $type_two:ty) => {

		impl From<$one> for $two {
			fn from(s: $one) -> Self {
				return Self {
					$(
						$mem_two: s.$mem_one as $type_two
					),+
				};
			}
		}

		impl Into<$one> for $two {
			fn into(self) -> $one {
				return $one {
					$(
						$mem_one: self.$mem_two as $type_one
					),+
				};
			}
		}

	}

}

gen_vec!(Vec2(vec2) -> (x, y): [f32; 2], (0, 0));
gen_vec!(Vec3(vec3) -> (x, y, z): [f32; 3], (0, 0, 0));
gen_vec!(Vec4(vec4) -> (x, y, z, w): [f32; 4], (0, 0, 0, 0));
gen_vec!(Color(color) -> (r, g, b, a): [f32; 4], (1, 1, 1, 1));
gen_vec!(Quad(quad) -> (x, y, w, h): [f32; 4], (0, 0, 0, 0));

mix!(Vec4(x, y, z, w): f32, Color(r, g, b, a): f32);
mix!(Vec4(x, y, z, w): f32, Quad(x, y, w, h): f32);

impl Vec2 {

	/// get a vector from given angle
	pub fn from_angle(angle: f32) -> Self {
		return vec2!(angle.cos(), angle.sin());
	}

	/// get vector magnitude
	pub fn mag(&self) -> f32 {
		return (self.x * self.x + self.y * self.y).sqrt();
	}

	/// normalize vector
	pub fn normalize(&self) -> Self {
		return self.clone() / self.mag();
	}

	/// get vector normal
	pub fn normal(&self) -> Self {
		return vec2!(self.y, -self.x);
	}

	/// dot product of 2 vectors
	pub fn dot(self, other: Self) -> f32 {
		return self.x * other.x + self.y * other.y;
	}

	/// get angle between 2 vectors
	pub fn angle(self, other: Self) -> f32 {
		return (other.y - self.y).atan2(other.x - self.x);
	}

	/// get distance between another vector
	pub fn dis(self, other: Self) -> f32 {
		return ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
	}

}

impl Vec3 {

	/// get vector magnitude
	pub fn mag(&self) -> f32 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
	}

	/// normalize vector
	pub fn normalize(&self) -> Self {
		return self.clone() / self.mag();
	}

	/// dot product
	pub fn dot(self, other: Self) -> f32 {
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}

	/// cross product
	pub fn cross(self, other: Self) -> Self {
		return vec3!(
			(self.y * other.z) - (self.z * other.y),
			(self.z * other.x) - (self.x * other.z),
			(self.x * other.y) - (self.y * other.x)
        );
	}

}

impl Color {

	pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0, };
	pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0, };
	pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0, };
	pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0, };
	pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0, };
	pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0, };
	pub const PURPLE: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0, };
	pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0, };
	pub const NONE: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0, };

	pub fn from_hex(hex: u32, opacity: f32) -> Self {

		let r = (hex >> 16) as f32 / 255.0;
		let g = ((hex >> 8) & 0xff) as f32 / 255.0;
		let b = (hex & 0xff) as f32 / 255.0;

		return color!(r, g, b, opacity);

	}

	pub fn to_rgba(&self) -> [u8; 4] {

		return [
			(self.r * 255.0) as u8,
			(self.g * 255.0) as u8,
			(self.b * 255.0) as u8,
			(self.a * 255.0) as u8,
		];

	}

	pub fn brightness(&self) -> f32 {
		return (self.r + self.g + self.b) / 3.0;
	}

	pub fn lighten(self, v: f32) -> Self {
		return self + color!(v, v, v, 0);
	}

	pub fn darken(self, v: f32) -> Self {
		return self - color!(v, v, v, 0);
	}

}

impl From<Color> for [u8; 4] {
	fn from(c: Color) -> [u8; 4] {
		return [
			(c.r * 255.0) as u8,
			(c.g * 255.0) as u8,
			(c.b * 255.0) as u8,
			(c.a * 255.0) as u8,
		];
	}
}

