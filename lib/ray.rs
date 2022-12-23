// Simple ray type.

use point::Point;
use xform::XForm;

#[derive(Clone)]
pub struct Ray {
    pub ro: Point,
    pub rd: Point,
}

impl Ray {
    fn new(ro: &Point, rd: &Point) -> Self {
        let mut result = Self {
            ro,
            rd,
        };
        result.rd.unitize();
        result
    }

    fn transform(&mut self, t: &XForm) {
        self.ro.transform(t);
        self.rd.transform(t);
        let mut dorg = Point::new(0.0, 0.0, 0.0);
        dorg.transform(t);
        self.rd -= dorg;
    }

    fn at(&self, t: f64) -> Point {
        self.rd * t + self.ro
    }
}
