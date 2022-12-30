use crate::*;

/// A sphere of unit radius at a given position.
#[derive(Default)]
pub struct Sphere {
    /// Position of sphere.
    tr: Point,
}

impl Shape for Sphere {
    /// Iff the incoming ray is pointing in the right direction
    /// and hits the sphere, return a homogeneous
    /// point representing its xy coordinate.
    fn intersect(&self, xform: &Xform, ray: &Ray) -> Option<Intersection> {
        // Put the ray in our coords…
        let mut r = ray.clone();
        let toi = &xform.mi;
        r.transform(toi);

        let a = r.d.mag2();
        let b = r.o * r.d;
        let c = r.o.mag2() - 1.0;
        let d = b * b - a * c;

        if d < TINY  {
            // The ray misses the sphere, so no hit.
            return None;
        }

        let t = if a < 0  {
            (-b + d.sqrt()) / a
        } else {
            (-b - d.sqrt()) / a
        };
        if( t < TINY ) {
            // The ray is travelling away from the sphere, so no hit.
            return None;
        }

        // Find the intersection point in object coords.
        let mut i = r.o + r.d * t;
        i.transform(toi);

        // There are many possible mappings -- here's a lame one
        Some(Intersection {
            t,
            at: i,
            normal: (i - tr).unit(),
        })
    }
}
