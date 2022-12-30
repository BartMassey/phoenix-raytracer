use crate::*;

/// Scene model for rendering. So far stored as naïve arrays
/// rather than octrees or something.
pub struct Model {
    pub eye: Point,
    pub the_light: Light,
    pub scene: Vec<Thing>,
    pub bg: Point,
    pub maxdepth: usize,
}

impl Model {
    fn new(eye: Point, the_light: Light, scene: Scene, bg: Point) -> Self {
        Self {
            eye,
            the_light,
            scene,
            bg,
            maxdepth: 10,
        }
    }
}
