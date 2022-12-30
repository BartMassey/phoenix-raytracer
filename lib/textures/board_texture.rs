pub struct BoardTexture {
    red_texture: Texture,
    black_texture: Texture,
}

impl BoardTexture {
    pub fn new(red_texture: Texture, black_texture: Texture) -> Self {
        Self { red_texture, black_texture }
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
        let x: usize = at[X].floor().try_into().unwrap();
        let y: usize = at[Y].floor().try_into().unwrap();
        let texture = if (x & 1) ^ (y & 1) == 1 {
            self.red_texture
        } else {
            self.black_texture
        };
        texture.value(at, gc, normal, m, depth)
    }
}
