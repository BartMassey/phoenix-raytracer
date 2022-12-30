// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Implementation of "points"; actually, short vectors of
//! floating-point numbers. Based on the 1991
//! implementation.

use infra::TINY;

use std::ops::{Mul, Neg, Add, Sub,
               MulAssign, AddAssign, SubAssign,
               Index, IndexMut};

/// Symbolic representation of x coordinate index of a point.
pub const X: usize = 0;
/// Symbolic representation of y coordinate index of a point.
pub const Y: usize = 1;
/// Symbolic representation of z coordinate index of a point.
pub const Z: usize = 2;
/// Symbolic representation of w coordinate index of a
/// 4-coordinate point.  (Homogeneous coordinate.)
pub const W: usize = 3;

/// A Point has dynamic length, which means care
/// is required in using it.
#[derive(Clone, Copy)]
pub struct Point {
    /// Coordinates of the Point.
    pub c: Vec<f64>,
}

/// Operations on Points.
impl Point {

    /// Create a new Point from an array of floats.
    /// (This should probably be a macro.)
    pub fn new<N: usize>(cs: [f64; N]) -> Self {
        Point { c : cs.to_vec() }
    }

    /// Convert the Point to its coordinate-wise negation.
    pub fn negate(&mut self) {
        for i in 0..self.c.len() {
            self.c[i] = -self.c[i];
        }
    }

    /// Cross product of two Points, which should be
    /// homogenous. (Operator `*` is dot product.)
    pub fn cross_product(&self, p: &Self) -> Self {
        assert!(self.c.len() == 3 && p.c.len() == 3 );
        let r = [
          self.c[1] * p.c[2] - self.c[2] * p.c[1],
          self.c[2] * p.c[0] - self.c[0] * p.c[2],
          self.c[0] * p.c[1] - self.c[1] * p.c[0],
          1.0 ];
        Point::new(r)
    }

    /// Square of Euclidean magnitude of a Point.
    pub fn mag2(&self) -> f64 {
        let mut r = 0.0;
        for i in 0..self.c.len() {
            let c = self.c[i];
            r += c * c;
        };
        r
    }

    /// Euclidean magnitude of a Point.
    pub fn mag(&self) -> f64 {
        self.mag2().sqrt()
    }

    /// Convert the Point to a unit (direction) vector.
    pub fn unitize(&mut self) {
        let m = self.mag();
        assert!( m > TINY );
        *self *= 1.0 / m;
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
        assert!(self.c.len() == 4);
        *self *= 1.0 / self.c[3];
    }

    /// Homogenous representation of a 4-coordinate
    /// Point. (Scaled so that final coordinate is 1.)
    pub fn homogeneous(&self) -> Self {
        let mut r = self.clone();
        r.homogenize();
        r
    }

    /// Return the number of coordinates of the Point.
    pub fn len(&self) -> usize {
        self.c.len()
    }
}

impl Index<usize> for Point {
    type Output = f64;
    
    /// Return the given coordinate of a Point using `[]`
    /// subscripting.
    fn index(&self, i: usize) -> &f64 {
        &self.c[i]
    }
}

impl IndexMut<usize> for Point {
    /// Assign the given coordinate of a Point using `[]`
    /// subscripting.
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.c[i]
    }
}

impl Mul for Point {
    type Output = f64;
    
    /// Return the dot product of two Points using `*` notation.
    /// The points should be homogenous: the z coordinate is ignored.
    fn mul(self, rhs: Self) -> f64 {
        let mut r: f64 = 0.0;
        for i in 0..self.c.len()-1 {
            r += self.c[i] * rhs.c[i];
        };
        r
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    
    /// Return the coordinate-wise product of a Point and
    /// the given scale using `*` notation.
    fn mul(self, rhs: f64) -> Self {
        let mut tmp = self.clone();
        tmp *= rhs;
        tmp
    }
}

impl Neg for Point {
    type Output = Self;

    /// Return the negation of a Point using unary `-`
    /// notation.
    fn neg(self) -> Self {
        let mut tmp = self.clone();
        tmp.negate();
        tmp
    }
}


impl Add for Point {
    type Output = Self;

    /// Return the coordinate-wise sum of two Points using
    /// `+` notation.
    fn add(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp += rhs;
        tmp
    }
}

impl Sub for Point {
    type Output = Self;

    /// Return the coordinate-wise difference of two Points
    /// using `-` notation.
    fn sub(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= rhs;
        tmp
    }
}

impl MulAssign<f64> for Point {
    /// Multiply the point by the given scalar using `*=`
    /// notation.
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.c.len() {
            self.c[i] *= rhs;
        };
    }
}

impl AddAssign for Point {
    /// Adjust each coordinate of the Point by adding the
    /// corresponding coordinate of the given Point using `+=`
    /// notation. (Vector sum.)
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.c.len() {
            self.c[i] += rhs.c[i];
        };
    }
}

impl SubAssign for Point {
    /// Adjust each coordinate of the Point by subtracting
    /// the corresponding coordinate of the given Point
    /// using `-=` notation. (Vector sum.)
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}
