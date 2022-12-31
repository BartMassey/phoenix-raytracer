use crate::*;

#[derive(Clone)]
pub struct Ray {
    pub ro: Point,
    pub rd: Point,
}

impl Ray {
    pub fn new(ro: Point, rd: Point) -> Self {
        let mut result = Self {
            ro,
            rd,
        };
        result.rd.unitize();
        result
    }

    pub fn transform(&mut self, t: &Xform) {
        self.ro.transform(t);
        self.rd.transform(t);
        let mut dorg = Point::new([0.0, 0.0, 0.0]);
        dorg.transform(t);
        self.rd -= dorg;
    }

    pub fn at(&self, t: f64) -> Point {
        self.rd.clone() * t + self.ro.clone()
    }
}
