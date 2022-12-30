// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Light structure.

use point::Point;

/// Information about a point light source.
pub struct PointLight {
    /// Position of light source.
    pub loc: Point,
    /// Luminance of light source.
    pub intensity: Point,
}
