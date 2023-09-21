use core::convert::TryFrom;

use crate::display::sprite::{Sprite, SpriteFactory};

use super::{farm_tile::FarmTileSprite, garden_action::GardenAction};

pub struct FarmGarden<'a> {
    pub tiles: [FarmTileSprite; 7 * 7],
    sprite_sheet: Sprite<'a>,
}
impl Default for FarmGarden<'static> {
    fn default() -> Self {
        // let mock_garden = [
        //     FarmTileSprite::Tilled,
        //     FarmTileSprite::Sprout,
        //     FarmTileSprite::Weed1,
        //     FarmTileSprite::Weed2,
        //     FarmTileSprite::Weed3,
        //     FarmTileSprite::Cuke1,
        //     FarmTileSprite::Cuke2,
        //     FarmTileSprite::Cuke3,
        //     FarmTileSprite::Corn1,
        //     FarmTileSprite::Corn2,
        //     FarmTileSprite::Corn3,
        //     FarmTileSprite::Corn4,
        //     FarmTileSprite::Onion1,
        //     FarmTileSprite::Onion2,
        //     FarmTileSprite::Onion3,
        //     FarmTileSprite::Onion4,
        //     FarmTileSprite::Onion5,
        //     FarmTileSprite::Tater1,
        //     FarmTileSprite::Tater2,
        //     FarmTileSprite::Tater3,
        //     FarmTileSprite::Tater4,
        //     FarmTileSprite::Tater5,
        //     FarmTileSprite::Carrot1,
        //     FarmTileSprite::Carrot2,
        //     FarmTileSprite::Carrot3,
        //     FarmTileSprite::Carrot4,
        //     FarmTileSprite::Carrot5,
        //     FarmTileSprite::Carrot6,
        //     FarmTileSprite::Spinach1,
        //     FarmTileSprite::Spinach2,
        //     FarmTileSprite::Spinach3,
        //     FarmTileSprite::Spinach4,
        //     FarmTileSprite::Spinach5,
        //     FarmTileSprite::Spinach6,
        //     FarmTileSprite::Mater1,
        //     FarmTileSprite::Mater2,
        //     FarmTileSprite::Mater3,
        //     FarmTileSprite::Mater4,
        //     FarmTileSprite::Pump1,
        //     FarmTileSprite::Pump2,
        //     FarmTileSprite::Pump3,
        //     FarmTileSprite::Pump4,
        //     FarmTileSprite::Pump5,
        //     FarmTileSprite::Pump6,
        //     FarmTileSprite::Planter,
        //     FarmTileSprite::Mulch,
        //     FarmTileSprite::BirdSeed,
        //     FarmTileSprite::Scare1,
        //     FarmTileSprite::Soil,
        // ];
        let mut mock_garden = [FarmTileSprite::Soil; 7 * 7];
        let rng = crate::globals::get_rng();
        for i in 0..7 * 7 {
            mock_garden[i] = FarmTileSprite::try_from(rng.next() % 50).unwrap();
        }

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
    pub fn act(&mut self, tile_id: usize, action: &GardenAction) {
        let tile = self.tiles[tile_id];
        match tile {
            super::farm_tile::FarmTileSprite::Tilled => todo!(),
            super::farm_tile::FarmTileSprite::Sprout => todo!(),
            super::farm_tile::FarmTileSprite::Weed1 => todo!(),
            super::farm_tile::FarmTileSprite::Weed2 => todo!(),
            super::farm_tile::FarmTileSprite::Weed3 => todo!(),
            super::farm_tile::FarmTileSprite::Cuke1 => todo!(),
            super::farm_tile::FarmTileSprite::Cuke2 => todo!(),
            super::farm_tile::FarmTileSprite::Cuke3 => todo!(),
            super::farm_tile::FarmTileSprite::Corn1 => todo!(),
            super::farm_tile::FarmTileSprite::Corn2 => todo!(),
            super::farm_tile::FarmTileSprite::Corn3 => todo!(),
            super::farm_tile::FarmTileSprite::Corn4 => todo!(),
            super::farm_tile::FarmTileSprite::Onion1 => todo!(),
            super::farm_tile::FarmTileSprite::Onion2 => todo!(),
            super::farm_tile::FarmTileSprite::Onion3 => todo!(),
            super::farm_tile::FarmTileSprite::Onion4 => todo!(),
            super::farm_tile::FarmTileSprite::Onion5 => todo!(),
            super::farm_tile::FarmTileSprite::Tater1 => todo!(),
            super::farm_tile::FarmTileSprite::Tater2 => todo!(),
            super::farm_tile::FarmTileSprite::Tater3 => todo!(),
            super::farm_tile::FarmTileSprite::Tater4 => todo!(),
            super::farm_tile::FarmTileSprite::Tater5 => todo!(),
            super::farm_tile::FarmTileSprite::Carrot1 => todo!(),
            super::farm_tile::FarmTileSprite::Carrot2 => todo!(),
            super::farm_tile::FarmTileSprite::Carrot3 => todo!(),
            super::farm_tile::FarmTileSprite::Carrot4 => todo!(),
            super::farm_tile::FarmTileSprite::Carrot5 => todo!(),
            super::farm_tile::FarmTileSprite::Carrot6 => todo!(),
            super::farm_tile::FarmTileSprite::Spinach1 => todo!(),
            super::farm_tile::FarmTileSprite::Spinach2 => todo!(),
            super::farm_tile::FarmTileSprite::Spinach3 => todo!(),
            super::farm_tile::FarmTileSprite::Spinach4 => todo!(),
            super::farm_tile::FarmTileSprite::Spinach5 => todo!(),
            super::farm_tile::FarmTileSprite::Spinach6 => todo!(),
            super::farm_tile::FarmTileSprite::Mater1 => todo!(),
            super::farm_tile::FarmTileSprite::Mater2 => todo!(),
            super::farm_tile::FarmTileSprite::Mater3 => todo!(),
            super::farm_tile::FarmTileSprite::Mater4 => todo!(),
            super::farm_tile::FarmTileSprite::Pump1 => todo!(),
            super::farm_tile::FarmTileSprite::Pump2 => todo!(),
            super::farm_tile::FarmTileSprite::Pump3 => todo!(),
            super::farm_tile::FarmTileSprite::Pump4 => todo!(),
            super::farm_tile::FarmTileSprite::Pump5 => todo!(),
            super::farm_tile::FarmTileSprite::Pump6 => todo!(),
            super::farm_tile::FarmTileSprite::Planter => todo!(),
            super::farm_tile::FarmTileSprite::Mulch => todo!(),
            super::farm_tile::FarmTileSprite::BirdSeed => todo!(),
            super::farm_tile::FarmTileSprite::Scare1 => todo!(),
            super::farm_tile::FarmTileSprite::Scare2 => todo!(),

            super::farm_tile::FarmTileSprite::Soil => match action {
                GardenAction::Till => self.tiles[tile_id] = FarmTileSprite::Tilled,
                GardenAction::BuildScarecrow => todo!(),
                GardenAction::BuildPlanter => todo!(),
                GardenAction::PlaceMulch => todo!(),
                GardenAction::PlaceBirdseed => todo!(),
                _ => todo!(),
            },
        }
    }
}
