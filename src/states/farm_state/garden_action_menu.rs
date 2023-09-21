use crate::{
    display::{
        render,
        text_writer::{self, FontStyle},
    },
    hardware::input::KeyNames,
};

use super::{farm_tile::FarmTileSprite, garden_action::GardenAction};

const NUM_OF_ACTIONS: usize = 8;

pub struct GardenActionMenu {
    pub x: i32,
    pub y: i32,
    selection: usize,
    selection_list: [GardenAction; NUM_OF_ACTIONS],
    tile_selected: usize,
    pub ready_to_exit: bool,
    err_timer: usize,
}

impl GardenActionMenu {
    pub const MENU_WIDTH: usize = 60;
    pub const MENU_HEIGHT: usize = 50;
    fn get_selection_list_from_tile(sprite: &FarmTileSprite) -> [GardenAction; 8] {
        match sprite {
            FarmTileSprite::Tilled => [
                GardenAction::None,
                GardenAction::Plant,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::Remove,
            ],

            FarmTileSprite::Sprout => [
                GardenAction::Till,
                GardenAction::Plant,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::Remove,
            ],

            FarmTileSprite::Weed1
            | FarmTileSprite::Weed2
            | FarmTileSprite::Weed3
            | FarmTileSprite::Cuke1
            | FarmTileSprite::Cuke2
            | FarmTileSprite::Corn1
            | FarmTileSprite::Corn2
            | FarmTileSprite::Corn3
            | FarmTileSprite::Onion1
            | FarmTileSprite::Onion3
            | FarmTileSprite::Onion4
            | FarmTileSprite::Tater1
            | FarmTileSprite::Tater2
            | FarmTileSprite::Tater4
            | FarmTileSprite::Carrot1
            | FarmTileSprite::Carrot2
            | FarmTileSprite::Carrot4
            | FarmTileSprite::Carrot5
            | FarmTileSprite::Spinach1
            | FarmTileSprite::Spinach2
            | FarmTileSprite::Spinach3
            | FarmTileSprite::Spinach5
            | FarmTileSprite::Mater1
            | FarmTileSprite::Mater2
            | FarmTileSprite::Mater3
            | FarmTileSprite::Pump1
            | FarmTileSprite::Pump2
            | FarmTileSprite::Pump3
            | FarmTileSprite::Pump4 => [
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::Remove,
            ],

            FarmTileSprite::Cuke3
            | FarmTileSprite::Corn4
            | FarmTileSprite::Onion2
            | FarmTileSprite::Onion5
            | FarmTileSprite::Tater3
            | FarmTileSprite::Tater5
            | FarmTileSprite::Carrot3
            | FarmTileSprite::Carrot6
            | FarmTileSprite::Spinach4
            | FarmTileSprite::Spinach6
            | FarmTileSprite::Mater4
            | FarmTileSprite::Pump5
            | FarmTileSprite::Pump6 => [
                GardenAction::None,
                GardenAction::None,
                GardenAction::Harvest,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::Remove,
            ],

            FarmTileSprite::Planter
            | FarmTileSprite::Mulch
            | FarmTileSprite::BirdSeed
            | FarmTileSprite::Scare1
            | FarmTileSprite::Scare2 => [
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::None,
                GardenAction::Remove,
            ],

            FarmTileSprite::Soil => [
                GardenAction::Till,
                GardenAction::None,
                GardenAction::None,
                GardenAction::BuildScarecrow,
                GardenAction::BuildPlanter,
                GardenAction::PlaceMulch,
                GardenAction::PlaceBirdseed,
                GardenAction::None,
            ],
        }
    }
    pub fn new(x: i32, y: i32, tile_selected: usize) -> Self {
        let sprite = &crate::globals::get_garden().tiles[tile_selected];
        Self {
            selection: NUM_OF_ACTIONS,
            selection_list: Self::get_selection_list_from_tile(sprite),
            ready_to_exit: false,
            x,
            y,
            tile_selected,
            err_timer: 0,
        }
    }
    pub fn input(&mut self) {
        let input = crate::globals::get_input();

        if input.get_state(&KeyNames::Back).just_released {
            self.ready_to_exit = true;
            return;
        }

        if input.get_state(&KeyNames::Left).just_pressed {
            self.dec_selection();
        }
        if input.get_state(&KeyNames::Right).just_pressed {
            self.inc_selection();
        }
        if input.get_state(&KeyNames::Confirm).just_released && self.selection != NUM_OF_ACTIONS {
            // TODO: handle action
            let action = &self.selection_list[self.selection];
            let result = crate::globals::get_garden().act(self.tile_selected, action);
            if result.is_ok() {
                self.ready_to_exit = true;
            } else {
                self.err_timer = 20;
            }
        }
    }
    pub fn draw(&mut self) {
        render::fill_rect(
            self.x,
            self.y,
            Self::MENU_WIDTH,
            Self::MENU_HEIGHT,
            0b111_111_11,
        );
        render::fancy_border(self.x, self.y, Self::MENU_WIDTH, Self::MENU_HEIGHT);
        let mut y = self.y + 5;
        for i in 0..NUM_OF_ACTIONS {
            if self.selection_list[i] != GardenAction::None {
                let action_str = fixedstr::str_format!(
                    fixedstr::str12,
                    "{}",
                    GardenAction::from_usize(i).unwrap()
                );
                text_writer::draw_text(self.x + 10, y, FontStyle::Small, 0b000_000_00, &action_str);

                if i == self.selection {
                    if self.err_timer > 0 {
                        render::h_solid_line(
                            self.x + 5,
                            y + 4,
                            Self::MENU_WIDTH - 10,
                            0b111_000_00,
                        );
                        self.err_timer -= 1;
                    } else {
                        text_writer::draw_text(self.x + 5, y, FontStyle::Icon, 0b000_000_00, "}");
                    }
                }

                y += 8;
            }
        }
    }

    fn dec_selection(&mut self) {
        if self.selection == 0 {
            self.selection = NUM_OF_ACTIONS - 1;
        } else {
            self.selection -= 1;
        }
        if self.selection_list[self.selection] == GardenAction::None {
            self.dec_selection()
        }
    }

    fn inc_selection(&mut self) {
        if self.selection >= NUM_OF_ACTIONS - 1 {
            self.selection = 0;
        } else {
            self.selection += 1;
        }
        if self.selection_list[self.selection] == GardenAction::None {
            self.inc_selection()
        }
    }
}
