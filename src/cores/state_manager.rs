use crate::states::{
    farm_state::FarmState, game_play_state::GamePlayState, select_food_state::SelectFoodState,
    settings_state::SettingsState, state_nyi::StateNyi, AppState, State,
};

#[derive(Default)]
pub struct StateManager<'a> {
    pub game_play_state: Option<GamePlayState<'a>>,
    pub select_food_state: Option<SelectFoodState>,
    pub state_nyi: Option<StateNyi>,
    pub settings_state: Option<SettingsState>,
    pub farm_state: Option<FarmState>,

    pub active_state: AppState,
}
impl StateManager<'static> {
    fn get_state(&mut self) -> &mut dyn State {
        match self.active_state {
            AppState::GamePlay => self.game_play_state.as_mut().unwrap(),
            AppState::SelectFood => self.select_food_state.as_mut().unwrap(),
            AppState::Settings => self.settings_state.as_mut().unwrap(),
            AppState::FarmState => self.farm_state.as_mut().unwrap(),

            AppState::AppState2
            | AppState::AppState3
            | AppState::AppState4
            | AppState::AppState5
            | AppState::AppState6
            | AppState::AppState7
            | AppState::AppState8 => self.state_nyi.as_mut().unwrap(),
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
                self.settings_state = None;
                self.farm_state = None;

                match next_state {
                    AppState::GamePlay => self.game_play_state = Some(GamePlayState::new()),
                    AppState::SelectFood => self.select_food_state = Some(SelectFoodState::new()),
                    AppState::Settings => self.settings_state = Some(SettingsState::new()),
                    AppState::FarmState => self.farm_state = Some(FarmState::new()),

                    AppState::AppState2
                    | AppState::AppState3
                    | AppState::AppState4
                    | AppState::AppState5
                    | AppState::AppState6
                    | AppState::AppState7
                    | AppState::AppState8 => self.state_nyi = Some(StateNyi::new()),
                }
            }
            None => {}
        }
    }
}
