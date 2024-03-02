pub mod main_state;
pub mod pomo_state;
pub mod settings_state;
pub mod state_nyi;

#[derive(Clone)]
pub enum AppState {
    Main,

    PomoState,
    EatState,
    StatState,
    CosmeticState,
    SettingsState,
}
impl Default for AppState {
    fn default() -> Self {
        Self::Main
    }
}

pub trait State {
    fn input(&mut self);
    fn tick(&mut self);
    fn sound(&mut self);
    fn draw(&mut self);

    fn next_state(&self) -> &Option<AppState>;
}
