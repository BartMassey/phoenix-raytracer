use crate::*;

/// A sphere of unit radius at a given position.
pub struct Sphere {
    /// Position of sphere.
    tr: Point,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere { tr: Point::new([0.0, 0.0, 0.0]) }
    }
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
        let c = r.ro.clone().mag2() - 1.0;
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
        let mut i = r.ro + r.rd * t;
        i.transform(&toi);

        // There are many possible mappings -- here's a lame one
        Some(Intersection {
            t,
            at: Point::new([i[X], i[Y]]),
            normal: (i - self.tr.clone()).unit(),
        })
    }

    fn complete(&mut self, xform: &Xform) {
        let mut o = Point::new([0.0, 0.0, 0.0]);
        o.transform(&xform.inverse());
        self.tr = o;
    }
}

#[test]
fn test_sphere_intersect() {
    let mut s = Sphere::default();
    let x = Point::new([0.0, 0.0, 3.0]);
    let xform = Xform::translation(&x);
    s.complete(&xform);

    let ray = Ray::new(
        Point::new([0.0, 0.0, 0.0]),
        Point::new([0.0, 0.0, 1.0]),
    );
    assert!(s.intersect(&xform, &ray).is_some());

    let ray = Ray::new(
        Point::new([0.0, 0.0, 0.0]),
        Point::new([1.0, 0.0, 1.0]).unit(),
    );
    assert!(s.intersect(&xform, &ray).is_none());
}
