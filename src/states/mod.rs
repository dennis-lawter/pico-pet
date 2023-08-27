use crate::system::SystemComponents;

pub mod game_play_state;
pub mod menu_state;

#[derive(Clone)]
pub enum AppState {
    GamePlay,
    Menu,
}

pub trait State {
    fn new() -> Self;

    fn input(&mut self, system: &mut SystemComponents);
    fn tick(&mut self, system: &mut SystemComponents);
    fn sound(&mut self, system: &mut SystemComponents);
    fn draw(&mut self, system: &mut SystemComponents);
    fn swap(&mut self, system: &mut SystemComponents);

    fn next_state(&mut self) -> &Option<AppState>;
}
