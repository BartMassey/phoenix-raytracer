// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Light structure.

use color::Color;
use point::Point;

/// Scene model for rendering. So far, stored as naïve arrays,
/// rather than octrees or something.
pub struct Light {
    pub posn: Point,
    pub color: Color
}
