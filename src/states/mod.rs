pub mod farm_state;
pub mod game_play_state;
pub mod select_food_state;
pub mod settings_state;
pub mod state_nyi;

#[derive(Clone)]
pub enum AppState {
    GamePlay,

    SelectFood,
    FarmState,
    AppState2,
    AppState3,
    AppState4,
    AppState5,
    AppState6,
    AppState7,
    AppState8,
    Settings,
}
impl Default for AppState {
    fn default() -> Self {
        Self::GamePlay
    }
}

pub trait State {
    fn input(&mut self);
    fn tick(&mut self);
    fn sound(&mut self);
    fn draw(&mut self);

    fn next_state(&self) -> &Option<AppState>;
}
