use crate::{
    display::render,
    hardware::input::KeyNames,
    states::{AppState, State},
};

use super::farm_garden::FarmGarden;

pub struct FarmState<'a> {
    farm: FarmGarden<'a>,
    tile_selected: Option<usize>,
    next_state: Option<AppState>,
    frame_count: usize,
}

impl State for FarmState<'static> {
    fn input(&mut self) {
        let input = crate::globals::get_input();

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
            }
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        render::flood(0b000_111_00);

        self.farm.draw();

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
        }
    }

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}

impl Default for FarmState<'static> {
    fn default() -> Self {
        Self {
            tile_selected: None,
            next_state: None,
            farm: FarmGarden::default(),
            frame_count: 0,
        }
    }
}
