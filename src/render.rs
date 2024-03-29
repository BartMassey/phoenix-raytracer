use rayon::prelude::*;

use crate::*;

pub fn trace(r: &Ray, m: &Model, depth: usize) -> Color {
    if depth > m.max_depth {
        return m.bg;
    }

    let first_intersection = m
        .scene
        .iter()
        .filter_map(|p| p.shape.intersect(&p.xform, r).map(|i| (i, p)))
        .min_by(|x1, x2| x1.0.t.partial_cmp(&x2.0.t).unwrap());

    match first_intersection {
        Some((i, p)) => {
            let nr = r.at(i.t);
            p.texture.value(&i.at, &nr, &i.normal, m, depth + 1)
        }
        None => m.bg,
    }
}

fn do_joggle(f: fn(f64) -> f64, i: usize, n: usize, t: f64) -> f64 {
    let mut a = i as f64 / n as f64 + t;
    while a > 0.5 {
        a -= 1.0;
    }
    0.5 * f(PI * a)
}

pub fn render<T>(
    mut out: T,
    m: &Model,
    w: usize,
    h: usize,
    sequential: bool,
    antialias: Option<usize>,
)
where
    T: Output,
{
    // Pick a uniform scale factor based on eyepoint distance
    // and aspect ratio.
    let hs = D * A.tan();
    let scale: f64 = hs / w.max(h) as f64;
    let view_xform = Xform::rotation_y(-A);

    let trace_one = |j, i| {
        let mut rt = Point::new([
            scale * (2.0 * j as f64 - w as f64),
            scale * (2.0 * i as f64 - h as f64),
            D,
        ]);
        match antialias {
            None => {
                rt.transform(&view_xform);
                let r = Ray::new(m.eye.clone(), rt);
                trace(&r, m, 0)
            }
            Some(aa) => {
                let mut ave = Color::new(0.0, 0.0, 0.0);
                for k in 0..aa {
                    let mut rt = Point::new([
                        2.0 * hs * (j as f64 + do_joggle(f64::cos, k, aa, frandom())) / h as f64 - hs,
                        2.0 * hs * (i as f64 + do_joggle(f64::sin, k, aa, frandom())) / w as f64 - hs,
                        D,
                    ]);
                    rt.transform(&view_xform);
                    let r = Ray::new(m.eye.clone(), rt);
                    ave += trace(&r, m, 0);
                }
                ave *= 1.0 / aa as f64;
                ave
            }
        }
    };

    if sequential {
        for j in (0..h).rev() {
            for i in 0..w {
                let ave = trace_one(j, i);
                out.put_pixel(i, h - j - 1, ave);
            }
            out.flush_row();
        }
    } else {
        let pixels: Vec<Vec<Color>> = (0..h)
            .into_par_iter()
            .rev()
            .map(|j| (0..w).into_par_iter().map(|i| trace_one(j, i)).collect())
            .collect();

        for (j, row) in pixels.into_iter().enumerate() {
            for (i, ave) in row.into_iter().enumerate() {
                out.put_pixel(i, j, ave);
            }
            out.flush_row();
        }
    }
}
