use crate::{
    display::{
        render,
        sprite::{Sprite, SpriteFactory},
    },
    system::{Frequency, SystemComponents},
};

use super::{AppState, State};

enum MenuSelection {
    Item0,
    Item1,
    Item2,
    Item3,
    Item4,
    Item5,
    Item6,
    Item7,
    Item8,
    Settings,

    None,
}
impl MenuSelection {
    const MAX_VALUE: u8 = 9;
    fn from_u8(value: u8) -> Self {
        match value {
            0 => MenuSelection::Item0,
            1 => MenuSelection::Item1,
            2 => MenuSelection::Item2,
            3 => MenuSelection::Item3,
            4 => MenuSelection::Item4,
            5 => MenuSelection::Item5,
            6 => MenuSelection::Item6,
            7 => MenuSelection::Item7,
            8 => MenuSelection::Item8,
            9 => MenuSelection::Settings,
            _ => MenuSelection::None,
        }
    }
    fn to_u8(&self) -> u8 {
        match self {
            MenuSelection::Item0 => 0,
            MenuSelection::Item1 => 1,
            MenuSelection::Item2 => 2,
            MenuSelection::Item3 => 3,
            MenuSelection::Item4 => 4,
            MenuSelection::Item5 => 5,
            MenuSelection::Item6 => 6,
            MenuSelection::Item7 => 7,
            MenuSelection::Item8 => 8,
            MenuSelection::Settings => 9,
            MenuSelection::None => 255, // TODO: remove gross sentinal value
        }
    }
    fn next(&self) -> MenuSelection {
        let mut value = self.to_u8();
        value += 1;
        if value > Self::MAX_VALUE {
            value = 0; // loops to 0
        }
        Self::from_u8(value)
    }
    fn prev(&self) -> MenuSelection {
        let mut value = self.to_u8();
        if value == 0 {
            value = Self::MAX_VALUE; // loops to the MAX_VALUE
        } else {
            value -= 1;
        }
        Self::from_u8(value)
    }
}

pub struct GamePlayState<'a> {
    ferris: Sprite<'a>,
    menu_sprite: Sprite<'a>,
    frame_count: u32,
    next_state: Option<AppState>,
    menu_item_selected: MenuSelection,
    key0_down: bool,
    key1_down: bool,
    key2_down: bool,
    key3_down: bool,
}
impl State for GamePlayState<'static> {
    fn new() -> Self {
        let ferris = SpriteFactory::new_ferris_sprite(
            (128 - SpriteFactory::FERRIS_DIMENSIONS.0 as i32) / 2,
            128 - 48,
        );

        let menu_sprite = SpriteFactory::new_menu_sprite(0, 0);

        Self {
            ferris,
            menu_sprite,
            frame_count: 0,
            next_state: None,
            menu_item_selected: MenuSelection::Item0,
            key0_down: false,
            key1_down: false,
            key2_down: false,
            key3_down: false,
        }
    }

    fn tick(&mut self, _system: &mut SystemComponents) {
        self.frame_count += 1;
        if self.frame_count % 80 == 20 || self.frame_count % 80 == 0 {
            self.ferris.x -= 8;
        } else if self.frame_count % 80 == 40 || self.frame_count % 80 == 60 {
            self.ferris.x += 8;
        }
    }

    fn sound(&mut self, system: &mut SystemComponents) {
        if (self.frame_count / 20) % 2 == 1 {
            if self.frame_count % 4 == 0 {
                system.start_tone(&Frequency::C4);
            } else if self.frame_count % 4 == 2 {
                system.start_tone(&Frequency::A4);
            } else {
                system.start_tone(&Frequency::None);
            }
        } else {
            system.end_tone();
        }
    }

    fn draw(&mut self, _system: &mut SystemComponents) {
        render::flood(0b010_010_01);

        // self.corro.draw(0);

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

        let sel_x = self.menu_item_selected.to_u8() % 5 * 24 + 5;
        let sel_y = self.menu_item_selected.to_u8() / 5 * (128 - 24);
        render::fancy_border(sel_x as i32, sel_y as i32, 24, 24);
        // let text = "DIALOG\\b700!\\b703 so \\c700smol\\c003\\\\ so cute";
        // text_writer::bottom_dialog_box(text);
        // text_writer::bottom_dialog_box(text);
        // text_writer::bottom_dialog_box(text);
        // text_writer::bottom_dialog_box(text);
        // text_writer::bottom_dialog_box(text);
    }

    fn swap(&mut self, system: &mut SystemComponents) {
        system.set_backlight();
        render::draw(&mut system.display);
    }

    fn input(&mut self, system: &mut SystemComponents) {
        if !system.key1_pressed() && !system.key2_pressed() {
            if self.key1_down && !self.key2_down {
                self.menu_item_selected = self.menu_item_selected.prev();
            } else if self.key2_down && !self.key1_down {
                self.menu_item_selected = self.menu_item_selected.next();
            }
        }

        if !system.key3_pressed() && self.key3_down {
            match self.menu_item_selected {
                MenuSelection::Item0 => {}
                MenuSelection::Item1 => {}
                MenuSelection::Item2 => {}
                MenuSelection::Item3 => {}
                MenuSelection::Item4 => {}
                MenuSelection::Item5 => {}
                MenuSelection::Item6 => {}
                MenuSelection::Item7 => {}
                MenuSelection::Item8 => {}
                MenuSelection::Settings => {
                    self.next_state = Some(AppState::Menu);
                }
                _ => {}
            }
        }

        self.key0_down = system.key0_pressed();
        self.key1_down = system.key1_pressed();
        self.key2_down = system.key2_pressed();
        self.key3_down = system.key3_pressed();
    }

    fn next_state(&mut self) -> &Option<super::AppState> {
        &self.next_state
    }
}
