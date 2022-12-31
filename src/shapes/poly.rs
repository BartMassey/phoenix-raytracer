use crate::*;

pub struct Poly {
    pub p: Vec<Point>,
    pub dp: usize,
    pub cnormal: Point,
}

impl Poly {
    pub fn new<const N: usize>(points: [Point; N]) -> Self {
        Self {
            p: points.into_iter().collect(),
            dp: 0,
            cnormal: Point::new([0.0, 0.0, 1.0]),
        }
    }
}

// Find which side of the line `origin`→`v1` `v2` is on.
fn side(origin: &Point, v1: &Point, v2: &Point) -> f64 {
    let c1 = (v1[X] - origin[X]) * (v2[Y] - origin[Y]);
    let c2 = (v2[X] - origin[X]) * (v1[Y] - origin[Y]);
    (c1 - c2).signum()
}

#[test]
fn test_side() {
    let origin = Point::new([0.0, 0.0]);
    let v1 = Point::new([1.0, 1.0]);
    let x = Point::new([1.0, 0.9]);
    let y = Point::new([0.9, 1.0]);
    assert_eq!(-1.0, side(&origin, &v1, &x));
    assert_eq!(1.0, side(&origin, &v1, &y));
}


impl Poly {
    pub fn contains(&self, v: &Point) -> bool {
        // Find the two edges of the convex polygon that
        // surround `v` in the y direction.
        let pn = self.p.len();
        let edges: Vec<_> = (0..pn)
            .filter_map(|i| {
                let mut pp = [&self.p[i], &self.p[(i + 1) % pn]];
                if pp[0][Y] > pp[1][Y] {
                    pp.swap(0, 1);
                }
                if pp[0][Y] < v[Y] && v[Y] < pp[1][Y] {
                    Some(pp)
                } else {
                    None
                }
            })
            .collect();
        if edges.len() != 2 {
            return false;
        }

        // What side of each edge is `v` on?
        let s0 = side(edges[0][0], edges[0][1], v);
        let s1 = side(edges[1][0], edges[1][1], v);

        // Iff `v` is on opposite sides of each edge,
        // `v` is contained in this polygon.
        s0 != s1
    }
}

impl Shape for Poly {
    fn intersect(&self, xform: &Xform, ray: &Ray) -> Option<Intersection> {
        // Get the ray in our coordinates.
        let mut ray = ray.clone();
        let toi = xform.inverse();
        ray.transform(&toi);
        let Ray { rd, ro } = ray;

        let b = rd[Z];
        if b.abs() < TINY {
            // The ray is parallel to the plane, so no hit.
            return None;
        }
        
        let t = ro[Z] / b;
        if t < TINY {
            // The ray is behind the plane, so no hit.
            return None;
        }

        let i = ro + rd * t;
        if self.contains(&i) {
            // Return the hit information.
            Some(Intersection {
                normal: self.cnormal.clone(),
                at: Point::new([i[X], i[Y]]),
                t,
            })
        } else {
            // The ray misses the polygon, so no hit.
            None
        }
    }

    fn complete(&mut self, xform: &Xform) {
        let mut o = Point::new([0.0, 0.0, 0.0]);
        let mut r = Point::new([0.0, 0.0, 1.0]);
        let toi = xform.inverse();
        r.transform(&toi);
        o.transform(&toi);
        r -= o;
        r.unitize();
        self.cnormal = r;
    }
}
