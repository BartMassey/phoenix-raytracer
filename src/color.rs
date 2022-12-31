//! Implementation of color as a 3-vector of floating-point
//! RGB values.

/// Symbolic representation of red coordinate index of a Color.
pub const R: usize = 0;
/// Symbolic representation of green coordinate index of a Color.
pub const G: usize = 1;
/// Symbolic representation of blue coordinate index of a Color.
pub const B: usize = 2;

use std::ops::{Mul, Add, Sub,
               MulAssign, AddAssign, SubAssign,
               Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Color {
    /// Elements of the Color.
    c: [f64; 3]
}

/// Operations on Colors.
impl Color {

    /// Create a new Color from components.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { c: [r, g, b] }
    }


    pub fn colorize(&self, c: &Color) -> Self {
        Self { c: [
            c.c[R] * self.c[R],
            c.c[G] * self.c[G],
            c.c[B] * self.c[B],
        ]}
    }

    pub fn apply<T, F>(&self, f: F) -> [T; 3]
        where F: Fn(f64) -> T
    {
        [f(self.c[R]), f(self.c[G]), f(self.c[B])]
    }
}

impl Index<usize> for Color {
    type Output = f64;
    
    /// Return the given coordinate of an Color using `[]`
    /// subscripting.
    fn index(&self, i: usize) -> &f64 {
        &self.c[i]
    }
}

impl IndexMut<usize> for Color {
    /// Assign the given coordinate of a Color using `[]`
    /// subscripting.
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.c[i]
    }
}

impl Mul for Color {
    type Output = Self;
    
    /// Coordinate-wise produce of two Colors.
    fn mul(self, other: Self) -> Self {
        let mut r = self;
        for i in 0..self.c.len() {
            r.c[i] *= other.c[i]
        };
        r
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    
    /// Return the coordinate-wise product of an Color and
    /// the given scale using `*` notation.
    fn mul(self, rhs: f64) -> Self {
        let mut tmp = self;
        tmp *= rhs;
        tmp
    }
}

impl Add for Color {
    type Output = Self;

    /// Return the coordinate-wise sum of two Colors using
    /// `+` notation.
    fn add(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp += rhs;
        tmp
    }
}

impl Sub for Color {
    type Output = Self;

    /// Return the coordinate-wise difference of two Colors
    /// using `-` notation.
    fn sub(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp -= rhs;
        tmp
    }
}

impl MulAssign<f64> for Color {
    /// Multiply the point by the given scalar using `*=`
    /// notation.
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.c.len() {
            self.c[i] *= rhs;
        };
    }
}

impl AddAssign for Color {
    /// Adjust each coordinate of the Color by adding the
    /// corresponding coordinate of the given Color using `+=`
    /// notation. (Vector sum.)
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.c.len() {
            self.c[i] += rhs.c[i];
        };
    }
}

impl SubAssign for Color {
    /// Adjust each coordinate of the Color by subtracting
    /// the corresponding coordinate of the given Color
    /// using `-=` notation. (Vector sum.)
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..self.c.len() {
            self.c[i] -= rhs.c[i];
        };
    }
}
