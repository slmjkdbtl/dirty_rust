// wengwengweng

use std::fmt;
use std::ops;

use derive_more::*;

use super::*;

// https://github.com/rust-lang/rust/issues/35853
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
						$crate::math::$name::default();
					};

					($v:expr) => {
						$crate::math::$name { $($member: $v as $type),+ };
					};

					($d($v:expr),*$d(,)?) => {
						$crate::math::$name::new($d($v as $type),*)
					};

				}

			}

		}

		pub use $sname;

		#[allow(missing_docs)]
		#[derive(Copy, Clone, PartialEq, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, From, Into)]
		pub struct $name {
			$(
			pub $member: $type
			),+
		}

		impl $name {

			/// initialize with specifying all fields
			pub const fn new($($member: $type,)+) -> Self {
				return Self {
					$($member: $member),+
				};
			}

			pub const fn as_arr(&self) -> [$type; $count] {
				return [
					$(
						self.$member,
					)+
				];
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

		impl ops::Mul<$name> for $type {

			type Output = $name;

			fn mul(self, v: $name) -> $name {
				return $name {
					$($member: v.$member * self),+
				};
			}

		}

		impl ops::Div<$name> for $name {

			type Output = Self;

			fn div(self, other: Self) -> Self {
				return Self {
					$($member: self.$member / other.$member),+
				};
			}

		}

		impl Into<[$type; $count]> for $name {
			fn into(self) -> [$type; $count] {
				return self.as_arr();
			}
		}

		impl Default for $name {
			fn default() -> Self {
				return $sname!($($default),+);
			}
		}

		impl fmt::Debug for $name {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				return <$name as fmt::Display>::fmt(self, f);
			}
		}

		impl fmt::Display for $name {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				return write!(f, "{}({})", stringify!($sname), [$(format!("{}", self.$member)),+].join(", "));
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
gen_vec!(Color(rgba) -> (r, g, b, a): [f32; 4], (1, 1, 1, 1));
gen_vec!(Quad(quad) -> (x, y, w, h): [f32; 4], (0, 0, 1, 1));
gen_vec!(Pt(pt) -> (x, y): [i32; 2], (0, 0));
gen_vec!(Size(size) -> (w, h): [i32; 2], (0, 0));

mix!(Vec4(x, y, z, w): f32, Color(r, g, b, a): f32);
mix!(Vec4(x, y, z, w): f32, Quad(x, y, w, h): f32);
mix!(Vec2(x, y): f32, Pt(x, y): i32);
mix!(Vec2(x, y): f32, Size(w, h): i32);
mix!(Pt(x, y): i32, Size(w, h): i32);

impl Vec2 {

	/// get a vector from given angle
	pub fn from_angle(angle: f32) -> Self {
		return vec2!(f32::cos(angle), f32::sin(angle));
	}

	/// get vector magnitude
	pub fn mag(&self) -> f32 {
		return f32::sqrt(self.x * self.x + self.y * self.y);
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

	/// get angle between 2 points
	pub fn angle(self, other: Self) -> f32 {
		return f32::atan2(other.y - self.y, other.x - self.x);
	}

	/// get distance between another vector
	pub fn dis(self, other: Self) -> f32 {
		return f32::sqrt((self.x - other.x).powi(2) + (self.y - other.y).powi(2));
	}

	/// clamp between 2 values
	pub fn clamp(self, low: Self, hi: Self) -> Self {
		return vec2!(
			self.x.clamp(low.x, hi.x),
			self.y.clamp(low.y, hi.y),
		);
	}

}

impl Vec3 {

	/// get vector magnitude
	pub fn mag(&self) -> f32 {
		return f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
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

	/// get xy component as a Vec2
	pub fn xy(self) -> Vec2 {
		return vec2!(self.x, self.y);
	}

	/// clamp between 2 values
	pub fn clamp(self, low: Self, hi: Self) -> Self {
		return vec3!(
			self.x.clamp(low.x, hi.x),
			self.y.clamp(low.y, hi.y),
			self.z.clamp(low.z, hi.z),
		);
	}

}

impl Vec4 {

	/// get xy component as a Vec2
	pub fn xy(self) -> Vec2 {
		return vec2!(self.x, self.y);
	}

	/// get xyz component as a Vec3
	pub fn xyz(self) -> Vec3 {
		return vec3!(self.x, self.y, self.z);
	}

}

impl Color {

	pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
	pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
	pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
	pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
	pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
	pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);
	pub const PURPLE: Self = Self::new(1.0, 0.0, 1.0, 1.0);
	pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);
	pub const NONE: Self = Self::new(0.0, 0.0, 0.0, 0.0);

	pub fn from_hex(hex: u32, opacity: f32) -> Self {

		let r = (hex >> 16) as f32 / 255.0;
		let g = ((hex >> 8) & 0xff) as f32 / 255.0;
		let b = (hex & 0xff) as f32 / 255.0;

		return rgba!(r, g, b, opacity);

	}

	pub fn as_u8(&self) -> [u8; 4] {

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

	pub fn clamp(self, c1: Self, c2: Self) -> Self {
		return rgba!(
			self.r.clamp(c1.r, c2.r),
			self.g.clamp(c1.g, c2.g),
			self.b.clamp(c1.b, c2.b),
			self.a.clamp(c1.a, c2.a),
		);
	}

	pub fn brighten(self, v: f32) -> Self {
		return (self + rgba!(v, v, v, 0)).clamp(rgba!(0), rgba!(1));
	}

	pub fn darken(self, v: f32) -> Self {
		return (self - rgba!(v, v, v, 0)).clamp(rgba!(0), rgba!(1));
	}

	pub fn a(mut self, a: f32) -> Self {
		self.a = a;
		return self;
	}

	pub fn rgb(&self) -> Vec3 {
		return vec3!(self.r, self.g, self.b);
	}

	pub fn to_srgb(&self) -> Self {
		return Self {
			r: linear_to_srgb(self.r),
			g: linear_to_srgb(self.g),
			b: linear_to_srgb(self.b),
			a: self.a,
		}
	}

	pub fn to_linear(&self) -> Self {
		return Self {
			r: srgb_to_linear(self.r),
			g: srgb_to_linear(self.g),
			b: srgb_to_linear(self.b),
			a: self.a,
		}
	}

}

// http://entropymine.com/imageworsener/srgbformula/
fn linear_to_srgb(v: f32) -> f32 {
	if v <= 0.0031308 && v >= 0.0 {
		return v * 12.92;
	} else {
		return 1.055 * f32::powf(v, 1.0 / 2.4) - 0.055;
	}
}

fn srgb_to_linear(v: f32) -> f32 {
	if v <= 0.04045 && v >= 0.0 {
		return v / 12.92;
	} else {
		return f32::powf((v + 0.055) / 1.055, 2.4);
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

