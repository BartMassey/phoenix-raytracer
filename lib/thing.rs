// Copyright © 1991 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Thing structure. Things are scene objects.

use shapes::Shape;
use textures::Texture;
use xform::*;

/// A thing has a shape, a texture and a transform that defines
/// its position, size and orientation.
pub struct Thing {
    /// Shape of thing.
    pub shape: Shape,
    pub texture: Texture,
    pub xform: XForm,
}

impl Thing {
    /// XXX For now, hard-code the things in the model. We
    /// will eventually fix this.
    pub fn generate() -> Vec<Thing> {
        const A: f64 = 25.0 * f64::PI / 180.0;
        const D: f64 = 10.0;
        const ZB: f64 = D * A.cos();

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
        let tboard = BoardTexture::new(tyellow.clone(), tblue.clone());

        let mut things = vec![];

        let p = Poly::new([ll, ul, ur, lr]);
        let mut xform = Xform::identity();
        xform.transform(&Xform::rotation_y(-f64::PI / 2.0));
        xform.transform(&Xform::scaling(&Point::new([1.0, D / 8.0, D / 8.0])));
        let xboard = Point::new(-joggle(), joggle(), ZB + joggle());
        xform.transform(&Xform::translation(&xboard));
        let board = Thing { shape: p, texture: tboard, xform };
        things.push(board);

        let s = Sphere::default();
        let mut xform = Xform::identity();
        xform.transform(&Xform::scaling(&Point::new([1.5, 1.5, 1.5])));
        let x = Point::new([1.5 + joggle(), joggle(), ZB + joggle()]);
        xform.transform(&Xform::translation(&x));
        let sgreen = Thing { shape: s, texture: tgreen, xform };
        things.push(sgreen);

        let s = Sphere::default();
        let mut xform = Xform::identity();
        let x = Point::new([1.0 + joggle(), -D / 3.0 + joggle(), ZB - D / 5 + joggle()]);
        xform.transform(&Xform::translation(&x));
        let sblue = Thing { shape: s, texture: tblue, xform };
        things.push(sblue);

        let s = Sphere::default();
        let mut xform = Xform::identity();
        let x = Point::new([1.0 + joggle(), D / 3.0 + joggle(), ZB - D / 3.0 + joggle()]);
        xform.transform(&Xform::translation(&x));
        let syellow = Thing { shape: s, texture: tyellow, xform };
        things.push(syellow);
    }
}
