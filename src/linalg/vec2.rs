use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::Neg;

use crate::imdraw::ImDraw;

#[derive(PartialEq, Debug, Copy, Clone, ImDraw)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
pub static RIGHT    : Vec2 = Vec2 { x:  1.0, y:  0.0 };
#[allow(dead_code)]
pub static LEFT     : Vec2 = Vec2 { x: -1.0, y:  0.0 };
#[allow(dead_code)]
pub static UP       : Vec2 = Vec2 { x:  0.0, y:  1.0 };
#[allow(dead_code)]
pub static DOWN     : Vec2 = Vec2 { x:  0.0, y: -1.0 };

impl Vec2 {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn mag(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        (x*x + y*y).sqrt()
    }

    pub fn mag2(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        x*x + y*y
    }

    pub fn norm(&self) -> Self {
        let mag = self.mag();
        assert!(mag > 0.0);
        *self / mag
    }

    pub fn normalize(&mut self) {
        *self = self.norm();
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec2 {
            x: rhs * self.x,
            y: rhs * self.y,
        };
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        // Does Rust do it correctly or do we need a custom float comparator?
        assert!(rhs != 0.0);

        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        // Does Rust do it correctly or do we need a custom float comparator?
        assert!(rhs != 0.0);

        *self = Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec2::new(), Vec2 { x: 0.0, y: 0.0 });
    }

    #[test]
    fn test_add() {
        let v0 = Vec2 { x: 1.0, y: 2.0 };
        let v1 = Vec2 { x: 4.0, y: 5.0 };

        assert_eq!(v0 + v1, Vec2 { x: 5.0, y: 7.0 });
    }

    #[test]
    fn test_add_assign() {
        let v0 = Vec2 { x: 1.0, y: 2.0 };
        let mut v1 = Vec2 { x: 4.0, y: 5.0 };
        v1 += v0;

        assert_eq!(v1, Vec2 { x: 5.0, y: 7.0 });
    }

    #[test]
    fn test_sub() {
        let v0 = Vec2 { x: 1.0, y: 2.0 };
        let v1 = Vec2 { x: 4.0, y: 6.0 };

        assert_eq!(v1 - v0, Vec2 { x: 3.0, y: 4.0 });
    }

    #[test]
    fn test_sub_assign() {
        let v0 = Vec2 { x: 1.0, y: 2.0 };
        let mut v1 = Vec2 { x: 4.0, y: 6.0 };
        v1 -= v0;

        assert_eq!(v1, Vec2 { x: 3.0, y: 4.0 });
    }

    #[test]
    fn test_mul() {
        let v = Vec2 { x: 1.0, y: 2.0 };

        assert_eq!(v * 2.0, Vec2 { x: 2.0, y: 4.0 });
        assert_eq!(v * 2 as f32, Vec2 { x: 2.0, y: 4.0 });
        assert_eq!(2.0 * v, Vec2 { x: 2.0, y: 4.0 });
        assert_eq!(2 as f32 * v, Vec2 { x: 2.0, y: 4.0 });
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec2 { x: 1.0, y: 2.0 };
        v *= 2.0;

        assert_eq!(v, Vec2 { x: 2.0, y: 4.0 });
    }

    #[test]
    fn test_div() {
        let v = Vec2 { x: 1.0, y: 2.0 };

        assert_eq!(v / 2.0, Vec2 { x: 0.5, y: 1.0 });
        assert_eq!(v / 2 as f32, Vec2 { x: 0.5, y: 1.0 });
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec2 { x: 1.0, y: 2.0 };
        v /= 2.0;

        assert_eq!(v, Vec2 { x: 0.5, y: 1.0 });
    }

    // TODO mag, mag2, norm
}
