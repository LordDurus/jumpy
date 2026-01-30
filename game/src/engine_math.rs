// engine_math.rs
//
// engine-owned math types. do not depend on platform math libs here.
// convert at the edges (render/input) if needed.

use crate::Level;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(feature = "pc")]
use nalgebra::Vector2 as NalgebraVector2;

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
pub fn do_they_overlap(a_left: f32, a_top: f32, a_width: f32, a_height: f32, b_left: f32, b_top: f32, b_width: f32, b_height: f32) -> bool {
	a_left < b_left + b_width && a_left + a_width > b_left && a_top < b_top + b_height && a_top + a_height > b_top
}

pub fn aabb_overlaps_solid_tiles(level: &Level, left: f32, right: f32, top: f32, bottom: f32) -> bool {
	let a_width: f32 = right - left;
	let a_height: f32 = bottom - top;

	let tile_width_world: f32 = level.tile_width as f32;
	let tile_height_world: f32 = level.tile_height as f32;

	let start_tile_x: i32 = (left / tile_width_world).floor() as i32;
	let end_tile_x: i32 = ((right - 0.001) / tile_width_world).floor() as i32;
	let start_tile_y: i32 = (top / tile_height_world).floor() as i32;
	let end_tile_y: i32 = ((bottom - 0.001) / tile_height_world).floor() as i32;

	for ty in start_tile_y..=end_tile_y {
		for tx in start_tile_x..=end_tile_x {
			if !level.is_solid_at_tile(tx, ty) {
				continue;
			}

			let tile_left: f32 = tx as f32 * tile_width_world;
			let tile_top: f32 = ty as f32 * tile_height_world;

			if do_they_overlap(left, top, a_width, a_height, tile_left, tile_top, tile_width_world, tile_height_world) {
				return true;
			}
		}
	}

	return false;
}

#[allow(dead_code)]
#[inline(always)]
pub fn random_u32(state: &mut u32) -> u32 {
	let mut x: u32 = *state;
	x ^= x << 13;
	x ^= x >> 17;
	x ^= x << 5;
	*state = x;
	return x;
}

#[inline(always)]
pub fn random_u16(state: &mut u16) -> u16 {
	let mut x: u16 = *state;
	x ^= x << 7;
	x ^= x >> 9;
	x ^= x << 8;
	*state = x;
	return x;
}
