pub use std::io::Write;
pub use std::fs::File;

use crate::*;

pub struct PpmRawOutput<T: Write> {
    output: OutputInfo<T>,
    curx: usize,
    cury: usize,
}

impl PpmRawOutput<File> {
    pub fn new(
        filename: &str,
        xsize: usize,
        ysize: usize,
    ) -> Result<Self, std::io::Error> {
        let filename = filename.to_string() + ".ppm";

        let mut output = std::fs::File::create(filename)?;
        write!(output, "P6\n{}\n{}\n{}\n", xsize, ysize, 255)?;
        let output = OutputInfo { xsize, ysize, output };
        Ok(Self { output, curx: 0, cury: 0 })
    }
}

impl<T: Write> Output for PpmRawOutput<T> {
    fn put_pixel(&mut self, x: usize, y: usize, c: Color) {
        assert!(x < self.output.xsize);
        assert_eq!(self.curx, x);
        assert_eq!(self.cury, y);
        let rgb = c.apply(gamma);
        self.output.output.write_all(rgb.as_ref()).unwrap();
        self.curx += 1;
        if self.curx >= self.output.xsize {
            self.curx = 0;
            self.cury += 1;
        }
    }

    fn flush_row(&mut self) {}
}
