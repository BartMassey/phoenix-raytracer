use crate::Color;
use crate::output::Output;

pub struct PpmRawOutput<T: Write> {
    output: OutputInfo<T>,
    curx: usize,
    cury: usize,
}

impl <T: Write> PpmRawOutput<T> {
    pub fn new(
        filename: &str,
        xsize: usize,
        ysize: usize,
    ) -> Result<Self, std::io::Error> {
        let filename = filename.to_string() + ".ppm";
        let output = std::fs::File::create(filename)?;
        let output = OutputInfo { xsize, ysize, output };
        Ok(Self { output, curx: 0, cury: 0 })
    }
}

impl <T: Write> Output<T> for PpmRawOutput<T> {
    fn putpixel(&mut self, x: usize, y: usize, c: Color) {
        assert!(x < xsize);
        assert_eq!(curx, x);
        assert_eq!(cury, y);
        let rgb = c.map(gamma);
        self.output.output.write(rgb.as_ref()).unwrap();
    }

    fn flushrow(&mut self) {}
}
