use crate::display::{render, text_writer};

use super::{AppState, State};

pub struct State3 {
    key0_down: bool,
    key1_down: bool,
    key2_down: bool,
    key3_down: bool,
    next_state: Option<AppState>,
}

impl State for State3 {
    fn new() -> Self {
        Self {
            key0_down: false,
            key1_down: false,
            key2_down: false,
            key3_down: false,
            next_state: None,
        }
    }

    fn input(&mut self) {
        let hardware = crate::globals::get_hardware();
        if !hardware.key0_pressed() && self.key0_down {
            self.next_state = Some(AppState::GamePlay);
        }
        self.key0_down = hardware.key0_pressed();
        self.key1_down = hardware.key1_pressed();
        self.key2_down = hardware.key2_pressed();
        self.key3_down = hardware.key3_pressed();
    }

    fn tick(&mut self) {
        //
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        render::flood(0b000_000_00);
        text_writer::full_dialog_box("NOT IMPL", "todo!()");
    }

    fn next_state(&self) -> &Option<super::AppState> {
        &self.next_state
    }
}
