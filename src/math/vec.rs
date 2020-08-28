// wengwengweng

use std::fmt;
use std::ops::*;

use serde::Serialize;
use serde::Deserialize;

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
						$crate::math::$name::default()
					};

					($v:expr) => {
						$crate::math::$name::all($v as $type)
					};

					($d($v:expr),*$d(,)?) => {
						$crate::math::$name::new($d($v as $type),*)
					};

				}

			}

		}

		pub use $sname;

		#[allow(missing_docs)]
		#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
		pub struct $name {
			$(
			pub $member: $type
			),+
		}

		impl $name {

			pub const fn new($($member: $type,)+) -> Self {
				return Self {
					$($member: $member,)+
				};
			}

			pub const fn all(v: $type) -> Self {
				return Self {
					$(
						$member: v,
					)+
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

		impl Add for $name {
			type Output = Self;
			fn add(self, other: Self) -> Self {
				return Self {
					$($member: self.$member + other.$member),+
				};
			}
		}

		impl AddAssign for $name {
			fn add_assign(&mut self, other: Self) {
				*self = *self + other;
			}
		}

		impl Sub for $name {
			type Output = Self;
			fn sub(self, other: Self) -> Self {
				return Self {
					$($member: self.$member - other.$member),+
				};
			}
		}

		impl SubAssign for $name {
			fn sub_assign(&mut self, other: Self) {
				*self = *self - other;
			}
		}

		impl Mul for $name {
			type Output = Self;
			fn mul(self, other: Self) -> Self {
				return Self {
					$($member: self.$member * other.$member),+
				};
			}
		}

		impl Mul<$type> for $name {
			type Output = $name;
			fn mul(self, s: $type) -> Self {
				return Self {
					$($member: self.$member * s),+
				};
			}
		}

		impl MulAssign for $name {
			fn mul_assign(&mut self, other: Self) {
				*self = *self * other;
			}
		}

		impl MulAssign<$type> for $name {
			fn mul_assign(&mut self, s: $type) {
				*self = *self * s;
			}
		}

		impl Div for $name {
			type Output = Self;
			fn div(self, other: Self) -> Self {
				return Self {
					$($member: self.$member / other.$member),+
				};
			}
		}

		impl Div<$type> for $name {
			type Output = Self;
			fn div(self, s: $type) -> Self {
				return Self {
					$($member: self.$member / s),+
				};
			}
		}

		impl DivAssign for $name {
			fn div_assign(&mut self, other: Self) {
				*self = *self / other;
			}
		}

		impl DivAssign<$type> for $name {
			fn div_assign(&mut self, s: $type) {
				*self = *self / s;
			}
		}

		impl Neg for $name {
			type Output = Self;
			fn neg(self) -> Self {
				return Self {
					$($member: -self.$member),+
				};
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

mix!(Vec4(x, y, z, w): f32, Color(r, g, b, a): f32);
mix!(Vec4(x, y, z, w): f32, Quad(x, y, w, h): f32);
mix!(Vec2(x, y): f32, Pt(x, y): i32);

impl Vec2 {

	pub fn from_angle(angle: f32) -> Self {
		return vec2!(f32::cos(angle), f32::sin(angle));
	}

	pub fn unit(self) -> Self {
		return self / self.len();
	}

	pub fn normal(self) -> Self {
		return vec2!(self.y, -self.x);
	}

	pub fn dot(p1: Self, p2: Self) -> f32 {
		return p1.x * p2.x + p1.y * p2.y;
	}

	pub fn cross(p1: Self, p2: Self) -> Vec3 {
		return Vec3::cross(vec3!(p1.x, p1.y, 0), vec3!(p2.x, p2.y, 0));
	}

	pub fn angle(self) -> f32 {
		return Vec2::angle_between(self, vec2!(0));
	}

	pub fn angle_between(p1: Self, p2: Self) -> f32 {
		return f32::atan2(p1.y - p2.y, p1.x - p2.x);
	}

	pub fn len(self) -> f32 {
		return Vec2::dist(self, vec2!(0));
	}

	pub fn dist(p1: Self, p2: Self) -> f32 {
		return f32::sqrt(
			(p1.x - p2.x) * (p1.x - p2.x) +
			(p1.y - p2.y) * (p1.y - p2.y)
		);
	}

	pub fn clamp(self, low: Self, hi: Self) -> Self {
		return vec2!(
			self.x.max(low.x).min(hi.x),
			self.y.max(low.y).min(hi.y),
		);
	}

}

impl Vec3 {

	pub fn len(self) -> f32 {
		return Vec3::dist(self, vec3!(0));
	}

	pub fn dist(p1: Self, p2: Self) -> f32 {
		return f32::sqrt(
			(p1.x - p2.x) * (p1.x - p2.x) +
			(p1.y - p2.y) * (p1.y - p2.y) +
			(p1.z - p2.z) * (p1.z - p2.z)
		);
	}

	pub fn unit(self) -> Self {
		return self / self.len();
	}

	pub fn dot(p1: Self, p2: Self) -> f32 {
		return p1.x * p2.x + p1.y * p2.y + p1.z * p2.z;
	}

	pub fn cross(p1: Self, p2: Self) -> Self {
		return vec3!(
			(p1.y * p2.z) - (p1.z * p2.y),
			(p1.z * p2.x) - (p1.x * p2.z),
			(p1.x * p2.y) - (p1.y * p2.x)
        );
	}

	pub fn xy(self) -> Vec2 {
		return vec2!(self.x, self.y);
	}

	pub fn xz(self) -> Vec2 {
		return vec2!(self.x, self.z);
	}

	pub fn clamp(self, low: Self, hi: Self) -> Self {
		return vec3!(
			self.x.max(low.x).min(hi.x),
			self.y.max(low.y).min(hi.y),
			self.z.max(low.z).min(hi.z),
		);
	}

}

impl Vec4 {

	pub fn xy(self) -> Vec2 {
		return vec2!(self.x, self.y);
	}

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

	pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
		return Self {
			r: r as f32 / 255.0,
			g: g as f32 / 255.0,
			b: b as f32 / 255.0,
			a: a as f32 / 255.0,
		};
	}

// 	pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
// 	}

	pub fn to_hsl(&self) -> (f32, f32, f32) {

		let cmax = self.r.max(self.g).max(self.b);
		let cmin = self.r.min(self.g).min(self.b);
		let delta = cmax - cmin;
		let l = (cmax + cmin) / 2.0;

		if delta == 0.0 {
			return (0.0, 0.0, l);
		}

		let h = if cmax == self.r {
			(self.g - self.b) / delta % 6.0
		} else if cmax == self.g {
			(self.b - self.r) / delta + 2.0
		} else if cmax == self.b {
			(self.r - self.g) / delta + 4.0
		} else {
			0.0
		} / 6.0;

		let h = if h < 0.0 {
			1.0 + h
		} else {
			h
		};

		let s = delta / (1.0 - (2.0 * l - 1.0).abs());

		return (h, s, l);

	}

	pub fn red(&self) -> f32 {
		return self.r;
	}

	pub fn set_red(&mut self, r: f32) {
		self.r = r.max(0.0).min(1.0);
	}

	pub fn green(&self) -> f32 {
		return self.g;
	}

	pub fn set_green(&mut self, g: f32) {
		self.g = g.max(0.0).min(1.0);
	}

	pub fn blue(&self) -> f32 {
		return self.b;
	}

	pub fn set_blue(&mut self, b: f32) {
		self.b = b.max(0.0).min(1.0);
	}

	pub fn alpha(&self) -> f32 {
		return self.a;
	}

	pub fn set_alpha(&mut self, a: f32) {
		self.a = a.max(0.0).min(1.0);
	}

	pub fn hue(&self) -> f32 {
		return self.to_hsl().0;
	}

	pub fn saturation(&self) -> f32 {
		return self.to_hsl().1;
	}

	pub fn lightness(&self) -> f32 {
		return self.to_hsl().2;
	}

	pub fn brighten(self, v: f32) -> Self {
		return (self + rgba!(v, v, v, 0)).clamp(rgba!(0), rgba!(1));
	}

	pub fn darken(self, v: f32) -> Self {
		return (self - rgba!(v, v, v, 0)).clamp(rgba!(0), rgba!(1));
	}

	pub fn invert(self) -> Self {
		return Self {
			r: 1.0 - self.r,
			g: 1.0 - self.g,
			b: 1.0 - self.b,
			a: self.a,
		};
	}

	pub fn as_u8(&self) -> (u8, u8, u8, u8) {
		return (
			(self.r * 255.0) as u8,
			(self.g * 255.0) as u8,
			(self.b * 255.0) as u8,
			(self.a * 255.0) as u8,
		);
	}

	pub fn brightness(&self) -> f32 {
		return (self.r + self.g + self.b) / 3.0;
	}

	pub fn clamp(self, c1: Self, c2: Self) -> Self {
		return rgba!(
			self.r.max(c1.r).min(c2.r),
			self.g.max(c1.g).min(c2.g),
			self.b.max(c1.b).min(c2.b),
			self.a.max(c1.a).min(c2.a),
		);
	}

	pub fn rgb(&self) -> Vec3 {
		return vec3!(self.r, self.g, self.b);
	}

	pub fn as_vec4(&self) -> Vec4 {
		return vec4!(self.r, self.g, self.b, self.a);
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

