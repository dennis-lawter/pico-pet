use crate::{
    display::{render, text_writer},
    hardware::input::KeyNames,
    states::{AppState, State},
};

pub struct StateNyi {
    next_state: Option<AppState>,
}

impl State for StateNyi {
    fn input(&mut self) {
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.next_state = Some(AppState::GamePlay);
        }
    }

    fn tick(&mut self) {
        ()
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        render::flood(0b000_000_00);
        text_writer::full_dialog_box("NOT IMPL", "todo!()");
    }

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}
impl Default for StateNyi {
    fn default() -> Self {
        Self { next_state: None }
    }
}
