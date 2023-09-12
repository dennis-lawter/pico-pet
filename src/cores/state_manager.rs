use crate::states::{
    game_play_state::GamePlayState, select_food_state::SelectFoodState,
    settings_state::SettingsState, state_nyi::StateNyi, AppState, State,
};

#[derive(Default)]
pub struct StateManager<'a> {
    pub game_play_state: Option<GamePlayState<'a>>,
    pub select_food_state: Option<SelectFoodState>,
    pub state_nyi: Option<StateNyi>,
    pub menu_state: Option<SettingsState>,

    pub active_state: AppState,
}
impl StateManager<'static> {
    fn get_state(&mut self) -> &mut dyn State {
        match self.active_state {
            AppState::GamePlay => self.game_play_state.as_mut().unwrap(),
            AppState::SelectFood => self.select_food_state.as_mut().unwrap(),
            AppState::Settings => self.menu_state.as_mut().unwrap(),
            _ => self.state_nyi.as_mut().unwrap(),
        }
    }

    pub fn update_and_draw(&mut self) {
        let curr_state = self.get_state();
        curr_state.input();
        curr_state.tick();
        curr_state.sound();
        curr_state.draw();
    }

    pub fn advance_state(&mut self) {
        let curr_state = self.get_state();
        match curr_state.next_state().clone() {
            Some(next_state) => {
                self.active_state = next_state.clone();
                self.game_play_state = None;
                self.select_food_state = None;
                self.state_nyi = None;
                self.menu_state = None;
                match next_state {
                    AppState::GamePlay => self.game_play_state = Some(GamePlayState::new()),
                    AppState::SelectFood => self.select_food_state = Some(SelectFoodState::new()),
                    AppState::Settings => self.menu_state = Some(SettingsState::new()),
                    _ => self.state_nyi = Some(StateNyi::new()),
                }
            }
            None => {}
        }
    }
}
