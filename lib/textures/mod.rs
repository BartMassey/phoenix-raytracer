pub mod solid_texture;
pub use self::solid_texture::*;

pub mod board_texture;
pub use self::board_texture::*;

use crate::*;

pub trait Texture {
    /// Given the scene model, a texture coordinate (usually just x and y),
    /// a unit vector pointing in the direction of intersection and
    /// an indication of how much recursion depth remains, return
    /// the color of the resulting ray.
    fn value(
        &self,
        at: &Point,
        gc: &Point,
        normal: &Point,
        m: &Model,
        depth: usize
    ) -> Color;
}
