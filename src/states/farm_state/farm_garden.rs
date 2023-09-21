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
    pub fn act(&mut self, tile_id: usize, action: &GardenAction) -> Result<(), ()> {
        let tile = self.tiles[tile_id];
        match action {
            GardenAction::Till => self.tiles[tile_id] = FarmTileSprite::Tilled,

            GardenAction::Plant => todo!(),

            GardenAction::Harvest => {
                // TODO: gain item
                self.tiles[tile_id] = FarmTileSprite::Soil
            }

            GardenAction::BuildScarecrow => {
                if tile_id > 6 && self.tiles[tile_id - 7] == FarmTileSprite::Soil {
                    self.tiles[tile_id - 7] = FarmTileSprite::Scare1;
                    self.tiles[tile_id] = FarmTileSprite::Scare2;
                } else if tile_id < 7 * 6 && self.tiles[tile_id + 7] == FarmTileSprite::Soil {
                    self.tiles[tile_id] = FarmTileSprite::Scare1;
                    self.tiles[tile_id + 7] = FarmTileSprite::Scare2;
                } else {
                    return Err(());
                }
            }
            GardenAction::BuildPlanter => {
                self.tiles[tile_id] = FarmTileSprite::Planter;
            }
            GardenAction::PlaceMulch => {
                self.tiles[tile_id] = FarmTileSprite::Mulch;
            }
            GardenAction::PlaceBirdseed => {
                self.tiles[tile_id] = FarmTileSprite::BirdSeed;
            }
            GardenAction::Remove => {
                match tile {
                    FarmTileSprite::Scare1 => {
                        if tile_id <= 7 * 6 && self.tiles[tile_id + 7] == FarmTileSprite::Scare2 {
                            self.tiles[tile_id + 7] = FarmTileSprite::Soil
                        }
                    }
                    FarmTileSprite::Scare2 => {
                        if tile_id >= 7 && self.tiles[tile_id - 7] == FarmTileSprite::Scare1 {
                            self.tiles[tile_id - 7] = FarmTileSprite::Soil
                        }
                    }

                    FarmTileSprite::Tilled
                    | FarmTileSprite::Sprout
                    | FarmTileSprite::Weed1
                    | FarmTileSprite::Weed2
                    | FarmTileSprite::Weed3
                    | FarmTileSprite::Cuke1
                    | FarmTileSprite::Cuke2
                    | FarmTileSprite::Cuke3
                    | FarmTileSprite::Corn1
                    | FarmTileSprite::Corn2
                    | FarmTileSprite::Corn3
                    | FarmTileSprite::Corn4
                    | FarmTileSprite::Onion1
                    | FarmTileSprite::Onion2
                    | FarmTileSprite::Onion3
                    | FarmTileSprite::Onion4
                    | FarmTileSprite::Onion5
                    | FarmTileSprite::Tater1
                    | FarmTileSprite::Tater2
                    | FarmTileSprite::Tater3
                    | FarmTileSprite::Tater4
                    | FarmTileSprite::Tater5
                    | FarmTileSprite::Carrot1
                    | FarmTileSprite::Carrot2
                    | FarmTileSprite::Carrot3
                    | FarmTileSprite::Carrot4
                    | FarmTileSprite::Carrot5
                    | FarmTileSprite::Carrot6
                    | FarmTileSprite::Spinach1
                    | FarmTileSprite::Spinach2
                    | FarmTileSprite::Spinach3
                    | FarmTileSprite::Spinach4
                    | FarmTileSprite::Spinach5
                    | FarmTileSprite::Spinach6
                    | FarmTileSprite::Mater1
                    | FarmTileSprite::Mater2
                    | FarmTileSprite::Mater3
                    | FarmTileSprite::Mater4
                    | FarmTileSprite::Pump1
                    | FarmTileSprite::Pump2
                    | FarmTileSprite::Pump3
                    | FarmTileSprite::Pump4
                    | FarmTileSprite::Pump5
                    | FarmTileSprite::Pump6
                    | FarmTileSprite::Planter
                    | FarmTileSprite::Mulch
                    | FarmTileSprite::BirdSeed
                    | FarmTileSprite::Soil => {}
                }
                self.tiles[tile_id] = FarmTileSprite::Soil;
            }
            GardenAction::None => return Err(()),
        };

        Ok(())
    }
}
