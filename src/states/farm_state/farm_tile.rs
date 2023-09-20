use crate::display::{render, sprite::Sprite};

pub const HARVESTABLE_SPRITES: [FarmTileSprites; 13] = [
    FarmTileSprites::Cuke3,    //
    FarmTileSprites::Corn4,    //
    FarmTileSprites::Onion2,   // veg
    FarmTileSprites::Onion5,   // seeds
    FarmTileSprites::Tater5,   // veg
    FarmTileSprites::Tater3,   // seeds
    FarmTileSprites::Carrot3,  // veg
    FarmTileSprites::Carrot6,  // seeds
    FarmTileSprites::Spinach4, // veg
    FarmTileSprites::Spinach6, // seeds
    FarmTileSprites::Mater4,   //
    FarmTileSprites::Pump5,    // veg
    FarmTileSprites::Pump6,    // seeds
];

pub const HARVESTABLE_COLOR: u8 = 0b000_000_11;

#[derive(Clone, Copy, PartialEq)]
pub enum FarmTileSprites {
    Tilled = 0,

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

    // no sprite
    Soil,
}
impl Into<usize> for FarmTileSprites {
    fn into(self) -> usize {
        self as usize
    }
}
impl FarmTileSprites {
    pub fn draw(&self, x: i32, y: i32, sprite_sheet: &mut Sprite) {
        render::fill_rect(x, y, 16, 16, 0b010_001_00);
        if self != &Self::Soil {
            sprite_sheet.x = x;
            sprite_sheet.y = y;
            sprite_sheet.draw((*self) as usize);

            if HARVESTABLE_SPRITES.contains(self) {
                render::solid_line_rect(x - 1, y - 1, 18, 18, HARVESTABLE_COLOR);
            }
        }
    }
}
