use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::Neg;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
static RIGHT    : Vec3 = Vec3 { x:  1.0, y:  0.0, z:  0.0 };
#[allow(dead_code)]
static LEFT     : Vec3 = Vec3 { x: -1.0, y:  0.0, z:  0.0 };
#[allow(dead_code)]
static UP       : Vec3 = Vec3 { x:  0.0, y:  1.0, z:  0.0 };
#[allow(dead_code)]
static DOWN     : Vec3 = Vec3 { x:  0.0, y: -1.0, z:  0.0 };
#[allow(dead_code)]
static BACKWARD : Vec3 = Vec3 { x:  0.0, y:  0.0, z:  1.0 };
#[allow(dead_code)]
static FORWARD  : Vec3 = Vec3 { x:  0.0, y:  0.0, z: -1.0 };

impl Vec3 {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn mag(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        (x*x + y*y + z*z).sqrt()
    }

    pub fn mag2(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        x*x + y*y + z*z
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

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        };
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        // Does Rust do it correctly or do we need a custom float comparator?
        assert!(rhs != 0.0);

        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        // Does Rust do it correctly or do we need a custom float comparator?
        assert!(rhs != 0.0);

        *self = Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(), Vec3 { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn test_add() {
        let v0 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v1 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };

        assert_eq!(v0 + v1, Vec3 { x: 5.0, y: 7.0, z: 9.0 });
    }

    #[test]
    fn test_add_assign() {
        let v0 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let mut v1 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        v1 += v0;

        assert_eq!(v1, Vec3 { x: 5.0, y: 7.0, z: 9.0 });
    }

    #[test]
    fn test_sub() {
        let v0 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v1 = Vec3 { x: 4.0, y: 6.0, z: 8.0 };

        assert_eq!(v1 - v0, Vec3 { x: 3.0, y: 4.0, z: 5.0 });
    }

    #[test]
    fn test_sub_assign() {
        let v0 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let mut v1 = Vec3 { x: 4.0, y: 6.0, z: 8.0 };
        v1 -= v0;

        assert_eq!(v1, Vec3 { x: 3.0, y: 4.0, z: 5.0 });
    }

    #[test]
    fn test_mul() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

        assert_eq!(v * 2.0, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
        assert_eq!(v * 2 as f32, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
        assert_eq!(2.0 * v, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
        assert_eq!(2 as f32 * v, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        v *= 2.0;

        assert_eq!(v, Vec3 { x: 2.0, y: 4.0, z: 6.0 });
    }

    #[test]
    fn test_div() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

        assert_eq!(v / 2.0, Vec3 { x: 0.5, y: 1.0, z: 1.5 });
        assert_eq!(v / 2 as f32, Vec3 { x: 0.5, y: 1.0, z: 1.5 });
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        v /= 2.0;

        assert_eq!(v, Vec3 { x: 0.5, y: 1.0, z: 1.5 });
    }
}
