use crate::{
    display::render,
    hardware::input::KeyNames,
    states::{AppState, State},
};

#[derive(Clone, Copy)]
enum FarmTile {
    Soil,
}
impl FarmTile {
    pub fn draw(&self, x: i32, y: i32) {
        render::fill_rect(x, y, 16, 16, 0b010_001_00);
    }
}

pub struct FarmState {
    farm: [FarmTile; 7 * 7],
    tile_selected: Option<usize>,
    next_state: Option<AppState>,
}

impl State for FarmState {
    fn input(&mut self) {
        let input = crate::globals::get_input();

        if self.tile_selected.is_none() {
            if input.get_state(&KeyNames::Back).just_released {
                self.next_state = Some(AppState::GamePlay);
            } else if input.get_state(&KeyNames::Left).just_pressed {
                self.tile_selected = Some(7 * 7 - 1);
            } else if input.get_state(&KeyNames::Right).just_pressed {
                self.tile_selected = Some(0);
            }
        } else {
            if input.get_state(&KeyNames::Back).just_released {
                self.tile_selected = None;
            } else if input.get_state(&KeyNames::Left).just_pressed {
                if self.tile_selected.unwrap() == 0 {
                    self.tile_selected = None
                } else {
                    self.tile_selected = Some(self.tile_selected.unwrap() - 1);
                }
            } else if input.get_state(&KeyNames::Right).just_pressed {
                if self.tile_selected.unwrap() == (7 * 7 - 1) {
                    self.tile_selected = None
                } else {
                    self.tile_selected = Some(self.tile_selected.unwrap() + 1);
                }
            }
        }
    }

    fn tick(&mut self) {
        //
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        render::flood(0b000_111_00);

        for y in 0..7 {
            for x in 0..7 {
                let index = y * 7 + x;
                let x_pixel = x as i32 * 17 + 5;
                let y_pixel = y as i32 * 17 + 5;
                self.farm[index].draw(x_pixel, y_pixel);
                if self.tile_selected.is_some() && index == self.tile_selected.unwrap() {
                    render::solid_line_rect(x_pixel - 1, y_pixel - 1, 18, 18, 0b000_000_11);
                }
            }
        }
    }

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}
impl FarmState {
    pub fn new() -> Self {
        Self {
            farm: [FarmTile::Soil; 7 * 7],
            tile_selected: None,
            next_state: None,
        }
    }
}
