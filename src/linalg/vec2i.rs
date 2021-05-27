use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::Neg;

use crate::app::ImDraw;

#[derive(PartialEq, Debug, Copy, Clone, ImDraw, Default)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
pub static RIGHT    : Vec2i = Vec2i { x:  1, y:  0 };
#[allow(dead_code)]
pub static LEFT     : Vec2i = Vec2i { x: -1, y:  0 };
#[allow(dead_code)]
pub static UP       : Vec2i = Vec2i { x:  0, y:  1 };
#[allow(dead_code)]
pub static DOWN     : Vec2i = Vec2i { x:  0, y: -1 };

impl Vec2i {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add for Vec2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2i {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub for Vec2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2i {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Mul<i32> for Vec2i {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2i {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl Mul<Vec2i> for i32 {
    type Output = Vec2i;

    fn mul(self, rhs: Vec2i) -> Self::Output {
        Vec2i {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign<i32> for Vec2i {
    fn mul_assign(&mut self, rhs: i32) {
        *self = Vec2i {
            x: rhs * self.x,
            y: rhs * self.y,
        };
    }
}

impl Div<i32> for Vec2i {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        // Does Rust do it correctly or do we need a custom float comparator?
        assert!(rhs != 0);

        Vec2i {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<i32> for Vec2i {
    fn div_assign(&mut self, rhs: i32) {
        // Does Rust do it correctly or do we need a custom float comparator?
        assert!(rhs != 0);

        *self = Vec2i {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl Neg for Vec2i {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2i {
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
        assert_eq!(Vec2i::new(), Vec2i { x: 0, y: 0 });
    }

    #[test]
    fn test_add() {
        let v0 = Vec2i { x: 1, y: 2 };
        let v1 = Vec2i { x: 4, y: 5 };

        assert_eq!(v0 + v1, Vec2i { x: 5, y: 7 });
    }

    #[test]
    fn test_add_assign() {
        let v0 = Vec2i { x: 1, y: 2 };
        let mut v1 = Vec2i { x: 4, y: 5 };
        v1 += v0;

        assert_eq!(v1, Vec2i { x: 5, y: 7 });
    }

    #[test]
    fn test_sub() {
        let v0 = Vec2i { x: 1, y: 2 };
        let v1 = Vec2i { x: 4, y: 6 };

        assert_eq!(v1 - v0, Vec2i { x: 3, y: 4 });
    }

    #[test]
    fn test_sub_assign() {
        let v0 = Vec2i { x: 1, y: 2 };
        let mut v1 = Vec2i { x: 4, y: 6 };
        v1 -= v0;

        assert_eq!(v1, Vec2i { x: 3, y: 4 });
    }

    #[test]
    fn test_mul() {
        let v = Vec2i { x: 1, y: 2 };

        assert_eq!(v * 2, Vec2i { x: 2, y: 4 });
        assert_eq!(2 * v, Vec2i { x: 2, y: 4 });
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec2i { x: 1, y: 2 };
        v *= 2;

        assert_eq!(v, Vec2i { x: 2, y: 4 });
    }

    #[test]
    fn test_div() {
        let v = Vec2i { x: 1, y: 2 };

        assert_eq!(v / 2, Vec2i { x: 0, y: 1 });
        assert_eq!(v / 2 as i32, Vec2i { x: 0, y: 1 });
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec2i { x: 1, y: 2 };
        v /= 2;

        assert_eq!(v, Vec2i { x: 0, y: 1 });
    }
}

