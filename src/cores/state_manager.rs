use crate::states::game_play_state::GamePlayState;
use crate::states::settings_state::SettingsState;
use crate::states::state_nyi::StateNyi;
use crate::states::AppState;
use crate::states::State;

#[derive(Default)]
pub struct StateManager<'a> {
    pub game_play_state: Option<GamePlayState<'a>>,
    pub pomo_state: Option<StateNyi>,
    pub eat_state: Option<StateNyi>,
    pub stat_state: Option<StateNyi>,
    pub cosmetic_state: Option<StateNyi>,
    pub settings_state: Option<SettingsState>,

    pub active_state: AppState,
}
impl StateManager<'static> {
    fn get_state(&mut self) -> &mut dyn State {
        match self.active_state {
            AppState::GamePlay => self.game_play_state.as_mut().unwrap(),
            AppState::SettingsState => self.settings_state.as_mut().unwrap(),
            AppState::PomoState => self.pomo_state.as_mut().unwrap(),
            AppState::EatState => self.eat_state.as_mut().unwrap(),
            AppState::StatState => self.stat_state.as_mut().unwrap(),
            AppState::CosmeticState => self.cosmetic_state.as_mut().unwrap(),
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
                self.pomo_state = None;
                self.eat_state = None;
                self.stat_state = None;
                self.cosmetic_state = None;
                self.settings_state = None;

                match next_state {
                    AppState::GamePlay => self.game_play_state = Some(GamePlayState::default()),
                    AppState::PomoState => self.pomo_state = Some(StateNyi::default()),
                    AppState::EatState => self.eat_state = Some(StateNyi::default()),
                    AppState::StatState => self.stat_state = Some(StateNyi::default()),
                    AppState::CosmeticState => self.cosmetic_state = Some(StateNyi::default()),
                    AppState::SettingsState => self.settings_state = Some(SettingsState::default()),
                }
            }
            None => {}
        }
    }
}
