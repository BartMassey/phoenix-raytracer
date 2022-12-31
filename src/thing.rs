//! Thing structure. Things are scene objects.

use crate::*;

/// A thing has a shape, a texture and a transform that defines
/// its position, size and orientation.
pub struct Thing {
    /// Shape of thing.
    pub shape: Box<dyn Shape>,
    pub texture: Box<dyn Texture>,
    pub xform: Xform,
}

impl Thing {
    /// XXX For now, hard-code the things in the scene. We
    /// will eventually fix this.
    pub fn generate() -> Vec<Thing> {
        let zb: f64 = D * A.cos();

        fn joggle() -> f64 {
            100.0 * TINY * (frandom() - 0.5)
        }

        let ll = Point::new([-4.0, -4.0]);
        let ul = Point::new([-4.0, 4.0]);
        let ur = Point::new([4.0, 4.0]);
        let lr = Point::new([4.0, -4.0]);

        let kyellow = Color::new(1.0, 0.9, 0.7);
        let kblue = Color::new(0.3, 0.3, 1.0);
        let kgreen = Color::new(0.3, 1.0, 0.3);
        let kwhite = Color::new(1.0, 1.0, 1.0);
        let kblack = Color::new(0.0, 0.0, 0.0);

        let tgreen = SolidTexture::new(kblack, kgreen * 0.2, kgreen * 0.75, 100.0);
        let tblue = SolidTexture::new(kblack, kblue * 0.2, kblue * 0.75, 100.0);
        let tyellow = SolidTexture::new(kblack, kyellow * 0.85, kwhite * 0.05, 10.0);
        let tboard = BoardTexture::new(
            Box::new(tyellow.clone()),
            Box::new(tblue.clone()),
        );

        let mut things = vec![];

        let p = Poly::new([ll, ul, ur, lr]);
        let mut xform = Xform::identity();
        xform *= &Xform::rotation_y(-PI / 2.0);
        xform *= &Xform::scaling(&Point::new([1.0, D / 8.0, D / 8.0]));
        let xboard = Point::new([-joggle(), joggle(), zb + joggle()]);
        xform *= &Xform::translation(&xboard);
        let board = Thing {
            shape: Box::new(p),
            texture: Box::new(tboard),
            xform,
        };
        things.push(board);

        let s = Sphere::default();
        let mut xform = Xform::identity();
        xform *= &Xform::scaling(&Point::new([1.5, 1.5, 1.5]));
        let x = Point::new([1.5 + joggle(), joggle(), zb + joggle()]);
        xform *= &Xform::translation(&x);
        let sgreen = Thing {
            shape: Box::new(s),
            texture: Box::new(tgreen),
            xform,
        };
        things.push(sgreen);

        let s = Sphere::default();
        let x = Point::new([1.0 + joggle(), -D / 3.0 + joggle(), zb - D / 5.0 + joggle()]);
        let xform = Xform::translation(&x);
        let sblue = Thing {
            shape: Box::new(s),
            texture: Box::new(tblue),
            xform,
        };
        things.push(sblue);

        let s = Sphere::default();
        let x = Point::new([1.0 + joggle(), D / 3.0 + joggle(), zb - D / 3.0 + joggle()]);
        let xform = Xform::translation(&x);
        let syellow = Thing {
            shape: Box::new(s),
            texture: Box::new(tyellow),
            xform,
        };
        things.push(syellow);

        things
    }
}
