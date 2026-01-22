// engine_math.rs
//
// engine-owned math types. do not depend on platform math libs here.
// convert at the edges (render/input) if needed.

#[cfg(feature = "pc")]
use nalgebra::Vector2 as NalgebraVector2;

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Add for Vec2 {
	type Output = Vec2;

	fn add(self, rhs: Vec2) -> Vec2 {
		return Vec2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		};
	}
}

impl Sub for Vec2 {
	type Output = Vec2;

	fn sub(self, rhs: Vec2) -> Vec2 {
		return Vec2 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		};
	}
}

impl AddAssign for Vec2 {
	fn add_assign(&mut self, rhs: Vec2) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl PartialEq for Vec2 {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl SubAssign for Vec2 {
	fn sub_assign(&mut self, rhs: Vec2) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2 {
		return Vec2 { x, y };
	}

	pub fn zero() -> Vec2 {
		return Vec2 { x: 0.0, y: 0.0 };
	}

	pub fn x(&self) -> f32 {
		return self.x;
	}

	pub fn y(&self) -> f32 {
		return self.y;
	}

	pub fn set_x(&mut self, x: f32) {
		self.x = x;
	}

	pub fn set_y(&mut self, y: f32) {
		self.y = y;
	}

	pub fn add(&mut self, other: &Vec2) {
		self.x += other.x;
		self.y += other.y;
	}

	pub fn sub(&mut self, other: &Vec2) {
		self.x -= other.x;
		self.y -= other.y;
	}

	pub fn scale(&mut self, scalar: f32) {
		self.x *= scalar;
		self.y *= scalar;
	}

	pub fn scaled(&self, scalar: f32) -> Vec2 {
		return Vec2 {
			x: self.x * scalar,
			y: self.y * scalar,
		};
	}

	pub fn dot(&self, other: &Vec2) -> f32 {
		return self.x * other.x + self.y * other.y;
	}

	pub fn length_squared(&self) -> f32 {
		return self.dot(self);
	}

	pub fn length(&self) -> f32 {
		return self.length_squared().sqrt();
	}

	pub fn normalized(&self) -> Vec2 {
		let len = self.length();
		if len == 0.0 {
			return Vec2::zero();
		}
		return Vec2 {
			x: self.x / len,
			y: self.y / len,
		};
	}
}

impl Mul<f32> for Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: f32) -> Vec2 {
		return Vec2 {
			x: self.x * rhs,
			y: self.y * rhs,
		};
	}
}

impl MulAssign<f32> for Vec2 {
	fn mul_assign(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl Neg for Vec2 {
	type Output = Vec2;

	fn neg(self) -> Vec2 {
		return Vec2 { x: -self.x, y: -self.y };
	}
}

// pc interop only at the edges
#[cfg(feature = "pc")]
impl From<Vec2> for NalgebraVector2<f32> {
	fn from(v: Vec2) -> NalgebraVector2<f32> {
		return NalgebraVector2::new(v.x, v.y);
	}
}

#[cfg(feature = "pc")]
impl From<NalgebraVector2<f32>> for Vec2 {
	fn from(v: NalgebraVector2<f32>) -> Vec2 {
		return Vec2 { x: v.x, y: v.y };
	}
}

#[inline(always)]
pub fn rects_overlap(a_left: f32, a_top: f32, a_width: f32, a_height: f32, b_left: f32, b_top: f32, b_width: f32, b_height: f32) -> bool {
	a_left < b_left + b_width && a_left + a_width > b_left && a_top < b_top + b_height && a_top + a_height > b_top
}
