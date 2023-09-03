use crate::display::{render, text_writer};

use super::{AppState, State};

pub struct State5 {
    key0_down: bool,
    key1_down: bool,
    key2_down: bool,
    key3_down: bool,
    next_state: Option<AppState>,
}

impl State for State5 {
    fn new() -> Self {
        Self {
            key0_down: false,
            key1_down: false,
            key2_down: false,
            key3_down: false,
            next_state: None,
        }
    }

    fn input(&mut self, system: &mut crate::system::SystemComponents) {
        if !system.key0_pressed() && self.key0_down {
            self.next_state = Some(AppState::GamePlay);
        }
        self.key0_down = system.key0_pressed();
        self.key1_down = system.key1_pressed();
        self.key2_down = system.key2_pressed();
        self.key3_down = system.key3_pressed();
    }

    fn tick(&mut self, _system: &mut crate::system::SystemComponents) {
        //
    }

    fn sound(&mut self, system: &mut crate::system::SystemComponents) {
        system.end_tone();
    }

    fn draw(&mut self, _system: &mut crate::system::SystemComponents) {
        render::flood(0b000_000_00);
        text_writer::full_dialog_box("NOT IMPL", "todo!()");
    }

    fn swap(&mut self, system: &mut crate::system::SystemComponents) {
        render::draw(&mut system.display);
    }

    fn next_state(&self) -> &Option<super::AppState> {
        &self.next_state
    }
}
