use crate::*;

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
        at: &Point,
        gc: &Point,
        normal: &Point,
        m: &Model,
        depth: usize
    ) -> Color {
        // Start with ambient term.
        let mut result = self.ka;

        let pl = m.the_light.at();
        let pli = m.the_light.i();
        let pe = m.eye;
        // Unit vector toward the light.
        let lv = (pl - gc).unit();

        // Diffuse term.
        let fd = lv * normal;
        if fd > TINY {
            result += kd.vproduct(&pli) * fd;
        }

        // Specular Terms.
        // Unit vector from the eye toward the target.
        let pt = (gc - pe).unit();
        // Specular direction.
        let ps = pt - normal * ((pt * normal) * 2.0);

        // Specular diffusion term.
        let fs = ps * lv;
        if fs > 0.0 {
            fs = (ns * fs.log()).exp();
            if fs > TINY {
                result += ks.vproduct(&pli) * fs;
            }
        }

        // Ray tracing term.
        result += ks.vproduct(trace(ray(gc, ps), m, depth).contraction());

        result
    }
}
