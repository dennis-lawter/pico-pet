use crate::{
    states::{game_play_state::GamePlayState, menu_state::MenuState, AppState, State},
    system::SystemComponents,
};

pub fn primary_main_loop(system: &mut SystemComponents) -> ! {
    let mut game_play_state: Option<GamePlayState> = Some(GamePlayState::new());
    let mut menu_state: Option<MenuState> = None;

    let mut active_state: AppState = AppState::GamePlay;

    loop {
        match active_state {
            AppState::GamePlay => {
                game_play_state.as_mut().unwrap().input(system);
                game_play_state.as_mut().unwrap().tick(system);
                game_play_state.as_mut().unwrap().draw(system);
                game_play_state.as_mut().unwrap().sound(system);
                game_play_state.as_mut().unwrap().swap(system);
            }
            AppState::Menu => {
                menu_state.as_mut().unwrap().input(system);
                menu_state.as_mut().unwrap().tick(system);
                menu_state.as_mut().unwrap().draw(system);
                menu_state.as_mut().unwrap().sound(system);
                menu_state.as_mut().unwrap().swap(system);
            }
        }

        let next_state = match active_state {
            AppState::GamePlay => {
                let state = game_play_state.as_mut().unwrap();
                state.next_state()
            }
            AppState::Menu => {
                let state = menu_state.as_mut().unwrap();
                state.next_state()
            }
        };
        match next_state {
            Some(new_state) => {
                active_state = new_state.clone();
                match new_state {
                    crate::states::AppState::GamePlay => {
                        game_play_state = Some(GamePlayState::new());
                    }
                    crate::states::AppState::Menu => {
                        menu_state = Some(MenuState::new());
                    }
                }
            }
            None => {}
        }
    }
}
