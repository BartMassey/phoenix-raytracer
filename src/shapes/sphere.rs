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
        let toi = xform.inverse();
        r.transform(&toi);

        let a = r.rd.clone().mag2();
        let b = r.ro.clone() * r.rd.clone();
        let c = r.ro.mag2() - 1.0;
        let d = b * b - a * c;

        if d < TINY  {
            // The ray misses the sphere, so no hit.
            return None;
        }

        let t = if a < 0.0  {
            (-b + d.sqrt()) / a
        } else {
            (-b - d.sqrt()) / a
        };
        if t < TINY {
            // The ray is travelling away from the sphere, so no hit.
            return None;
        }

        // Find the intersection point in object coords.
        let i = r.ro + r.rd * t;
        let i = toi * &i;

        // There are many possible mappings -- here's a lame one
        Some(Intersection {
            t,
            at: i.clone(),
            normal: (i - self.tr.clone()).unit(),
        })
    }
}
