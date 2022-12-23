// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Library for raytracing. This contains most of the
//! functionality of a raytracer.

mod infra;

pub mod frandom;
pub mod color;
pub mod point;
pub mod xform;
pub mod light;
pub mod textures;
pub mod shapes;
pub mod thing;
pub mod model;
pub mod ray;

pub use self::frandom::*;
pub use self::color::*;
pub use self::point::*;
pub use self::xform::*;
pub use self::light::*;
pub use self::textures::texture::Texture;
pub use self::shapes::shape::Shape;
pub use self::thing::*;
pub use self::model::*;
pub use self::ray::*;
