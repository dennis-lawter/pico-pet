use crate::system::SystemComponents;

pub mod game_play_state;
pub mod select_food_state;
pub mod settings_state;
pub mod state_1;
pub mod state_2;
pub mod state_3;
pub mod state_4;
pub mod state_5;
pub mod state_6;
pub mod state_7;
pub mod state_8;

#[derive(Clone)]
pub enum AppState {
    GamePlay,

    SelectFood,
    AppState1,
    AppState2,
    AppState3,
    AppState4,
    AppState5,
    AppState6,
    AppState7,
    AppState8,
    Settings,
}

pub trait State {
    fn new() -> Self;

    fn input(&mut self, system: &mut SystemComponents);
    fn tick(&mut self, system: &mut SystemComponents);
    fn sound(&mut self, system: &mut SystemComponents);
    fn draw(&mut self, system: &mut SystemComponents);
    fn swap(&mut self, system: &mut SystemComponents);

    fn next_state(&self) -> &Option<AppState>;
}
