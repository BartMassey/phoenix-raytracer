use crate::Color;

pub use ppm_raw::*;

pub struct OutputInfo<T: Write> {
    xsize: usize,
    ysize: usize,
    output: T,
}

pub trait Output {
    fn putpixel(&mut self, x: usize, y: usize, c: Color);
    fn flushrow(&mut self);
}

pub fn gamma(v: f64) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5).floor().try_into().unwrap()
}
