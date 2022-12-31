pub mod point_light;
pub use point_light::*;

use crate::*;

pub trait Light {
    fn at(&self) -> Point;
    fn i(&self) -> Color;
}
