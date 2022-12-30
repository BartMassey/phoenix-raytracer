pub use point_light::*;

use Point;

pub trait Light {
    fn at(&self) -> Point;
    fn i(&self) -> Point;
}

impl Light for PointLight {
    fn at(&self) -> Point {
        self.posn
    }

    fn i(&self) -> Point {
        self.intensity
    }
}
