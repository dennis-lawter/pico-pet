use crate::states::{game_play_state::GamePlayState, AppState};

use super::state_manager::StateManager;

pub fn primary_main_loop() -> ! {
    let mut state_manager = StateManager::default();
    state_manager.game_play_state = Some(GamePlayState::default());
    state_manager.active_state = AppState::GamePlay;

    loop {
        let input = crate::globals::get_input();
        input.update();
        state_manager.update_and_draw();

        swap();

        state_manager.advance_state();
    }
}

fn swap() {
    let hardware = crate::globals::get_hardware();
    hardware.set_backlight();
    crate::display::render::draw(&mut hardware.display);
}
