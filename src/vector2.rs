#[cfg(feature = "pc")]
use nalgebra::Vector2 as NalgebraVector2;

#[allow(dead_code)]
pub trait Vector2 {
    fn new(x: f32, y: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn add(&mut self, other: &Self);
    fn scale(&mut self, scalar: f32);
}

// Custom Vector2 implementation
#[derive(Debug, Clone, Copy, Default)]
pub struct MyVector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 for MyVector2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }

    fn scale(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

// Optional: Implementation for nalgebra's Vector2
#[cfg(feature = "pc")]
impl Vector2 for NalgebraVector2<f32> {
    fn new(x: f32, y: f32) -> Self {
        Self::new(x, y)
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn add(&mut self, other: &Self) {
        *self += *other;
    }

    fn scale(&mut self, scalar: f32) {
        *self *= scalar;
    }
}

// Gravity constant (common for both implementations)
pub const GRAVITY: MyVector2 = MyVector2 { x: 0.0, y: -9.8 };
