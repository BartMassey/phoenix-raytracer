// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Model structure.

use light::Light;
use thing::Thing;
use color::Color;

/// Scene model for rendering. So far, stored as naïve arrays,
/// rather than octrees or something.
pub struct Model<'a> {
    pub lights: Vec<Light>,
    pub things: Vec<Thing<'a>>,
    pub ambient: Color
}
