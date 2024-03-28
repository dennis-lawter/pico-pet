use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;

use super::SettingComponentTrait;

const FRAMES_TO_RESET: u32 = 5 * 8 * 2; // this is pixel based... the bar grows 1 pixel per tick
pub struct ResetSettingComponent {
    frames_reset_button_held: u32,
    pub will_be_deselected: bool,
}

impl Default for ResetSettingComponent {
    fn default() -> Self {
        Self {
            frames_reset_button_held: 0,
            will_be_deselected: false,
        }
    }
}

impl SettingComponentTrait for ResetSettingComponent {
    fn draw(&mut self, y_offset: i32, _selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::from_u8(0b110_000_00),
            "!!! RESET ALL DATA !!!",
        );

        crate::display::render::solid_line_rect(
            LCD_WIDTH as i32 / 2 - (5 * 8),
            y_offset + 8,
            5 * 16,
            8,
            Rgb332::from_u8(0b110_000_00),
        );

        crate::display::render::fill_rect(
            LCD_WIDTH as i32 / 2 - (5 * 8) + 1,
            y_offset + 8 + 1,
            self.frames_reset_button_held as usize,
            6,
            Rgb332::from_u8(0b001_000_00),
        );
    }

    fn tick(&mut self) {}

    fn input(&mut self) {
        let input = crate::globals::get_input();
        let nvm = crate::globals::get_nvm();

        if input.get_state(&KeyNames::Back).just_released {
            self.will_be_deselected = true;
        }
        if input.get_state(&KeyNames::Confirm).is_down {
            if self.frames_reset_button_held < FRAMES_TO_RESET {
                self.frames_reset_button_held += 1;
            } else {
                nvm.erase_all_then_reboot();

                self.frames_reset_button_held = 0;
                self.will_be_deselected = true;
            }
        } else {
            self.frames_reset_button_held = 0;
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset_internal_state(&mut self) {
        self.will_be_deselected = false;
        self.frames_reset_button_held = 0;
    }
}
