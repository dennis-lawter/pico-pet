use core::fmt::Display;

use crate::{
    display::{
        render,
        text_writer::{self, FontStyle},
    },
    hardware::input::KeyNames,
};

use super::farm_tile::FarmTileSprite;

pub struct FarmActionMenu {
    pub x: i32,
    pub y: i32,
    selection: u8,
    selections_enabled: u16, // bitmap
    pub ready_to_exit: bool,
}

impl FarmActionMenu {
    pub const MENU_WIDTH: usize = 60;
    pub const MENU_HEIGHT: usize = 50;
    fn get_selections_enabled_from_farm_tile_sprite(sprite: &FarmTileSprite) -> u16 {
        match sprite {
            FarmTileSprite::Tilled => {
                // plant
                1 << (Selection::Plant as usize)
            }

            FarmTileSprite::Sprout => {
                // till prune
                1 << (Selection::Till as usize) | 1 << (Selection::Prune as usize)
            }

            FarmTileSprite::Weed1 | FarmTileSprite::Weed2 | FarmTileSprite::Weed3 => {
                // prune
                1 << (Selection::Prune as usize)
            }

            FarmTileSprite::Cuke1 | FarmTileSprite::Cuke2 => {
                // prune
                1 << (Selection::Prune as usize)
            }
            FarmTileSprite::Cuke3 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }

            FarmTileSprite::Corn1 | FarmTileSprite::Corn2 | FarmTileSprite::Corn3 => {
                // prune
                1 << (Selection::Prune as usize)
            }
            FarmTileSprite::Corn4 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }

            FarmTileSprite::Onion1 | FarmTileSprite::Onion3 | FarmTileSprite::Onion4 => {
                // prune
                1 << (Selection::Prune as usize)
            }
            FarmTileSprite::Onion2 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }
            FarmTileSprite::Onion5 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }

            FarmTileSprite::Tater1 | FarmTileSprite::Tater2 | FarmTileSprite::Tater4 => {
                // prune
                1 << (Selection::Prune as usize)
            }
            FarmTileSprite::Tater3 | FarmTileSprite::Tater5 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }

            FarmTileSprite::Carrot1
            | FarmTileSprite::Carrot2
            | FarmTileSprite::Carrot4
            | FarmTileSprite::Carrot5 => {
                // prune
                1 << (Selection::Prune as usize)
            }
            FarmTileSprite::Carrot3 | FarmTileSprite::Carrot6 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }

            FarmTileSprite::Spinach1
            | FarmTileSprite::Spinach2
            | FarmTileSprite::Spinach3
            | FarmTileSprite::Spinach5 => {
                // prune
                1 << (Selection::Prune as usize)
            }
            FarmTileSprite::Spinach4 | FarmTileSprite::Spinach6 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }

            FarmTileSprite::Mater1 | FarmTileSprite::Mater2 | FarmTileSprite::Mater3 => {
                // prune
                1 << (Selection::Prune as usize)
            }
            FarmTileSprite::Mater4 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }

            FarmTileSprite::Pump1
            | FarmTileSprite::Pump2
            | FarmTileSprite::Pump3
            | FarmTileSprite::Pump4 => {
                // prune
                1 << (Selection::Prune as usize)
            }
            FarmTileSprite::Pump5 | FarmTileSprite::Pump6 => {
                // prune harvest
                1 << (Selection::Prune as usize) | 1 << (Selection::Harvest as usize)
            }

            FarmTileSprite::Planter
            | FarmTileSprite::Mulch
            | FarmTileSprite::BirdSeed
            | FarmTileSprite::Scare1
            | FarmTileSprite::Scare2 => {
                // destroy
                1 << (Selection::Destroy as usize)
            }

            FarmTileSprite::Soil => {
                // till scarecrow planter mulch birdseed
                1 << (Selection::Till as usize)
                    | 1 << (Selection::BuildScarecrow as usize)
                    | 1 << (Selection::BuildPlanter as usize)
                    | 1 << (Selection::PlaceMulch as usize)
                    | 1 << (Selection::PlaceBirdseed as usize)
            }
        }
    }
    pub fn new(x: i32, y: i32, sprite: &FarmTileSprite) -> Self {
        Self {
            selection: 0,
            selections_enabled: Self::get_selections_enabled_from_farm_tile_sprite(sprite),
            ready_to_exit: false,
            x,
            y,
        }
    }
    pub fn input(&mut self) {
        let input = crate::globals::get_input();

        if input.get_state(&KeyNames::Back).just_released {
            self.ready_to_exit = true;
            return;
        }

        let mut number_of_bits_in_selections_enabled = 0;
        for i in 0..16
        /* based on u16 */
        {
            if 1 << i & self.selections_enabled != 0 {
                number_of_bits_in_selections_enabled += 1;
            }
        }

        if input.get_state(&KeyNames::Left).just_pressed {
            if self.selection == 0 {
                self.selection = number_of_bits_in_selections_enabled - 1;
            } else {
                self.selection -= 1;
            }
        } else if input.get_state(&KeyNames::Right).just_pressed {
            if self.selection >= number_of_bits_in_selections_enabled - 1 {
                self.selection = 0;
            } else {
                self.selection += 1;
            }
        }
    }
    pub fn draw(&self) {
        render::fill_rect(
            self.x,
            self.y,
            Self::MENU_WIDTH,
            Self::MENU_HEIGHT,
            0b111_111_11,
        );
        render::fancy_border(self.x, self.y, Self::MENU_WIDTH, Self::MENU_HEIGHT);
        let mut y = self.y + 5;
        let mut sel_num = 0;
        for i in 0..Selection::NumberOfSelections as usize {
            if (1 << i) & self.selections_enabled != 0 {
                let sel_str =
                    fixedstr::str_format!(fixedstr::str12, "{}", Selection::from_usize(i).unwrap());
                text_writer::draw_text(self.x + 10, y, FontStyle::Small, 0b000_000_00, &sel_str);

                if sel_num == self.selection {
                    text_writer::draw_text(self.x + 5, y, FontStyle::Icon, 0b000_000_00, "}")
                }

                y += 8;
                sel_num += 1;
            }
        }
        // draw selection cursor
    }
}

// TODO: move

enum Selection {
    Till,
    Plant,
    Prune,
    Harvest,
    BuildScarecrow,
    BuildPlanter,
    PlaceMulch,
    PlaceBirdseed,
    Destroy,

    NumberOfSelections,
}
impl Display for Selection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Selection::Till => write!(f, "TILL"),
            Selection::Plant => write!(f, "PLANT"),
            Selection::Prune => write!(f, "PRUNE"),
            Selection::Harvest => write!(f, "HARVEST"),
            Selection::BuildScarecrow => write!(f, "SCARECROW"),
            Selection::BuildPlanter => write!(f, "PLANTER"),
            Selection::PlaceMulch => write!(f, "MULCH"),
            Selection::PlaceBirdseed => write!(f, "BIRDSEED"),
            Selection::Destroy => write!(f, "DESTROY"),
            Selection::NumberOfSelections => Err(core::fmt::Error),
        }
    }
}
impl Selection {
    fn from_usize(i: usize) -> Result<Self, ()> {
        match i {
            0 => Ok(Selection::Till),
            1 => Ok(Selection::Plant),
            2 => Ok(Selection::Prune),
            3 => Ok(Selection::Harvest),
            4 => Ok(Selection::BuildScarecrow),
            5 => Ok(Selection::BuildPlanter),
            6 => Ok(Selection::PlaceMulch),
            7 => Ok(Selection::PlaceBirdseed),
            8 => Ok(Selection::Destroy),

            _ => Err(()),
        }
    }
}
