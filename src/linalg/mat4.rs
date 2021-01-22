use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use std::ops::Neg;

use crate::imgui::*;
use crate::app::imgui::*;

use super::vec3::Vec3;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

#[allow(dead_code)]
pub static IDENTITY : Mat4 = Mat4 {
    m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl Mat4 {
    pub fn new() -> Self {
        Self {
            m: [[0.0; 4]; 4],
        }
    }

    pub fn transpose(&mut self) {
        let tmp = self.m[0][1]; self.m[0][1] = self.m[1][0]; self.m[1][0] = tmp;
        let tmp = self.m[0][2]; self.m[0][2] = self.m[2][0]; self.m[2][0] = tmp;
        let tmp = self.m[0][3]; self.m[0][3] = self.m[3][0]; self.m[3][0] = tmp;

        let tmp = self.m[1][2]; self.m[1][2] = self.m[2][1]; self.m[2][1] = tmp;
        let tmp = self.m[1][3]; self.m[1][3] = self.m[3][1]; self.m[3][1] = tmp;

        let tmp = self.m[2][3]; self.m[2][3] = self.m[3][2]; self.m[3][2] = tmp;
    }

    pub fn transposed(&self) -> Mat4 {
        Mat4 {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
                [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
            ],
        }
    }
}

#[allow(dead_code)]
pub fn translation(v: Vec3) -> Mat4 {
    Mat4 {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [v.x, v.y, v.z, 1.0],
        ],
    }
}

#[allow(dead_code)]
pub fn rotation(rad: f32, axis: Vec3) -> Mat4 {
    let (s, c) = f32::sin_cos(rad);

    let a = axis.norm();
    let t = a * (1.0 - c);

    let mut m = [[0.0f32; 4]; 4];

    m[0][0] = c + t.x * a.x;
    m[0][1] = 0.0 + t.x * a.y + s * a.z;
    m[0][2] = 0.0 + t.x * a.z - s * a.y;
    m[0][3] = 0.0;

    m[1][0] = 0.0 + t.y * a.x - s * a.z;
    m[1][1] = c + t.y * a.y;
    m[1][2] = 0.0 + t.y * a.z + s * a.x;
    m[1][3] = 0.0;

    m[2][0] = 0.0 + t.z * a.x + s * a.y;
    m[2][1] = 0.0 + t.z * a.y - s * a.x;
    m[2][2] = c + t.z * a.z;
    m[2][3] = 0.0;

    m[3][3] = 1.0;

    Mat4 { m }
}

// TODO invert_z: bool
#[allow(dead_code)]
pub fn ortho(
    left: f32, right: f32,
    bottom: f32, top: f32,
    near: f32, far: f32
) -> Mat4 {

    Mat4 {
        m: [
            [ 2.0 / (right - left), 0.0, 0.0, 0.0 ],
            [ 0.0, 2.0 / (top - bottom), 0.0, 0.0 ],
            [ 0.0, 0.0, -2.0 / (far - near), 0.0 ],
            [
                (left + right) / (left - right),
                (bottom + top) / (bottom - top),
                (far + near) / (far - near),
                1.0
            ],
        ]
    }
}

impl ImDraw for Mat4 {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {

        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            let id = ui.push_id(label);

            Drag::new(im_str2!("[0]")).build_array(ui, &mut self.m[0]);
            Drag::new(im_str2!("[1]")).build_array(ui, &mut self.m[1]);
            Drag::new(im_str2!("[2]")).build_array(ui, &mut self.m[2]);
            Drag::new(im_str2!("[3]")).build_array(ui, &mut self.m[3]);

            id.pop(ui);
        });
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut m = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = self.m[i][j] + rhs.m[i][j];
            }
        }

        Self { m }
    }
}

impl AddAssign for Mat4 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] += rhs.m[i][j];
            }
        }
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut m = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = self.m[i][j] - rhs.m[i][j];
            }
        }

        Self { m }
    }
}

impl SubAssign for Mat4 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] -= rhs.m[i][j];
            }
        }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut m = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                m[i][j] =
                    self.m[i][0] * rhs.m[0][j] +
                    self.m[i][1] * rhs.m[1][j] +
                    self.m[i][2] * rhs.m[2][j] +
                    self.m[i][3] * rhs.m[3][j]
                ;
            }
        }

        Self { m }
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut m = self.clone();

        for i in 0..4 {
            for j in 0..4 {
                m.m[i][j] *= rhs;
            }
        }

        m
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Mat4 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] *= rhs;
            }
        }
    }
}

