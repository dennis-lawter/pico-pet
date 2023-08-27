use crate::system::System;

use super::{AppState, State};

pub struct MenuState;

impl State<'_> for MenuState {
    fn enter_state(system_ptr: *mut System) -> AppState<'static> {
        todo!()
    }

    fn input(&mut self) {
        todo!()
    }

    fn tick(&mut self) {
        todo!()
    }

    fn sound(&mut self) {
        todo!()
    }

    fn draw(&mut self) {
        todo!()
    }

    fn exit_state(&mut self) {
        todo!()
    }

    fn next_state(&self) -> Option<AppState> {
        todo!()
    }
}
