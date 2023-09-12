mod menu_selection;

use crate::{
    display::{
        render,
        sprite::{Sprite, SpriteFactory},
    },
    hardware::Frequency,
};

use self::menu_selection::MenuSelection;

use super::{AppState, State};

pub struct GamePlayState<'a> {
    ferris: Sprite<'a>,
    menu_sprite: Sprite<'a>,
    frame_count: u32,
    next_state: Option<AppState>,
    menu_item_selected: MenuSelection,
    menu_select_tone_timer: u8,
    key0_down: bool,
    key1_down: bool,
    key2_down: bool,
    key3_down: bool,
}
impl State for GamePlayState<'static> {
    fn new() -> Self {
        let ferris = SpriteFactory::new_ferris_sprite(
            (128 - SpriteFactory::FERRIS_DIMENSIONS.0 as i32) / 2,
            128 - 64,
        );

        let menu_sprite = SpriteFactory::new_menu_sprite(0, 0);

        Self {
            ferris,
            menu_sprite,
            frame_count: 0,
            next_state: None,
            menu_item_selected: MenuSelection::Feed,
            menu_select_tone_timer: 0,
            key0_down: false,
            key1_down: false,
            key2_down: false,
            key3_down: false,
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        if self.frame_count % 80 == 20 || self.frame_count % 80 == 0 {
            self.ferris.x -= 8;
        } else if self.frame_count % 80 == 40 || self.frame_count % 80 == 60 {
            self.ferris.x += 8;
        }
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        if self.menu_select_tone_timer > 0 {
            self.menu_select_tone_timer -= 1;
            hardware.start_tone(&Frequency::Ds6);
        } else {
            if (self.frame_count / 20) % 2 == 1 {
                if self.frame_count % 4 == 0 {
                    hardware.start_tone(&Frequency::C4);
                } else if self.frame_count % 4 == 2 {
                    hardware.start_tone(&Frequency::A4);
                } else {
                    hardware.start_tone(&Frequency::None);
                }
            } else {
                hardware.end_tone();
            }
        }
    }

    fn draw(&mut self) {
        render::flood(0b010_010_01);

        self.ferris.draw(((self.frame_count / 20) % 2) as usize);

        for column in 0..5 {
            self.menu_sprite.x = column * 24 + 4;
            self.menu_sprite.y = 0;

            self.menu_sprite.draw(column as usize);
        }
        for column in 0..5 {
            self.menu_sprite.x = column * 24 + 4;
            self.menu_sprite.y = 128 - 24;

            self.menu_sprite.draw((column + 5) as usize);
        }

        let sel_x: i32 = self.menu_item_selected as u8 as i32 % 5 * 24 + 5;
        let sel_y: i32 = self.menu_item_selected as u8 as i32 / 5 * (128 - 24);
        render::fancy_border(sel_x as i32, sel_y as i32, 24, 24);
    }

    fn input(&mut self) {
        let hardware = crate::globals::get_hardware();
        if !(hardware.key1_pressed() && hardware.key2_pressed()) {
            if hardware.key1_pressed() && !self.key1_down {
                self.menu_item_selected = self.menu_item_selected.prev();
                self.menu_select_tone_timer = 3;
            } else if hardware.key2_pressed() && !self.key2_down {
                self.menu_item_selected = self.menu_item_selected.next();
                self.menu_select_tone_timer = 3;
            }
        }

        if !hardware.key3_pressed() && self.key3_down {
            self.menu_button_confirmed();
        }

        self.key0_down = hardware.key0_pressed();
        self.key1_down = hardware.key1_pressed();
        self.key2_down = hardware.key2_pressed();
        self.key3_down = hardware.key3_pressed();
    }

    fn next_state(&self) -> &Option<super::AppState> {
        &self.next_state
    }
}

impl GamePlayState<'static> {
    fn menu_button_confirmed(&mut self) {
        match self.menu_item_selected {
            MenuSelection::Feed => self.next_state = Some(AppState::SelectFood),
            MenuSelection::Item1 => self.next_state = Some(AppState::AppState1),
            MenuSelection::Item2 => self.next_state = Some(AppState::AppState2),
            MenuSelection::Item3 => self.next_state = Some(AppState::AppState3),
            MenuSelection::Item4 => self.next_state = Some(AppState::AppState4),
            MenuSelection::Item5 => self.next_state = Some(AppState::AppState5),
            MenuSelection::Item6 => self.next_state = Some(AppState::AppState6),
            MenuSelection::Item7 => self.next_state = Some(AppState::AppState7),
            MenuSelection::Item8 => self.next_state = Some(AppState::AppState8),
            MenuSelection::Settings => self.next_state = Some(AppState::Settings),
            MenuSelection::None => {}
        }
    }
}
