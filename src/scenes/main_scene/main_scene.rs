use crate::display::render;
use crate::display::sprite::Sprite;
use crate::display::sprite::SpriteFactory;
use crate::hardware::audio::AudioFrequency;
use crate::hardware::input::KeyNames;
use crate::scenes::SceneBehavior;
use crate::scenes::SceneType;

use super::menu_selection::MenuSelection;

pub struct MainScene<'a> {
    ferris: Sprite<'a>,
    menu_sprite: Sprite<'a>,
    frame_count: u32,
    next_scene: Option<SceneType>,
    menu_item_selected: MenuSelection,
    menu_select_tone_timer: u8,
}
impl SceneBehavior for MainScene<'static> {
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
            hardware.start_tone(&AudioFrequency::Ds6);
        } else {
            if (self.frame_count / 20) % 2 == 1 {
                if self.frame_count % 4 == 0 {
                    hardware.start_tone(&AudioFrequency::C4);
                } else if self.frame_count % 4 == 2 {
                    hardware.start_tone(&AudioFrequency::A4);
                } else {
                    hardware.start_tone(&AudioFrequency::None);
                }
            } else {
                hardware.end_tone();
            }
        }
    }

    fn draw(&mut self) {
        // render::flood(Rgb332::from_u8(0b000_000_01));
        self.ferris.draw(((self.frame_count / 20) % 2) as usize);

        for column in 0..5 {
            self.menu_sprite.x = column * 24 + 4;
            self.menu_sprite.y = 128 - 24;

            self.menu_sprite.draw((column) as usize);
        }

        let sel_x: i32 = self.menu_item_selected as u8 as i32 * 24 + 4;
        let sel_y: i32 = 128 - 24;
        render::fancy_border(sel_x as i32, sel_y as i32, 24, 24);
    }

    fn input(&mut self) {
        let input = crate::globals::get_input();

        if input.get_state(&KeyNames::Left).just_pressed {
            self.menu_item_selected = self.menu_item_selected.prev();
            self.menu_select_tone_timer = 3;
        }
        if input.get_state(&KeyNames::Right).just_pressed {
            self.menu_item_selected = self.menu_item_selected.next();
            self.menu_select_tone_timer = 3;
        }

        if input.get_state(&KeyNames::Confirm).just_released {
            self.menu_button_confirmed();
        }
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}

impl MainScene<'static> {
    fn menu_button_confirmed(&mut self) {
        match self.menu_item_selected {
            MenuSelection::Pomo => self.next_scene = Some(SceneType::Pomo),
            MenuSelection::Eat => self.next_scene = Some(SceneType::Eat),
            MenuSelection::Stat => self.next_scene = Some(SceneType::Stat),
            MenuSelection::Cosmetic => self.next_scene = Some(SceneType::Cosmetics),
            MenuSelection::Settings => self.next_scene = Some(SceneType::Settings),
            MenuSelection::None => {}
        }
    }
}

impl Default for MainScene<'static> {
    fn default() -> Self {
        let ferris = SpriteFactory::new_ferris_sprite(
            (128 - SpriteFactory::FERRIS_DIMENSIONS.0 as i32) / 2,
            128 - 64,
        );

        let menu_sprite = SpriteFactory::new_menu_sprite(0, 0);

        Self {
            ferris,
            menu_sprite,
            frame_count: 0,
            next_scene: None,
            menu_item_selected: MenuSelection::Pomo,
            menu_select_tone_timer: 0,
        }
    }
}
