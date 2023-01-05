pub mod point_light;
pub use point_light::*;

use crate::*;

pub trait Light: Send + Sync {
    fn at(&self) -> Point;
    fn i(&self) -> Color;
}
