// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Implementation of "points"; actually, short vectors of
//! floating-point numbers. Based on the 1991
//! implementation.

use std::ops::*;

use crate::*;

/// A Point has dynamic length, which means care
/// is required in using it.
#[derive(Clone)]
pub struct Point(nalgebra::DVector<f64>);

macro_rules! coord {
    ($id:ident, $idx:literal) => {
        pub fn $id(&self) -> f64 {
            *self.get($idx)
        }
    };
}

/// Operations on Points.
impl Point {
    /// Create a new Point from an array of floats.
    pub fn new<const N: usize>(cs: [f64; N]) -> Self {
        Point(nalgebra::DVector::from_row_slice(&cs))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn get(&self, i: usize) -> &f64 {
        &self.0[i]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.0[i]
    }

    /// Convert the Point to its coordinate-wise negation.
    pub fn negate(&mut self) {
        let p = Point(-self.0.clone());
        *self = p;
    }

    /// Cross product of two Points, which should be dim 3.
    pub fn cross_product(&self, rhs: &Self) -> Self {
        assert!(self.len() == 3);
        assert!(rhs.len() == 3);
        let mut p = self.0.cross(&rhs.0);
        p.resize_vertically_mut(4, 1.0);
        Point(p)
    }

    /// Square of Euclidean magnitude of a Point.
    pub fn mag2(&self) -> f64 {
        self.0.dot(&self.0)
    }

    /// Euclidean magnitude of a Point.
    pub fn mag(&self) -> f64 {
        self.mag2().sqrt()
    }

    /// Convert the Point to a unit (direction) vector.
    pub fn unitize(&mut self) {
        let m = self.mag();
        assert!(m > TINY);
        self.0 /= m;
    }

    /// Return the unit (direction) vector of a Point.
    pub fn unit(&self) -> Self {
        let mut r = self.clone();
        r.unitize();
        r
    }

    /// Convert the 4-coordinate Point to a homogenous
    /// representation. (Scale so that final coordinate is 1.)
    pub fn homogenize(&mut self) {
        assert!(self.len() == 4);
        self.0 /= self.w();
    }

    /// Homogenous representation of a 4-coordinate
    /// Point. (Scaled so that final coordinate is 1.)
    pub fn homogeneous(&self) -> Self {
        let mut r = self.clone();
        r.homogenize();
        r
    }

    pub fn transform(&mut self, t: &Xform) {
        let nself = self.len();
        assert!(nself <= 4);
        if nself < 4 {
            self.0.resize_vertically_mut(4, 0.0);
            self.0[3] = 1.0;
        }
        let mut x = t * self;
        x.0.resize_vertically_mut(nself, 0.0);
        *self = x;
    }

    coord!(x, 0);
    coord!(y, 1);
    coord!(z, 2);
    coord!(w, 3);
}

impl Index<usize> for Point {
    type Output = f64;

    /// Return the given coordinate of a Point using `[]`
    /// subscripting.
    fn index(&self, i: usize) -> &f64 {
        self.get(i)
    }
}

impl IndexMut<usize> for Point {
    /// Assign the given coordinate of a Point using `[]`
    /// subscripting.
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        self.get_mut(i)
    }
}

impl Mul for &Point {
    type Output = f64;

    /// Return the dot product of two Points using `*` notation.
    /// The points should be homogenous.
    fn mul(self, rhs: Self) -> f64 {
        assert_eq!(self.len(), rhs.len());
        self.0.dot(&rhs.0)
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    /// Return the coordinate-wise product of a Point and
    /// the given scale using `*` notation.
    fn mul(self, rhs: f64) -> Self::Output {
        Point(&self.0 * rhs)
    }
}

impl Neg for Point {
    type Output = Self;

    /// Return the negation of a Point using unary `-`
    /// notation.
    fn neg(self) -> Self {
        Point(-self.0)
    }
}

impl Add for Point {
    type Output = Self;

    /// Return the coordinate-wise sum of two Points using
    /// `+` notation.
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0)
    }
}

impl Sub for Point {
    type Output = Self;

    /// Return the coordinate-wise difference of two Points
    /// using `-` notation.
    fn sub(self, rhs: Self) -> Self {
        Point(self.0 - rhs.0)
    }
}

impl MulAssign<f64> for Point {
    /// Multiply the point by the given scalar using `*=`
    /// notation.
    fn mul_assign(&mut self, rhs: f64) {
        let p = &*self * rhs;
        *self = p;
    }
}

impl AddAssign for Point {
    /// Adjust each coordinate of the Point by adding the
    /// corresponding coordinate of the given Point using `+=`
    /// notation. (Vector sum.)
    fn add_assign(&mut self, rhs: Point) {
        let p = Point(&self.0 + rhs.0);
        *self = p;
    }
}

impl SubAssign for Point {
    /// Adjust each coordinate of the Point by subtracting
    /// the corresponding coordinate of the given Point
    /// using `-=` notation. (Vector sum.)
    fn sub_assign(&mut self, rhs: Self) {
        let p = Point(&self.0 - rhs.0);
        *self = p;
    }
}
