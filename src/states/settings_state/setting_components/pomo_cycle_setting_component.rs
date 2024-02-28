use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::globals;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;

use super::SettingComponentTrait;

pub struct PomoCycleSettingComponent {}

impl Default for PomoCycleSettingComponent {
    fn default() -> Self {
        Self {}
    }
}

impl SettingComponentTrait for PomoCycleSettingComponent {
    fn draw(&mut self, y_offset: i32, selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "POMODORO CYCLES",
        );
        let pomo_time_setting = unsafe { &globals::POMO_CYCLE_SETTING }.get_value();
        if selected {
            let time_str = str_format!(fixedstr::str12, "{}       ", pomo_time_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                y_offset + 8 - 1,
                FontStyle::Small,
                Rgb332::BLUE,
                time_str.as_str(),
            );
        } else {
            let time_str = str_format!(fixedstr::str12, "{}       ", pomo_time_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::BLACK,
                time_str.as_str(),
            );
        }
        let min_str = "  cycles";
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset + 8,
            FontStyle::Small,
            Rgb332::BLACK,
            min_str,
        );
    }

    fn tick(&mut self) {}

    fn input(&mut self) {
        let input = crate::globals::get_input();
        let setting = unsafe { &mut globals::POMO_CYCLE_SETTING };

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
