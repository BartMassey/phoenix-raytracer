use std::ops::{Mul, MulAssign};

use crate::*;

// Convenience type for 4x4 matrices.
type XFMatrix = [[f64; 4]; 4];

/// A transformation has a forward matrix and an inverse
/// matrix, maintained in parallel for efficiency and ease
/// of implementation.
#[derive(Clone, Default)]
pub struct Xform {
    /// Forward transformation matrix.
    pub m: XFMatrix,
    /// Inverse transformation matrix.
    pub mi: XFMatrix,
}

impl Xform {
    /// Identity transformation.
    pub fn identity() -> Self {
        let mut x = Xform::default();
        for r in 0..4 {
            for c in 0..4 {
                if r == c {
                    x.m[r][c] = 1.0;
                    x.mi[r][c] = 1.0;
                };
            }
        }
        x
    }

    /// Rotation transformation around the x axis by given
    /// angle in radians.
    pub fn rotation_x(angle: f64) -> Self {
        let ca = angle.cos();
        let sa = angle.sin();
        let mut x = Xform::default();
        x.m[0][0] = 1.0;
        x.m[3][3] = 1.0;
        x.m[1][1] = ca;
        x.m[2][2] = ca;
        x.m[1][2] = -sa;
        x.m[2][1] = sa;
        x.mi[0][0] = 1.0;
        x.mi[3][3] = 1.0;
        x.mi[1][1] = ca;
        x.mi[2][2] = ca;
        x.mi[1][2] = sa;
        x.mi[2][1] = -sa;
        x
    }

    /// Rotation transformation around the y axis by given
    /// angle in radians.
    pub fn rotation_y(angle: f64) -> Self {
        let ca = angle.cos();
        let sa = angle.sin();
        let mut x = Xform::default();
        x.m[1][1] = 1.0;
        x.m[3][3] = 1.0;
        x.m[0][0] = ca;
        x.m[2][2] = ca;
        x.m[2][0] = -sa;
        x.m[0][2] = sa;
        x.mi[1][1] = 1.0;
        x.mi[3][3] = 1.0;
        x.mi[0][0] = ca;
        x.mi[2][2] = ca;
        x.mi[2][0] = sa;
        x.mi[0][2] = -sa;
        x
    }

    /// Rotation transformation around the z axis by given
    /// angle in radians.
    pub fn rotation_z(angle: f64) -> Self {
        let ca = angle.cos();
        let sa = angle.sin();
        let mut x = Xform::default();
        x.m[2][2] = 1.0;
        x.m[3][3] = 1.0;
        x.m[0][0] = ca;
        x.m[1][1] = ca;
        x.m[0][1] = -sa;
        x.m[1][0] = sa;
        x.mi[2][2] = 1.0;
        x.mi[3][3] = 1.0;
        x.mi[0][0] = ca;
        x.mi[1][1] = ca;
        x.mi[0][1] = sa;
        x.mi[1][0] = -sa;
        x
    }

    /// Translation transformation by given offset.
    pub fn translation(trans: &Point) -> Self {
        assert!(trans.len() == 3);
        let mut x = Xform::identity();
        x.m[0][3] = trans[0];
        x.m[1][3] = trans[1];
        x.m[2][3] = trans[2];
        x.mi[0][3] = -trans[0];
        x.mi[1][3] = -trans[1];
        x.mi[2][3] = -trans[2];
        x
    }

    /// Scaling transformation by given scale.
    pub fn scaling(scale: &Point) -> Self {
        assert!(scale.len() == 3);
        let mut x = Xform::default();
        x.m[0][0] = scale[0];
        x.m[1][1] = scale[1];
        x.m[2][2] = scale[2];
        x.m[3][3] = 1.0;
        x.mi[0][0] = 1.0 / scale[0];
        x.mi[1][1] = 1.0 / scale[1];
        x.mi[2][2] = 1.0 / scale[2];
        x.mi[3][3] = 1.0;
        x
    }

    /// Invert the transformation by exchanging the
    /// forward and inverse matrices.
    pub fn invert(&mut self) {
        std::mem::swap(&mut self.m, &mut self.mi);
    }

    /// Produce the inverse of a transformation with
    /// the forward and inverse matrices interchanged.
    pub fn inverse(&self) -> Self {
        let mut tmp = self.clone();
        tmp.invert();
        tmp
    }
}

impl Mul<&Point> for Xform {
    type Output = Point;

    /// Apply the transformation to the given Point
    /// by matrix-vector multiplication.
    fn mul(self, rhs: &Point) -> Point {
        assert!(rhs.len() == 4);
        let mut t = Point::new([0.0; 4]);
        for r in 0..4 {
            for c in 0..4 {
                t[r] += self.m[r][c] * rhs[c];
            }
        }
        t
    }
}

impl Mul<&Point> for &Xform {
    type Output = Point;

    /// Apply the transformation to the given Point
    /// by matrix-vector multiplication.
    fn mul(self, rhs: &Point) -> Point {
        assert!(rhs.len() == 4);
        let mut t = Point::new([0.0; 4]);
        for r in 0..4 {
            for c in 0..4 {
                t[r] += self.m[r][c] * rhs[c];
            }
        }
        t
    }
}

impl Mul<&Xform> for Xform {
    type Output = Xform;

    /// Compose two transformations by matrix multiplication
    /// of the forward transformation matrix and reversed
    /// matrix multiplication of the inverse transformation
    /// matrix using `*` notation.
    fn mul(self, rhs: &Xform) -> Xform {
        let mut t = Xform::default();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    t.m[i][j] += self.m[i][k] * rhs.m[k][j];
                    t.mi[i][j] += rhs.mi[i][k] * self.mi[k][j];
                }
            }
        }
        t
    }
}

impl MulAssign<&Xform> for Xform {
    /// Compose the transformation with a given
    /// transformation by matrix multiplication of the
    /// forward transformation matrix and reversed matrix
    /// multiplication of the inverse transformation matrix
    /// using `*=` notation.
    fn mul_assign(&mut self, rhs: &Xform) {
        let tmp = self.clone();
        for i in 0..4 {
            for j in 0..4 {
                self.m[i][j] = tmp.m[i][0] * rhs.m[0][j];
                self.mi[i][j] = tmp.mi[i][0] * rhs.mi[0][j];
                for k in 1..4 {
                    self.m[i][j] += tmp.m[i][k] * rhs.m[k][j];
                    self.mi[i][j] += tmp.mi[i][k] * rhs.mi[k][j];
                }
            }
        }
    }
}
