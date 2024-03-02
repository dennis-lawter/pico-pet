use super::render;

pub struct Sprite<'a> {
    pub x: i32,
    pub y: i32,
    pub w: usize,
    pub h: usize,
    data: &'a [u8],
}

impl<'a> Sprite<'a> {
    pub fn new(x: i32, y: i32, w: usize, h: usize, data: &'a [u8]) -> Self {
        Sprite { x, y, w, h, data }
    }

    pub fn draw(&self, frame: usize) {
        let offset = frame * self.w * self.h;
        render::blit_from_offset(self.x, self.y, offset, self.w, self.h, self.data)
    }
}

pub struct SpriteFactory;

impl SpriteFactory {
    pub const FERRIS_DIMENSIONS: (usize, usize) = (40, 24);
    pub const MENU_DIMENSIONS: (usize, usize) = (24, 24);
    pub const POMO_MENU_DIMENSIONS: (usize, usize) = (24, 24);

    pub fn new_ferris_sprite(x: i32, y: i32) -> Sprite<'static> {
        Sprite::new(
            x,
            y,
            Self::FERRIS_DIMENSIONS.0,
            Self::FERRIS_DIMENSIONS.1,
            include_bytes!("../../sprite_raw/ferris.data"),
        )
    }

    pub fn new_menu_sprite(x: i32, y: i32) -> Sprite<'static> {
        Sprite::new(
            x,
            y,
            Self::MENU_DIMENSIONS.0,
            Self::MENU_DIMENSIONS.1,
            include_bytes!("../../sprite_raw/menu.data"),
        )
    }

    pub fn new_pomo_menu_sprite(x: i32, y: i32) -> Sprite<'static> {
        Sprite::new(
            x,
            y,
            Self::POMO_MENU_DIMENSIONS.0,
            Self::POMO_MENU_DIMENSIONS.1,
            include_bytes!("../../sprite_raw/pomo_menu.data"),
        )
    }
}
