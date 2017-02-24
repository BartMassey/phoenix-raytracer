// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Generic interface to shapes.

pub use point::Point;

pub trait Shape {
    /// Given a ray specified by origin and unit direction,
    /// return the location and unit direction of intersection
    /// in texture coordinates, if intersection happens.
    fn intersect(&self, at: &Point, dirn: &Point) -> Option<Point>;
}
