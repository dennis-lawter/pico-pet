use crate::states::main_state::MainState;
use crate::states::AppState;

use super::state_manager::StateManager;

pub fn primary_main_loop() -> ! {
    let mut state_manager = StateManager::default();
    state_manager.game_play_state = Some(MainState::default());
    state_manager.active_state = AppState::Main;

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
