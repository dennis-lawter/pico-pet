use core::convert::TryFrom;

use crate::display::{render, sprite::Sprite};

pub const HARVESTABLE_SPRITES: [FarmTileSprite; 13] = [
    FarmTileSprite::Cuke3,    //
    FarmTileSprite::Corn4,    //
    FarmTileSprite::Onion2,   // veg
    FarmTileSprite::Onion5,   // seeds
    FarmTileSprite::Tater5,   // veg
    FarmTileSprite::Tater3,   // seeds
    FarmTileSprite::Carrot3,  // veg
    FarmTileSprite::Carrot6,  // seeds
    FarmTileSprite::Spinach4, // veg
    FarmTileSprite::Spinach6, // seeds
    FarmTileSprite::Mater4,   //
    FarmTileSprite::Pump5,    // veg
    FarmTileSprite::Pump6,    // seeds
];

pub const HARVESTABLE_COLOR: u8 = 0b000_000_11;

#[derive(Clone, Copy, PartialEq)]
pub enum FarmTileSprite {
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
impl Into<usize> for FarmTileSprite {
    fn into(self) -> usize {
        self as usize
    }
}
impl TryFrom<u8> for FarmTileSprite {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= (FarmTileSprite::Soil as u8) {
            Ok(unsafe { core::mem::transmute(value) })
        } else {
            Err(())
        }
    }
}
impl FarmTileSprite {
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
