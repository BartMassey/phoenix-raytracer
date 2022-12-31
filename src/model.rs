use crate::*;

/// Scene model for rendering. So far stored as naïve arrays
/// rather than octrees or something.
pub struct Model {
    pub eye: Point,
    pub the_light: Box<dyn Light>,
    pub scene: Vec<Thing>,
    pub bg: Color,
    pub max_depth: usize,
}

impl Model {
    pub fn new(eye: Point, the_light: Box<dyn Light>, scene: Vec<Thing>, bg: Color) -> Self {
        Self {
            eye,
            the_light,
            scene,
            bg,
            max_depth: 10,
        }
    }

    /// XXX For now, hard-code the parameters in the
    /// model. We will eventually fix this.
    pub fn generate(scene: Vec<Thing>) -> Self {
        let he: f64 = D * A.sin();

        let the_light = PointLight {
            loc: Point::new([2.0 * he, -he, 0.0]),
            intensity: Color::new(1.25, 1.25, 1.15),
        };
        let eye = Point::new([he, 0.0, 0.0]);
        let dkgray = Color::new(0.2, 0.2, 0.2);
        Model::new(eye, Box::new(the_light), scene, dkgray)
    }
}
