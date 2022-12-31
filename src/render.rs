use crate::*;

pub fn trace(r: &Ray, m: &Model, depth: usize) -> Color {
    if depth > m.max_depth {
        return m.bg;
    }

    let first_intersection = m.scene
        .iter()
        .filter_map(|p| {
            p.shape.intersect(&p.xform, r).map(|i| (i, p))
        })
        .min_by(|x1, x2| x1.0.t.partial_cmp(&x2.0.t).unwrap());

    match first_intersection {
        Some((i, p)) => {
            let nr = r.at(i.t);
            p.texture.value(&i.at, &nr, &i.normal, m, depth + 1)
        }
        None => {
            m.bg
        }
    }
}

/*
#[cfg(feature = "antialias")]
fn do_joggle(f: fn(f64) -> f64, i: usize, n: usize, t: f64) {
    let mut a = i as f64 / n as f64 + t;
    while a > 0.5 {
        a -= 1.0;
    }
    0.5 * f(PI * a)
}
*/

pub fn render(m: &Model, w: usize, h: usize) {
    let hs: f64 = D * A.tan();
    let mut out = PpmRawOutput::new("render.out", w, h).unwrap();

    for j in (0..h).rev() {
        for i in 0..w {
            let rt = Point::new([
                2.0 * hs * j as f64 / h as f64 - hs,
                2.0 * hs * i as f64 / w as f64 - hs,
                D,
            ]);
            let r = Ray::new(m.eye.clone(), rt);
            let ave = trace(&r, m, 0);
            out.put_pixel(i, h - j - 1, ave);
        }
        out.flush_row();
    }
}
