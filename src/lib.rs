//! Library for raytracing. This contains most of the
//! functionality of a raytracer.

pub mod color;
pub mod frandom;
pub mod infra;
pub mod lights;
pub mod model;
pub mod outputs;
pub mod point;
pub mod ray;
pub mod render;
pub mod shapes;
pub mod textures;
pub mod thing;
pub mod xform;

pub use color::*;
pub use frandom::*;
pub use infra::*;
pub use lights::*;
pub use model::*;
pub use outputs::*;
pub use point::*;
pub use ray::*;
pub use render::*;
pub use shapes::*;
pub use textures::*;
pub use thing::*;
pub use xform::*;
