use crate::display::{render, sprite::Sprite};

#[derive(Clone, Copy, PartialEq)]
pub enum FarmTile {
    Soil = 0,

    Tilled,

    Sprout,

    Weed1,
    Weed2,
    Weed3,

    Cuke1,
    Cuke2,
    Cuke3,

    Corn1,
    Corn2,
    Corn3,
    Corn4,

    Onion1,
    Onion2,
    Onion3,
    Onion4,
    Onion5,

    Tater1,
    Tater2,
    Tater3,
    Tater4,
    Tater5,

    Carrot1,
    Carrot2,
    Carrot3,
    Carrot4,
    Carrot5,
    Carrot6,

    Spinach1,
    Spinach2,
    Spinach3,
    Spinach4,
    Spinach5,
    Spinach6,

    Mater1,
    Mater2,
    Mater3,
    Mater4,

    Pump1,
    Pump2,
    Pump3,
    Pump4,
    Pump5,
    Pump6,

    Planter,

    Mulch,

    BirdSeed,

    Scare1,
    Scare2,
}
impl Into<usize> for FarmTile {
    fn into(self) -> usize {
        self as usize
    }
}
impl FarmTile {
    pub fn draw(&self, x: i32, y: i32, sprite_sheet: &mut Sprite) {
        render::fill_rect(x, y, 16, 16, 0b010_001_00);
        if self != &Self::Soil {
            sprite_sheet.x = x;
            sprite_sheet.y = y;
            sprite_sheet.draw((*self) as usize - 1usize);
        }
    }
}
