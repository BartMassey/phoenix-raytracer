// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Thing structure. Things are scene objects.

use shapes::Shape;
use textures::Texture;
use xform::*;

/// A thing has a shape, a texture and a transform that defines
/// its position, size and orientation.
pub struct Thing<'a> {
    /// Shape of thing.
    pub shape: &'a dyn Shape,
    pub texture: &'a dyn Texture,
    pub xform: XForm
}
