use crate::display::sprite::{Sprite, SpriteFactory};

use super::farm_tile::FarmTile;

pub struct FarmGarden<'a> {
    tiles: [FarmTile; 7 * 7],
    sprite_sheet: Sprite<'a>,
}
impl Default for FarmGarden<'static> {
    fn default() -> Self {
        let mock_garden = [
            FarmTile::Tilled,
            FarmTile::Sprout,
            FarmTile::Weed1,
            FarmTile::Weed2,
            FarmTile::Weed3,
            FarmTile::Cuke1,
            FarmTile::Cuke2,
            FarmTile::Cuke3,
            FarmTile::Corn1,
            FarmTile::Corn2,
            FarmTile::Corn3,
            FarmTile::Corn4,
            FarmTile::Onion1,
            FarmTile::Onion2,
            FarmTile::Onion3,
            FarmTile::Onion4,
            FarmTile::Onion5,
            FarmTile::Tater1,
            FarmTile::Tater2,
            FarmTile::Tater3,
            FarmTile::Tater4,
            FarmTile::Tater5,
            FarmTile::Carrot1,
            FarmTile::Carrot2,
            FarmTile::Carrot3,
            FarmTile::Carrot4,
            FarmTile::Carrot5,
            FarmTile::Carrot6,
            FarmTile::Spinach1,
            FarmTile::Spinach2,
            FarmTile::Spinach3,
            FarmTile::Spinach4,
            FarmTile::Spinach5,
            FarmTile::Spinach6,
            FarmTile::Mater1,
            FarmTile::Mater2,
            FarmTile::Mater3,
            FarmTile::Mater4,
            FarmTile::Pump1,
            FarmTile::Pump2,
            FarmTile::Pump3,
            FarmTile::Pump4,
            FarmTile::Pump5,
            FarmTile::Pump6,
            FarmTile::Planter,
            FarmTile::Mulch,
            FarmTile::BirdSeed,
            FarmTile::Scare1,
            FarmTile::Scare2,
        ];
        Self {
            tiles: mock_garden,
            sprite_sheet: SpriteFactory::new_farm_sprite(0, 0),
        }
    }
}
impl FarmGarden<'static> {
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
