use crate::*;

/// Information about a point light source.
pub struct PointLight {
    /// Position of light source.
    pub loc: Point,
    /// Luminance of light source.
    pub intensity: Color,
}


impl Light for PointLight {
    fn at(&self) -> Point {
        self.loc.clone()
    }

    fn i(&self) -> Color {
        self.intensity
    }
}
