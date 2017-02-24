// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

use textures::texture::*;

/// A flat texture: does not raytrace or shade, just
/// returns its color.
pub struct Flat {
    color: Color
}

impl Texture for Flat {
    /// Return the texture color, independent of everything.
    fn value(&self, _: &Model, _: &Point, _: &Point, _: usize)
             -> Color {
        self.color.clone()
    }
}
