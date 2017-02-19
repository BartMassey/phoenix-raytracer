// Points (from 1991)

use infra::TINY;

use std::ops::{Mul, Neg, Add, Sub,
               MulAssign, AddAssign, SubAssign,
               Index, IndexMut};

#[derive(Clone)]
pub struct Point {
    c: Vec<f64>
}

impl Point {

    pub fn new(cs: Vec<f64>) -> Self {
        Point { c : cs.clone() }
    }

    pub fn negate(&mut self) {
        for i in 0..self.c.len() {
            self.c[i] = -self.c[i];
        }
    }

    pub fn cross_product(&self, p: &Self) -> Self {
        assert!(self.c.len() == 3 && p.c.len() == 3 );
        let r: Vec<f64> =
            vec![
             self.c[1] * p.c[2] - self.c[2] * p.c[1],
             self.c[2] * p.c[0] - self.c[0] * p.c[2],
             self.c[0] * p.c[1] - self.c[1] * p.c[0]];
        Point::new(r)
    }

    pub fn pointwise_product(&self, p: &Self) -> Self {
        let nc = self.c.len();
        assert_eq!(nc, p.c.len());
        let mut r: Vec<f64> =  Vec::new();
        for i in 0..nc {
            r.push(self.c[i] * p.c[i]);
        };
        Point::new(r)
    }

    pub fn mag2(&self) -> f64 {
        let mut r = 0.0;
        for i in 0..self.c.len() {
            let c = self.c[i];
            r += c * c;
        };
        r
    }

    pub fn mag(&self) -> f64 {
        self.mag2().sqrt()
    }

    pub fn unitize(&mut self) {
        let m = self.mag();
        assert!( m > TINY );
        *self *= 1.0 / m;
    }

    pub fn unit(&self) -> Self {
        let mut r = self.clone();
        r.unitize();
        r
    }

    pub fn homogenize(&mut self) {
        assert!(self.c.len() == 4);
        *self *= 1.0 / self.c[3];
    }

    pub fn homogeneous(&self) -> Self {
        let mut r = self.clone();
        r.homogenize();
        r
    }

    pub fn dilation(&self, nc: f64) -> Self {
        let mut r = self.clone();
        r.c.push(nc);
        r
    }


    pub fn dilate(&mut self, nc: f64) {
        assert!(self.c.len() < 4);
        self.c.push(nc);
    }

    pub fn contraction(&self) -> Self {
        let mut r = self.clone();
        let _ = r.c.pop();
        r
    }

    pub fn contract(&mut self) {
        let _ = self.c.pop();
    }

    pub fn len(&self) -> usize {
        self.c.len()
    }
}

impl Index<usize> for Point {
    type Output = f64;
    
    fn index(&self, i: usize) -> &f64 {
        &self.c[i]
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.c[i]
    }
}

impl Mul for Point {
    type Output = f64;
    
    // Point multiplication is dot product.
    fn mul(self, rhs: Self) -> f64 {
        let mut r: f64 = 0.0;
        let nc = self.c.len();
        assert_eq!(nc, rhs.c.len());
        for i in 0..nc {
            r += self.c[i] * rhs.c[i];
        };
        r
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    
    fn mul(self, rhs: f64) -> Self {
        let mut tmp = self.clone();
        tmp *= rhs;
        tmp
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        let mut tmp = self.clone();
        tmp.negate();
        tmp
    }
}


impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp += rhs;
        tmp
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut tmp = self.clone();
        tmp -= rhs;
        tmp
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.c.len() {
            self.c[i] *= rhs;
        };
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        let nc = self.c.len();
        assert_eq!(nc, rhs.c.len());
        for i in 0..nc {
            self.c[i] += rhs.c[i];
        };
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

pub const X: usize = 0;
pub const Y: usize = 1;
pub const Z: usize = 2;
pub const W: usize = 3;
pub const R: usize = 0;
pub const G: usize = 1;
pub const B: usize = 2;
