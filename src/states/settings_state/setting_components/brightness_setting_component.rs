use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::globals;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;

use super::SettingComponentTrait;

pub struct BrightnessSettingComponent {}

impl Default for BrightnessSettingComponent {
    fn default() -> Self {
        Self {}
    }
}

impl SettingComponentTrait for BrightnessSettingComponent {
    fn draw(&mut self, y_offset: i32, selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "BRIGHTNESS",
        );
        text_writer::draw_text(
            24,
            y_offset + 8,
            FontStyle::Icon,
            Rgb332::BLUE,
            unsafe { &globals::BRIGHTNESS_SETTING }.generate_bar(selected),
        );
    }

    fn tick(&mut self) {}

    fn input(&mut self) {
        let input = crate::globals::get_input();
        let setting = unsafe { &mut globals::BRIGHTNESS_SETTING };

        if input.get_state(&KeyNames::Left).is_down && input.get_state(&KeyNames::Right).is_down {
            return;
        }

        if input.get_state(&KeyNames::Left).key_repeat_triggered {
            setting.dec();
        } else if input.get_state(&KeyNames::Right).key_repeat_triggered {
            setting.inc();
        }
    }

    fn is_deselected(&mut self) -> bool {
        false
    }
}
