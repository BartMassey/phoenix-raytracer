// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Light structure.

use color::Color;
use point::Point;

/// Information about a light source. Right now this is a
/// point source.
pub struct Light {
    /// Position of light source.
    pub posn: Point,
    /// Luminance of light source.
    pub color: Color
}
