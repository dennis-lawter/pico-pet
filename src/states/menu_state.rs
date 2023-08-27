use crate::{
    display::{
        render,
        sprite::{Sprite, SpriteFactory},
        text_writer::{self, FontStyle},
    },
    globals,
    setting_value::Setting,
    system::{Frequency, SystemComponents},
};

use super::{AppState, State};

const KEY_REPEAT_FRAMES: u8 = 5;

pub struct MenuState {
    frame_count: u32,
    key_repeat_slowdown_timer: u8,
    next_state: Option<AppState>,
}
impl State for MenuState {
    fn new() -> Self {
        Self {
            frame_count: 0,
            key_repeat_slowdown_timer: 0,
            next_state: None,
        }
    }

    fn tick(&mut self, system: &mut SystemComponents) {
        self.frame_count += 1;
    }

    fn sound(&mut self, system: &mut SystemComponents) {
        system.end_tone();
    }

    fn draw(&mut self, system: &mut SystemComponents) {
        render::flood(0b000_000_00);

        let title = "BRIGHTNESS";
        let menu_body = "";
        text_writer::full_dialog_box(title, menu_body);
        text_writer::draw_text(
            24,
            18,
            FontStyle::Icon,
            0b000_000_11,
            unsafe { &globals::BRIGHTNESS_SETTING }.generate_bar(),
        );
    }

    fn swap(&mut self, system: &mut SystemComponents) {
        system.set_backlight(unsafe { &globals::BRIGHTNESS_SETTING });
        render::draw(&mut system.display);
    }

    fn input(&mut self, system: &mut SystemComponents) {
        if system.key0_pressed() {
            self.next_state = Some(AppState::GamePlay);
            return;
        }

        if system.key1_pressed() && !system.key2_pressed() {
            if self.key_repeat_slowdown_timer == 0 {
                self.key_repeat_slowdown_timer = KEY_REPEAT_FRAMES;
                unsafe { &mut globals::BRIGHTNESS_SETTING }.dec();
            } else {
                self.key_repeat_slowdown_timer -= 1;
            }
        } else if system.key2_pressed() && !system.key1_pressed() {
            if self.key_repeat_slowdown_timer == 0 {
                self.key_repeat_slowdown_timer = 5;
                unsafe { &mut globals::BRIGHTNESS_SETTING }.inc();
            } else {
                self.key_repeat_slowdown_timer -= 1;
            }
        } else {
            self.key_repeat_slowdown_timer = 0;
        }
    }

    fn next_state(&mut self) -> &Option<super::AppState> {
        &self.next_state
    }
}
