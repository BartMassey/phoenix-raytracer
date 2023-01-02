use crate::*;

pub struct BoardTexture {
    red_texture: Box<dyn Texture>,
    black_texture: Box<dyn Texture>,
}

impl BoardTexture {
    pub fn new(r: Box<dyn Texture>, b: Box<dyn Texture>) -> Self {
        Self {
            red_texture: r,
            black_texture: b,
        }
    }
}

impl Texture for BoardTexture {
    fn value(
        &self,
        at: &Point,
        gc: &Point,
        normal: &Point,
        m: &Model,
        depth: usize,
    ) -> Color {
        let x: isize = at[X].floor() as isize;
        let y: isize = at[Y].floor() as isize;
        let texture = if (x & 1) ^ (y & 1) == 1 {
            &self.red_texture
        } else {
            &self.black_texture
        };
        texture.value(at, gc, normal, m, depth)
    }
}
