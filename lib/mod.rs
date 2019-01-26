// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Library for raytracing. This contains most of the
//! functionality of a raytracer.

mod infra;

pub mod frandom;
pub use self::frandom::*;

pub mod color;
pub use self::color::*;

pub mod point;
pub use self::point::*;

pub mod xform;
pub use self::xform::*;

pub mod light;
pub use self::light::*;

pub mod textures;
pub use self::textures::texture::Texture;

pub mod shapes;
pub use self::shapes::shape::Shape;

pub mod thing;
pub use self::thing::*;

pub mod model;
pub use self::model::*;
