use crate::*;

#[derive(Clone)]
pub struct SolidTexture {
    ka: Color,
    kd: Color,
    ks: Color,
    ns: f64,
}

impl SolidTexture {
    pub fn new(ka: Color, kd: Color, ks: Color, ns: f64) -> Self {
        Self { ka, kd, ks, ns }
    }
}

impl Texture for SolidTexture {
    fn value(
        &self,
        _at: &Point,
        gc: &Point,
        normal: &Point,
        m: &Model,
        depth: usize
    ) -> Color {
        // Start with ambient term.
        let mut result = self.ka;

        let pl = m.the_light.at();
        let pli = m.the_light.i();
        let pe = m.eye.clone();
        // Unit vector toward the light.
        let lv = (pl - gc.clone()).unit();

        // Diffuse term.
        let fd = lv.clone() * normal.clone();
        if fd > TINY {
            result += self.kd.colorize(&pli) * fd;
        }

        // Specular Terms.
        // Unit vector from the eye toward the target.
        let pt = (gc.clone() - pe).unit();
        // Specular direction.
        let ps = pt.clone() - normal.clone() * ((pt * normal.clone()) * 2.0);

        // Specular diffusion term.
        let fs = ps.clone() * lv;
        if fs > 0.0 {
            let fs = (self.ns * fs.ln()).exp();
            if fs > TINY {
                result += self.ks.colorize(&pli) * fs;
            }
        }

        // Ray tracing term.
        let ray = Ray::new(gc.clone(), ps);
        let tr = trace(&ray, m, depth);
        result += self.ks.colorize(&tr);

        result
    }
}