impl Div<f32> for Mat4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        assert!(rhs != 0.0);

        let mut m = self.clone();

        for i in 0..4 {
            for j in 0..4 {
                m.m[i][j] /= rhs;
            }
        }

        m
    }
}

impl DivAssign<f32> for Mat4 {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] /= rhs;
            }
        }
    }
}

impl Neg for Mat4 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut m = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = -self.m[i][j];
            }
        }

        Self { m }
    }
}

/*
// Should I do this? This will just create more code that does the same thing as casting to f32
// before calling the multiplication

extern crate num_traits;
use num_traits::{Num, ToPrimitive};
impl<T: Num + ToPrimitive> Mul<T> for Mat4 {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut m = self.clone();

        for i in 0..4 {
            for j in 0..4 {
                m.m[i][j] *= rhs.to_f32().unwrap();
            }
        }

        m
    }
}
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Mat4::new().m, [[0.0; 4]; 4]);
    }

    #[test]
    fn test_add() {
        let m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let m1 = Mat4 {
            m: [
                [13.0, 14.0, 15.0, 16.0],
                [9.0, 10.0, 11.0, 12.0],
                [5.0, 6.0, 7.0, 8.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        };

        let res = Mat4 {
            m: [
                [1.0 + 13.0, 2.0 + 14.0, 3.0 + 15.0, 4.0 + 16.0],
                [5.0 + 9.0, 6.0 + 10.0, 7.0 + 11.0, 8.0 + 12.0],
                [9.0 + 5.0, 10.0 + 6.0, 11.0 + 7.0, 12.0 + 8.0],
                [13.0 + 1.0, 14.0 + 2.0, 15.0 + 3.0, 16.0 + 4.0],
            ]
        };

        assert_eq!(m0 + m1, res);

        let res = Mat4 {
            m: [
                [1.0 + 1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0 + 1.0, 7.0, 8.0],
                [9.0, 10.0, 11.0 + 1.0, 12.0],
                [13.0, 14.0, 15.0, 16.0 + 1.0],
            ]
        };

        assert_eq!(m0 + IDENTITY, res);
    }

    #[test]
    fn test_add_assign() {
        let mut m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let m1 = Mat4 {
            m: [
                [13.0, 14.0, 15.0, 16.0],
                [9.0, 10.0, 11.0, 12.0],
                [5.0, 6.0, 7.0, 8.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        };

        m0 += m1;

        let res = Mat4 {
            m: [
                [1.0 + 13.0, 2.0 + 14.0, 3.0 + 15.0, 4.0 + 16.0],
                [5.0 + 9.0, 6.0 + 10.0, 7.0 + 11.0, 8.0 + 12.0],
                [9.0 + 5.0, 10.0 + 6.0, 11.0 + 7.0, 12.0 + 8.0],
                [13.0 + 1.0, 14.0 + 2.0, 15.0 + 3.0, 16.0 + 4.0],
            ]
        };

        assert_eq!(m0, res);

        let mut m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        m0 += IDENTITY;

        let res = Mat4 {
            m: [
                [1.0 + 1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0 + 1.0, 7.0, 8.0],
                [9.0, 10.0, 11.0 + 1.0, 12.0],
                [13.0, 14.0, 15.0, 16.0 + 1.0],
            ]
        };

        assert_eq!(m0, res);
    }

    #[test]
    fn test_sub() {
        let m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let m1 = Mat4 {
            m: [
                [13.0, 14.0, 15.0, 16.0],
                [9.0, 10.0, 11.0, 12.0],
                [5.0, 6.0, 7.0, 8.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        };

        let res = Mat4 {
            m: [
                [1.0 - 13.0, 2.0 - 14.0, 3.0 - 15.0, 4.0 - 16.0],
                [5.0 - 9.0, 6.0 - 10.0, 7.0 - 11.0, 8.0 - 12.0],
                [9.0 - 5.0, 10.0 - 6.0, 11.0 - 7.0, 12.0 - 8.0],
                [13.0 - 1.0, 14.0 - 2.0, 15.0 - 3.0, 16.0 - 4.0],
            ]
        };

        assert_eq!(m0 - m1, res);

        let res = Mat4 {
            m: [
                [1.0 - 1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0 - 1.0, 7.0, 8.0],
                [9.0, 10.0, 11.0 - 1.0, 12.0],
                [13.0, 14.0, 15.0, 16.0 - 1.0],
            ]
        };

        assert_eq!(m0 - IDENTITY, res);
    }

    #[test]
    fn test_sub_assign() {
        let mut m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let m1 = Mat4 {
            m: [
                [13.0, 14.0, 15.0, 16.0],
                [9.0, 10.0, 11.0, 12.0],
                [5.0, 6.0, 7.0, 8.0],
                [1.0, 2.0, 3.0, 4.0],
            ]
        };

        m0 -= m1;

        let res = Mat4 {
            m: [
                [1.0 - 13.0, 2.0 - 14.0, 3.0 - 15.0, 4.0 - 16.0],
                [5.0 - 9.0, 6.0 - 10.0, 7.0 - 11.0, 8.0 - 12.0],
                [9.0 - 5.0, 10.0 - 6.0, 11.0 - 7.0, 12.0 - 8.0],
                [13.0 - 1.0, 14.0 - 2.0, 15.0 - 3.0, 16.0 - 4.0],
            ]
        };

        assert_eq!(m0, res);

        let mut m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        m0 -= IDENTITY;

        let res = Mat4 {
            m: [
                [1.0 - 1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0 - 1.0, 7.0, 8.0],
                [9.0, 10.0, 11.0 - 1.0, 12.0],
                [13.0, 14.0, 15.0, 16.0 - 1.0],
            ]
        };

        assert_eq!(m0, res);
    }

    #[test]
    fn test_mul_scalar() {
        let m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let res = Mat4 {
            m: [
                [2.0, 4.0, 6.0, 8.0],
                [10.0, 12.0, 14.0, 16.0],
                [18.0, 20.0, 22.0, 24.0],
                [26.0, 28.0, 30.0, 32.0],
            ]
        };

        assert_eq!(m0 * 2.0, res);
        assert_eq!(m0 * 2 as f32, res);
        assert_eq!(2.0 * m0, res);
        assert_eq!(2 as f32 * m0, res);
    }

    #[test]
    fn test_mul_scalar_assign() {
        let mut m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        m0 *= 2.0;

        let res = Mat4 {
            m: [
                [2.0, 4.0, 6.0, 8.0],
                [10.0, 12.0, 14.0, 16.0],
                [18.0, 20.0, 22.0, 24.0],
                [26.0, 28.0, 30.0, 32.0],
            ]
        };

        assert_eq!(m0, res);
    }

    #[test]
    fn test_mul_mat4() {
        let m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let res = Mat4 {
            m: [
                [
                    1.0 * 1.0 + 2.0 * 5.0 + 3.0 * 9.0 + 4.0 * 13.0,
                    1.0 * 2.0 + 2.0 * 6.0 + 3.0 * 10.0 + 4.0 * 14.0,
                    1.0 * 3.0 + 2.0 * 7.0 + 3.0 * 11.0 + 4.0 * 15.0,
                    1.0 * 4.0 + 2.0 * 8.0 + 3.0 * 12.0 + 4.0 * 16.0,
                ],
                [
                    5.0 * 1.0 + 6.0 * 5.0 + 7.0 * 9.0 + 8.0 * 13.0,
                    5.0 * 2.0 + 6.0 * 6.0 + 7.0 * 10.0 + 8.0 * 14.0,
                    5.0 * 3.0 + 6.0 * 7.0 + 7.0 * 11.0 + 8.0 * 15.0,
                    5.0 * 4.0 + 6.0 * 8.0 + 7.0 * 12.0 + 8.0 * 16.0,
                ],
                [
                    9.0 * 1.0 + 10.0 * 5.0 + 11.0 * 9.0 + 12.0 * 13.0,
                    9.0 * 2.0 + 10.0 * 6.0 + 11.0 * 10.0 + 12.0 * 14.0,
                    9.0 * 3.0 + 10.0 * 7.0 + 11.0 * 11.0 + 12.0 * 15.0,
                    9.0 * 4.0 + 10.0 * 8.0 + 11.0 * 12.0 + 12.0 * 16.0,
                ],
                [
                    13.0 * 1.0 + 14.0 * 5.0 + 15.0 * 9.0 + 16.0 * 13.0,
                    13.0 * 2.0 + 14.0 * 6.0 + 15.0 * 10.0 + 16.0 * 14.0,
                    13.0 * 3.0 + 14.0 * 7.0 + 15.0 * 11.0 + 16.0 * 15.0,
                    13.0 * 4.0 + 14.0 * 8.0 + 15.0 * 12.0 + 16.0 * 16.0,
                ],
            ]
        };

        assert_eq!(m0 * m0, res);
    }

    #[test]
    fn test_mul_mat4_assign() {
        let mut m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        m0 *= m0;

        let res = Mat4 {
            m: [
                [
                    1.0 * 1.0 + 2.0 * 5.0 + 3.0 * 9.0 + 4.0 * 13.0,
                    1.0 * 2.0 + 2.0 * 6.0 + 3.0 * 10.0 + 4.0 * 14.0,
                    1.0 * 3.0 + 2.0 * 7.0 + 3.0 * 11.0 + 4.0 * 15.0,
                    1.0 * 4.0 + 2.0 * 8.0 + 3.0 * 12.0 + 4.0 * 16.0,
                ],
                [
                    5.0 * 1.0 + 6.0 * 5.0 + 7.0 * 9.0 + 8.0 * 13.0,
                    5.0 * 2.0 + 6.0 * 6.0 + 7.0 * 10.0 + 8.0 * 14.0,
                    5.0 * 3.0 + 6.0 * 7.0 + 7.0 * 11.0 + 8.0 * 15.0,
                    5.0 * 4.0 + 6.0 * 8.0 + 7.0 * 12.0 + 8.0 * 16.0,
                ],
                [
                    9.0 * 1.0 + 10.0 * 5.0 + 11.0 * 9.0 + 12.0 * 13.0,
                    9.0 * 2.0 + 10.0 * 6.0 + 11.0 * 10.0 + 12.0 * 14.0,
                    9.0 * 3.0 + 10.0 * 7.0 + 11.0 * 11.0 + 12.0 * 15.0,
                    9.0 * 4.0 + 10.0 * 8.0 + 11.0 * 12.0 + 12.0 * 16.0,
                ],
                [
                    13.0 * 1.0 + 14.0 * 5.0 + 15.0 * 9.0 + 16.0 * 13.0,
                    13.0 * 2.0 + 14.0 * 6.0 + 15.0 * 10.0 + 16.0 * 14.0,
                    13.0 * 3.0 + 14.0 * 7.0 + 15.0 * 11.0 + 16.0 * 15.0,
                    13.0 * 4.0 + 14.0 * 8.0 + 15.0 * 12.0 + 16.0 * 16.0,
                ],
            ]
        };

        assert_eq!(m0, res);
    }

    #[test]
    fn test_div() {
        let m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let res = Mat4 {
            m: [
                [0.5, 1.0, 1.5, 2.0],
                [2.5, 3.0, 3.5, 4.0],
                [4.5, 5.0, 5.5, 6.0],
                [6.5, 7.0, 7.5, 8.0],
            ]
        };

        assert_eq!(m0 / 2.0, res);
    }

    #[test]
    fn test_div_assign() {
        let mut m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        m0 /= 2.0;

        let res = Mat4 {
            m: [
                [0.5, 1.0, 1.5, 2.0],
                [2.5, 3.0, 3.5, 4.0],
                [4.5, 5.0, 5.5, 6.0],
                [6.5, 7.0, 7.5, 8.0],
            ]
        };

        assert_eq!(m0, res);
    }

    #[test]
    fn test_neg() {
        let m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let res = Mat4 {
            m: [
                [-1.0, -2.0, -3.0, -4.0],
                [-5.0, -6.0, -7.0, -8.0],
                [-9.0, -10.0, -11.0, -12.0],
                [-13.0, -14.0, -15.0, -16.0],
            ]
        };

        assert_eq!(-m0, res);
    }

    #[test]
    fn test_transpose() {
        let m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        let res = Mat4 {
            m: [
                [1.0, 5.0, 9.0, 13.0],
                [2.0, 6.0, 10.0, 14.0],
                [3.0, 7.0, 11.0, 15.0],
                [4.0, 8.0, 12.0, 16.0],
            ]
        };

        assert_eq!(m0.transposed(), res);

        let mut m0 = Mat4 {
            m: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
        };

        m0.transpose();
        assert_eq!(m0, res);
    }

    #[test]
    fn test_ortho() {
        let m = ortho(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);

        let res = Mat4 {
            m: [
                [ 1.0, 0.0,  0.0, 0.0 ],
                [ 0.0, 1.0,  0.0, 0.0 ],
                [ 0.0, 0.0, -1.0, 0.0 ],
                [ 0.0, 0.0,  0.0, 1.0 ],
            ]
        };

        assert_eq!(m, res);
    }
}
