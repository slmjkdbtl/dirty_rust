// wengwengweng

use std::ops;
use std::fmt;

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

	($name:ident($sname:ident) -> ($($member:ident),+): $type:ty, ($($default:expr),+)) => {

		nested_macro! {

			($d:tt) => {

				#[macro_export]
				#[allow(missing_docs)]
				macro_rules! $sname {

					() => {
						crate::math::$name::default();
					};

					($v:expr) => {
						crate::math::$name::all($v as $type);
					};

					($d($v:expr),*) => {
						crate::math::$name::new($d($v as $type),*)
					};

				}

			}

		}

		#[allow(missing_docs)]
		#[derive(Copy, Clone, PartialEq, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, From, Into)]
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

		impl fmt::Debug for $name {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				return write!(f, "{}", self);
			}
		}

	}

}

gen_vec!(Vec2(vec2) -> (x, y): f32, (0, 0));
gen_vec!(Vec3(vec3) -> (x, y, z): f32, (0, 0, 0));
gen_vec!(Vec4(vec4) -> (x, y, z, w): f32, (0, 0, 0, 0));
gen_vec!(Size(size) -> (x, y): u32, (0, 0));
gen_vec!(Color(color) -> (r, g, b, a): f32, (1, 1, 1, 1));
gen_vec!(Rect(rect) -> (x, y, w, h): f32, (0, 0, 0, 0));

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
	pub fn unit(&self) -> Self {
		return self.clone() / self.mag();
	}

	/// get vector normal
	pub fn normal(&self) -> Self {
		return vec2!(self.y, -self.x);
	}

	/// dot product of 2 vectors
	pub fn dot(&self, other: Self) -> f32 {
		return self.x * other.x + self.y * other.y;
	}

	/// get angle between 2 vectors
	pub fn angle(&self, other: Self) -> f32 {
		return (other.y - self.y).atan2(other.x - self.x);
	}

	/// get distance between another vector
	pub fn dis(&self, other: Self) -> f32 {
		return ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
	}

}

impl Vec3 {

	/// get vector magnitude
	pub fn mag(&self) -> f32 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
	}

	/// normalize vector
	pub fn unit(&self) -> Self {
		return self.clone() / self.mag();
	}

	/// dot product
	pub fn dot(&self, other: Self) -> f32 {
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}

	/// cross product
	pub fn cross(&self, other: Self) -> Self {
		return vec3!(
			(self.y * other.z) - (self.z * other.y),
			(self.z * other.x) - (self.x * other.z),
			(self.x * other.y) - (self.y * other.x)
        );
	}

}

impl Color {

	pub fn from_hex(hex: u32, opacity: f32) -> Self {

		let r = (hex >> 16) as f32 / 255.0;
		let g = ((hex >> 8) & 0xff) as f32 / 255.0;
		let b = (hex & 0xff) as f32 / 255.0;

		return color!(r, g, b, opacity);

	}

	pub fn to_rgba(&self) -> [u8; 4] {

		let r = (self.r * 255.0) as u8;
		let g = (self.g * 255.0) as u8;
		let b = (self.b * 255.0) as u8;
		let a = (self.a * 255.0) as u8;

		return [r, g, b, a];

	}

}

impl From<Size> for Vec2 {
	fn from(s: Size) -> Self {
		return vec2!(s.x, s.y);
	}
}

