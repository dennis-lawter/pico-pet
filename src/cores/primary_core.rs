use crate::{
    hardware::HardwareComponents,
    states::{
        game_play_state::GamePlayState, select_food_state::SelectFoodState,
        settings_state::SettingsState, state_1::State1, state_2::State2, state_3::State3,
        state_4::State4, state_5::State5, state_6::State6, state_7::State7, state_8::State8,
        AppState, State,
    },
};

pub fn primary_main_loop(system: &mut HardwareComponents) -> ! {
    let mut game_play_state: Option<GamePlayState> = Some(GamePlayState::new());
    let mut select_food_state: Option<SelectFoodState> = None;
    let mut state_1: Option<State1> = None;
    let mut state_2: Option<State2> = None;
    let mut state_3: Option<State3> = None;
    let mut state_4: Option<State4> = None;
    let mut state_5: Option<State5> = None;
    let mut state_6: Option<State6> = None;
    let mut state_7: Option<State7> = None;
    let mut state_8: Option<State8> = None;
    let mut menu_state: Option<SettingsState> = None;

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
            AppState::SelectFood => {
                select_food_state.as_mut().unwrap().input(system);
                select_food_state.as_mut().unwrap().tick(system);
                select_food_state.as_mut().unwrap().draw(system);
                select_food_state.as_mut().unwrap().sound(system);
                select_food_state.as_mut().unwrap().swap(system);
            }
            AppState::AppState1 => {
                state_1.as_mut().unwrap().input(system);
                state_1.as_mut().unwrap().tick(system);
                state_1.as_mut().unwrap().draw(system);
                state_1.as_mut().unwrap().sound(system);
                state_1.as_mut().unwrap().swap(system);
            }
            AppState::AppState2 => {
                state_2.as_mut().unwrap().input(system);
                state_2.as_mut().unwrap().tick(system);
                state_2.as_mut().unwrap().draw(system);
                state_2.as_mut().unwrap().sound(system);
                state_2.as_mut().unwrap().swap(system);
            }
            AppState::AppState3 => {
                state_3.as_mut().unwrap().input(system);
                state_3.as_mut().unwrap().tick(system);
                state_3.as_mut().unwrap().draw(system);
                state_3.as_mut().unwrap().sound(system);
                state_3.as_mut().unwrap().swap(system);
            }
            AppState::AppState4 => {
                state_4.as_mut().unwrap().input(system);
                state_4.as_mut().unwrap().tick(system);
                state_4.as_mut().unwrap().draw(system);
                state_4.as_mut().unwrap().sound(system);
                state_4.as_mut().unwrap().swap(system);
            }
            AppState::AppState5 => {
                state_5.as_mut().unwrap().input(system);
                state_5.as_mut().unwrap().tick(system);
                state_5.as_mut().unwrap().draw(system);
                state_5.as_mut().unwrap().sound(system);
                state_5.as_mut().unwrap().swap(system);
            }
            AppState::AppState6 => {
                state_6.as_mut().unwrap().input(system);
                state_6.as_mut().unwrap().tick(system);
                state_6.as_mut().unwrap().draw(system);
                state_6.as_mut().unwrap().sound(system);
                state_6.as_mut().unwrap().swap(system);
            }
            AppState::AppState7 => {
                state_7.as_mut().unwrap().input(system);
                state_7.as_mut().unwrap().tick(system);
                state_7.as_mut().unwrap().draw(system);
                state_7.as_mut().unwrap().sound(system);
                state_7.as_mut().unwrap().swap(system);
            }
            AppState::AppState8 => {
                state_8.as_mut().unwrap().input(system);
                state_8.as_mut().unwrap().tick(system);
                state_8.as_mut().unwrap().draw(system);
                state_8.as_mut().unwrap().sound(system);
                state_8.as_mut().unwrap().swap(system);
            }
            AppState::Settings => {
                menu_state.as_mut().unwrap().input(system);
                menu_state.as_mut().unwrap().tick(system);
                menu_state.as_mut().unwrap().draw(system);
                menu_state.as_mut().unwrap().sound(system);
                menu_state.as_mut().unwrap().swap(system);
            }
        }

        let next_state = get_next_state(
            &active_state,
            &game_play_state,
            &select_food_state,
            &state_1,
            &state_2,
            &state_3,
            &state_4,
            &state_5,
            &state_6,
            &state_7,
            &state_8,
            &menu_state,
        );

        match next_state {
            Some(new_state) => {
                active_state = new_state.clone();
                game_play_state = None;
                select_food_state = None;
                state_1 = None;
                state_2 = None;
                state_3 = None;
                state_4 = None;
                state_5 = None;
                state_6 = None;
                state_7 = None;
                state_8 = None;
                menu_state = None;
                match new_state {
                    crate::states::AppState::GamePlay => {
                        game_play_state = Some(GamePlayState::new());
                    }

                    crate::states::AppState::SelectFood => {
                        select_food_state = Some(SelectFoodState::new());
                    }

                    crate::states::AppState::AppState1 => {
                        state_1 = Some(State1::new());
                    }

                    crate::states::AppState::AppState2 => {
                        state_2 = Some(State2::new());
                    }

                    crate::states::AppState::AppState3 => {
                        state_3 = Some(State3::new());
                    }

                    crate::states::AppState::AppState4 => {
                        state_4 = Some(State4::new());
                    }

                    crate::states::AppState::AppState5 => {
                        state_5 = Some(State5::new());
                    }

                    crate::states::AppState::AppState6 => {
                        state_6 = Some(State6::new());
                    }

                    crate::states::AppState::AppState7 => {
                        state_7 = Some(State7::new());
                    }

                    crate::states::AppState::AppState8 => {
                        state_8 = Some(State8::new());
                    }

                    crate::states::AppState::Settings => {
                        menu_state = Some(SettingsState::new());
                    }
                }
            }
            None => {}
        }
    }
}

fn get_next_state(
    active_state: &AppState,
    game_play_state: &Option<GamePlayState<'static>>,
    select_food_state: &Option<SelectFoodState>,
    state_1: &Option<State1>,
    state_2: &Option<State2>,
    state_3: &Option<State3>,
    state_4: &Option<State4>,
    state_5: &Option<State5>,
    state_6: &Option<State6>,
    state_7: &Option<State7>,
    state_8: &Option<State8>,
    menu_state: &Option<SettingsState>,
) -> Option<AppState> {
    match active_state {
        AppState::GamePlay => {
            let state = game_play_state.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::SelectFood => {
            let state = select_food_state.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::AppState1 => {
            let state = state_1.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::AppState2 => {
            let state = state_2.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::AppState3 => {
            let state = state_3.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::AppState4 => {
            let state = state_4.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::AppState5 => {
            let state = state_5.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::AppState6 => {
            let state = state_6.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::AppState7 => {
            let state = state_7.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::AppState8 => {
            let state = state_8.as_ref().unwrap();
            state.next_state().clone()
        }

        AppState::Settings => {
            let state = menu_state.as_ref().unwrap();
            state.next_state().clone()
        }
    }
}
