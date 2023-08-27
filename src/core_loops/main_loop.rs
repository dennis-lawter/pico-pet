use crate::{
    state::{game_play_state::GamePlayState, AppState, State, StateManager},
    system::System,
};

pub fn main_loop(system: &mut System) -> ! {
    let system_ptr = system as *mut System;
    let mut app: AppState = GamePlayState::enter_state(system_ptr);
    loop {
        app.input();
        app.tick();
        app.draw();
        app.exit_state();

        // if let Some(next_state) = app.next_state() {
        //     app = next_state;
        // } else {
        //     break; // or continue, depending on your design
        // }
    }
}
