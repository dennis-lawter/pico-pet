use crate::system::System;

use self::{game_play_state::GamePlayState, menu_state::MenuState};

pub mod game_play_state;
pub mod menu_state;

pub trait State<'a> {
    fn enter_state(system_ptr: *mut System) -> AppState<'a>;

    fn input(&mut self);
    fn tick(&mut self);
    fn sound(&mut self);
    fn draw(&mut self);

    fn exit_state(&mut self);

    fn next_state(&self) -> Option<AppState>;
}

pub trait StateManager {
    fn input(&mut self);
    fn tick(&mut self);
    fn sound(&mut self);
    fn draw(&mut self);

    fn exit_state(&mut self);

    fn next_state(&self) -> Option<AppState>;
}

pub enum AppState<'a> {
    GamePlay(GamePlayState<'a>),
    Menu(MenuState),
}
impl StateManager for AppState<'static> {
    fn input(&mut self) {
        match self {
            AppState::GamePlay(game_play_state) => game_play_state.input(),
            AppState::Menu(menu_state) => menu_state.input(),
        }
    }

    fn tick(&mut self) {
        match self {
            AppState::GamePlay(game_play_state) => game_play_state.tick(),
            AppState::Menu(menu_state) => menu_state.tick(),
        }
    }

    fn sound(&mut self) {
        match self {
            AppState::GamePlay(game_play_state) => game_play_state.sound(),
            AppState::Menu(menu_state) => menu_state.sound(),
        }
    }

    fn draw(&mut self) {
        match self {
            AppState::GamePlay(game_play_state) => game_play_state.draw(),
            AppState::Menu(menu_state) => menu_state.draw(),
        }
    }

    fn exit_state(&mut self) {
        match self {
            AppState::GamePlay(game_play_state) => game_play_state.exit_state(),
            AppState::Menu(menu_state) => menu_state.exit_state(),
        }
    }

    fn next_state(&self) -> Option<AppState> {
        match self {
            AppState::GamePlay(game_play_state) => game_play_state.next_state(),
            AppState::Menu(menu_state) => menu_state.next_state(),
        }
    }
}
