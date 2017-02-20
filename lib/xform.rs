// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! 4x4 transformations. Based very loosely on the
//! 1991 implementation.

use std::ops::{Mul, MulAssign};

use point::Point;

/// Convenience type for 4x4 matrices.
type XFMatrix = [[f64; 4]; 4];

/// A transformation has a forward matrix and an inverse
/// matrix, maintained in parallel for efficiency and ease
/// of implementation.
#[derive(Clone)]
pub struct XForm {
    /// Forward transformation matrix.
    m: XFMatrix,  
    /// Inverse transformation matrix.
    mi: XFMatrix
}

/// All-zeros transformation matrix.
const NULL_MATRIX: XFMatrix = [[0.0; 4]; 4];
/// All-zeros transform.
const NULL_XFORM: XForm =
    XForm { m: NULL_MATRIX, mi: NULL_MATRIX };

impl XForm {

    /// Identity transformation.
    pub fn identity() -> Self {
        let mut x = NULL_XFORM;
        for r in 0..4 {
            for c in 0..4 {
                if r == c {
                    x.m[r][c] = 1.0;
                    x.mi[r][c] = 1.0;
                };
            };
        };
        x
    }

    /// Rotation transformation around the x axis by given
    /// angle in radians.
    pub fn rotation_x(angle: f64) -> Self {
        let ca = angle.cos();
        let sa = angle.sin();
        let mut x = NULL_XFORM;
        x.m[0][0] = 1.0;
        x.m[1][1] = ca;
        x.m[2][2] = ca;
        x.m[3][3] = 1.0;
        x.m[1][2] = -sa;
        x.m[2][1] = sa;
        x.mi[0][0] = 1.0;
        x.mi[1][1] = ca;
        x.mi[2][2] = ca;
        x.mi[3][3] = 1.0;
        x.mi[1][2] = sa;
        x.mi[2][1] = -sa;
        x
    }

    /// Rotation transformation around the y axis by given
    /// angle in radians.
    pub fn rotation_y(angle: f64) -> Self {
        let ca = angle.cos();
        let sa = angle.sin();
        let mut x = NULL_XFORM;
        x.m[0][0] = ca;
        x.m[1][1] = 1.0;
        x.m[2][2] = ca;
        x.m[3][3] = 1.0;
        x.m[0][2] = sa;
        x.m[2][0] = -sa;
        x.mi[0][0] = ca;
        x.mi[1][1] = 1.0;
        x.mi[2][2] = ca;
        x.mi[3][3] = 1.0;
        x.mi[0][2] = -sa;
        x.mi[2][0] = sa;
        x
    }

    /// Rotation transformation around the z axis by given
    /// angle in radians.
    pub fn rotation_z(angle: f64) -> Self {
        let ca = angle.cos();
        let sa = angle.sin();
        let mut x = NULL_XFORM;
        x.m[0][0] = ca;
        x.m[1][1] = ca;
        x.m[2][2] = 1.0;
        x.m[3][3] = 1.0;
        x.m[0][1] = -sa;
        x.m[1][0] = sa;
        x.mi[0][0] = ca;
        x.mi[1][1] = ca;
        x.mi[2][2] = 1.0;
        x.mi[3][3] = 1.0;
        x.mi[0][1] = sa;
        x.mi[1][0] = -sa;
        x
    }

    /// Translation transformation by given offset.
    pub fn translation(trans: Point) -> Self {
        assert!(trans.len() == 3);
        let mut x = NULL_XFORM;
        x.m[0][0] = 1.0;
        x.m[1][1] = 1.0;
        x.m[2][2] = 1.0;
        x.m[3][3] = 1.0;
        x.m[0][3] = trans[0];
        x.m[1][3] = trans[1];
        x.m[2][3] = trans[2];
        x.mi[0][0] = 1.0;
        x.mi[1][1] = 1.0;
        x.mi[2][2] = 1.0;
        x.mi[3][3] = 1.0;
        x.m[0][3] = -trans[0];
        x.m[1][3] = -trans[1];
        x.m[2][3] = -trans[2];
        x
    }

    /// Scaling transformation by given scale.
    pub fn scaling(scale: Point) -> Self {
        assert!(scale.len() == 3);
        let mut x = NULL_XFORM;
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
        let tmp = self.m;
        self.m = self.mi;
        self.mi = tmp;
    }

    /// Produce the inverse of a transformation with
    /// the forward and inverse matrices interchanged.
    pub fn inverse(&self) -> Self {
        let mut tmp = self.clone();
        tmp.invert();
        tmp
    }
}

impl Mul<Point> for XForm {
    type Output = Point;

    /// Apply the transformation to the given Point
    /// by matrix-vector multiplication.
    fn mul(self, rhs: Point) -> Point {
        assert!(rhs.len() == 4);
        let mut t = Point::new(vec![0.0;4]);
        for r in 0..4 {
            for c in 0..4 {
                t[r] += self.m[r][c] * rhs[c];
            }
        };
        t
    }
}

impl Mul for XForm {
    type Output = XForm;

    /// Compose two transformations by matrix multiplication
    /// of the forward transformation matrix and reversed
    /// matrix multiplication of the inverse transformation
    /// matrix using `*` notation.
    fn mul(self, rhs: XForm) -> XForm {
        let mut t = NULL_XFORM;
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    t.m[i][j] += self.m[i][k] * rhs.m[k][j];
                    t.mi[i][j] += rhs.mi[i][k] * self.mi[k][j];
                }
            }
        };
        t
    }
}

impl MulAssign for XForm {
    /// Compose the transformation with a given
    /// transformation by matrix multiplication of the
    /// forward transformation matrix and reversed matrix
    /// multiplication of the inverse transformation matrix
    /// using `*=` notation.
    fn mul_assign(&mut self, rhs: XForm) {
        let mut t = NULL_XFORM;
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    t.m[i][j] += self.m[i][k] * rhs.m[k][j];
                    t.mi[i][j] += rhs.mi[i][k] * self.mi[k][j];
                }
            }
        };
        *self = t;
    }
}
