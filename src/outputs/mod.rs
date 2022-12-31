pub mod ppm_raw;
pub use ppm_raw::*;

use crate::*;

pub struct OutputInfo<T: Write> {
    pub xsize: usize,
    pub ysize: usize,
    pub output: T,
}

pub trait Output {
    fn put_pixel(&mut self, x: usize, y: usize, c: Color);
    fn flush_row(&mut self);
}

pub fn gamma(v: f64) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5).floor() as u8
}
