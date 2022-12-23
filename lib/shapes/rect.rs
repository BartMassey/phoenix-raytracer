// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

use infra::TINY;
use point::{X,Y,Z};
use shapes::shape::*;

/// A rectangle of given width and height, centered on the
/// xy plane.
pub struct Rect {
    /// X-dimension of rectangle.
    width: f64,
    /// Y-dimension of rectangle.
    height: f64
}

impl Shape for Rect {
    /// Iff the incoming ray is pointing in the right direction
    /// and hits inside the rectangle, return a homogeneous
    /// point representing its xy coordinate.
    fn intersect(&self, ray: &Ray) -> Option<Ray> {
        let angle = -(*at) * *dirn;
        if angle < TINY {
            return None;
        };
        let ray = *dirn * at[Z];
        let x = ray[X];
        let y = ray[Y];
        if x < -self.width * 0.5 || x > self.width * 0.5 {
            return None;
        };
        if y < -self.height * 0.5 || y > self.height * 0.5 {
            return None;
        };
        Some(Point::new([x, y, 0.0, 1.0]))
    }
}
