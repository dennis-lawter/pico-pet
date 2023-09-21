use crate::{
    display::render,
    hardware::input::KeyNames,
    states::{AppState, State},
};

use super::garden_action_menu::GardenActionMenu;

pub struct FarmState {
    tile_selected: Option<usize>,
    next_state: Option<AppState>,
    frame_count: usize,
    selector_menu: Option<GardenActionMenu>,
}

impl State for FarmState {
    fn input(&mut self) {
        let input = crate::globals::get_input();

        match &mut self.selector_menu {
            Some(selector_menu) => {
                selector_menu.input();
            }
            None => {
                if self.tile_selected.is_none() {
                    if input.get_state(&KeyNames::Back).just_released {
                        self.next_state = Some(AppState::GamePlay);
                    } else if input.get_state(&KeyNames::Left).key_repeat_triggered {
                        self.tile_selected = Some(7 * 7 - 1);
                    } else if input.get_state(&KeyNames::Right).key_repeat_triggered {
                        self.tile_selected = Some(0);
                    }
                } else {
                    if input.get_state(&KeyNames::Back).just_released {
                        self.tile_selected = None;
                    } else if input.get_state(&KeyNames::Left).key_repeat_triggered {
                        if self.tile_selected.unwrap() == 0 {
                            self.tile_selected = None
                        } else {
                            self.tile_selected = Some(self.tile_selected.unwrap() - 1);
                        }
                    } else if input.get_state(&KeyNames::Right).key_repeat_triggered {
                        if self.tile_selected.unwrap() == (7 * 7 - 1) {
                            self.tile_selected = None
                        } else {
                            self.tile_selected = Some(self.tile_selected.unwrap() + 1);
                        }
                    } else if input.get_state(&KeyNames::Confirm).just_released {
                        let selected_x = (self.tile_selected.unwrap() % 7) * 17 + 5;
                        let selected_y = (self.tile_selected.unwrap() / 7) * 17 + 5;

                        let menu_x = if selected_x > 64 {
                            selected_x + 18 - GardenActionMenu::MENU_WIDTH
                        } else {
                            selected_x - 1
                        };
                        let menu_y = if selected_y > 64 {
                            selected_y - 2 - GardenActionMenu::MENU_HEIGHT
                        } else {
                            selected_y + 18
                        };

                        self.selector_menu = Some(GardenActionMenu::new(
                            menu_x as i32,
                            menu_y as i32,
                            self.tile_selected.unwrap(),
                        ));
                    }
                }
            }
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        if self.selector_menu.is_some() {
            if self.selector_menu.as_ref().unwrap().ready_to_exit {
                self.selector_menu = None;
            }
        }
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        render::flood(0b000_111_00);

        crate::globals::get_garden().draw();

        if self.tile_selected.is_some() {
            let index_x = self.tile_selected.unwrap() % 7;
            let index_y = self.tile_selected.unwrap() / 7;

            let x_pixel = index_x as i32 * 17 + 4;
            let y_pixel = index_y as i32 * 17 + 4;

            render::dithered_line_rect(
                x_pixel,
                y_pixel,
                18,
                18,
                0b111_111_11,
                (self.frame_count / 5) % 2 == 1,
            );
            render::dithered_line_rect(
                x_pixel - 1,
                y_pixel - 1,
                20,
                20,
                0b111_111_11,
                (self.frame_count / 5) % 2 == 1,
            );
        }

        if self.selector_menu.is_some() {
            self.selector_menu.as_mut().unwrap().draw();
        }
    }

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}

impl Default for FarmState {
    fn default() -> Self {
        Self {
            tile_selected: None,
            next_state: None,
            frame_count: 0,
            selector_menu: None,
        }
    }
}
