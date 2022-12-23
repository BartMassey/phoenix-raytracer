// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

use infra::TINY;
use point::{X,Y,Z};
use shapes::shape::*;

/// A sphere of unit radius at a given position.
pub struct Rect {
    /// Transform into planar coordinates on sphere.
    to: XForm,
    /// Position of sphere.
    tr: Point,
}

impl Shape for Sphere {
    /// Iff the incoming ray is pointing in the right direction
    /// and hits the sphere, return a homogeneous
    /// point representing its xy coordinate.
    fn intersect(&self, ray: &Ray) -> Option<Point> {
        ray.transform(self.to.mi);  // now we have the ray in object coords...

        float a = r.d().mag2();
        float b = r.o() * r.d();
        float c = r.o().mag2() - 1.0;
        float d = b*b - a*c;
        float t;

        if( d < TINY ) {
          // ray misses sphere
          return 0;
        }
        if( a < 0 )
          t = (-b + sqrt( d )) / a;
        else
          t = (-b - sqrt( d )) / a;
        if( t < TINY ) {
          // ray is travelling away from sphere, so no hit
          return 0;
        }
        point i( r.o() + r.d() * t );  // the intersection point in object coords
        i.transform(toi);

        // there are many possible mappings -- here's a lame one
        s.t = t;
        s.at = new point( X(i), Y(i) );
        s.normal = new point( (i - tr).unit() );

        return 1;
    }
}
