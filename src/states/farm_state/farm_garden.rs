use crate::display::sprite::{Sprite, SpriteFactory};

use super::farm_tile::FarmTileSprites;

pub struct FarmGarden<'a> {
    tiles: [FarmTileSprites; 7 * 7],
    sprite_sheet: Sprite<'a>,
}
impl Default for FarmGarden<'static> {
    fn default() -> Self {
        let mock_garden = [
            FarmTileSprites::Tilled,
            FarmTileSprites::Sprout,
            FarmTileSprites::Weed1,
            FarmTileSprites::Weed2,
            FarmTileSprites::Weed3,
            FarmTileSprites::Cuke1,
            FarmTileSprites::Cuke2,
            FarmTileSprites::Cuke3,
            FarmTileSprites::Corn1,
            FarmTileSprites::Corn2,
            FarmTileSprites::Corn3,
            FarmTileSprites::Corn4,
            FarmTileSprites::Onion1,
            FarmTileSprites::Onion2,
            FarmTileSprites::Onion3,
            FarmTileSprites::Onion4,
            FarmTileSprites::Onion5,
            FarmTileSprites::Tater1,
            FarmTileSprites::Tater2,
            FarmTileSprites::Tater3,
            FarmTileSprites::Tater4,
            FarmTileSprites::Tater5,
            FarmTileSprites::Carrot1,
            FarmTileSprites::Carrot2,
            FarmTileSprites::Carrot3,
            FarmTileSprites::Carrot4,
            FarmTileSprites::Carrot5,
            FarmTileSprites::Carrot6,
            FarmTileSprites::Spinach1,
            FarmTileSprites::Spinach2,
            FarmTileSprites::Spinach3,
            FarmTileSprites::Spinach4,
            FarmTileSprites::Spinach5,
            FarmTileSprites::Spinach6,
            FarmTileSprites::Mater1,
            FarmTileSprites::Mater2,
            FarmTileSprites::Mater3,
            FarmTileSprites::Mater4,
            FarmTileSprites::Pump1,
            FarmTileSprites::Pump2,
            FarmTileSprites::Pump3,
            FarmTileSprites::Pump4,
            FarmTileSprites::Pump5,
            FarmTileSprites::Pump6,
            FarmTileSprites::Planter,
            FarmTileSprites::Mulch,
            FarmTileSprites::BirdSeed,
            FarmTileSprites::Scare1,
            FarmTileSprites::Scare2,
        ];
        Self {
            tiles: mock_garden,
            sprite_sheet: SpriteFactory::new_farm_sprite(0, 0),
        }
    }
}
impl FarmGarden<'static> {
    pub fn tick(&mut self) {
        // check to make sure we're in daylight hours (8:00 - 19:00)
        // check to see if we're running a tended tick (minute should be 29-31)
        // advance each tile
        // check for birds
        // check for weeds
    }
    pub fn draw(&mut self) {
        for y in 0..7 {
            for x in 0..7 {
                let index = y * 7 + x;
                let x_pixel = x as i32 * 17 + 5;
                let y_pixel = y as i32 * 17 + 5;
                self.tiles[index].draw(x_pixel, y_pixel, &mut self.sprite_sheet);
            }
        }
    }
}
