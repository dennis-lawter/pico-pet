use super::farm_tile::FarmTile;

pub struct FarmGarden {
    tiles: [FarmTile; 7 * 7],
}
impl Default for FarmGarden {
    fn default() -> Self {
        Self {
            tiles: [FarmTile::Soil; 7 * 7],
        }
    }
}
impl FarmGarden {
    pub fn draw(&self) {
        for y in 0..7 {
            for x in 0..7 {
                let index = y * 7 + x;
                let x_pixel = x as i32 * 17 + 5;
                let y_pixel = y as i32 * 17 + 5;
                self.tiles[index].draw(x_pixel, y_pixel);
            }
        }
    }
}
