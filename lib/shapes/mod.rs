pub mod sphere;
pub use sphere::*;

pub mod poly;
pub use poly::*;

use crate::*;

pub struct Intersection {
  pub t: float,
  pub normal: Point,
  pub at: Point,
}

pub trait Shape {
    /// Given a ray specified by origin and unit direction,
    /// return the location and unit direction of intersection
    /// in texture coordinates, if intersection happens.
    fn intersect(&self, xform: &XForm, ray: &Ray) -> Option<Intersection>;
}
