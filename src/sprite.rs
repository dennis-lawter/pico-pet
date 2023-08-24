use crate::render;

pub struct Sprite<'a> {
    pub x: i32,
    pub y: i32,
    w: usize,
    h: usize,
    data: &'a [u8],
}

impl<'a> Sprite<'a> {
    pub fn new(x: i32, y: i32, w: usize, h: usize, data: &'a [u8]) -> Self {
        Sprite { x, y, w, h, data }
    }

    pub fn draw(&self) {
        render::blit(self.x, self.y, self.w, self.h, self.data)
    }
}

pub struct SpriteFactory;

impl SpriteFactory {
    const FERRIS_DIMENSIONS: (usize, usize) = (32, 24);
    const URCHIN_DIMENSIONS: (usize, usize) = (36, 24);

    pub fn new_ferris_sprite() -> Sprite<'static> {
        Sprite::new(
            0,
            0,
            Self::FERRIS_DIMENSIONS.0,
            Self::FERRIS_DIMENSIONS.1,
            include_bytes!("../rgb332/ferris.png.data"),
        )
    }

    pub fn new_corro_sprite() -> Sprite<'static> {
        Sprite::new(
            0,
            0,
            Self::URCHIN_DIMENSIONS.0,
            Self::URCHIN_DIMENSIONS.1,
            include_bytes!("../rgb332/corro.png.data"),
        )
    }
}
