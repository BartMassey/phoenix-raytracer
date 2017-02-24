// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Generic interface to textures.

pub use model::Model;
pub use point::Point;
pub use color::Color;

pub trait Texture {
    /// Given the scene model, a texture coordinate (usually just x and y),
    /// a unit vector pointing in the direction of intersection and
    /// an indication of how much recursion depth remains, return
    /// the color of the resulting ray.
    fn value(&self, m: &Model, at: &Point, dirn: &Point, depth: usize)
    -> Color;
}
