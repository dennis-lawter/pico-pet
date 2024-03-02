use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::globals;
use crate::hardware::hardware::LCD_WIDTH;

use super::adjust_setting;
use super::check_if_confirming;
use super::check_if_exiting;
use super::SettingComponentTrait;

pub struct PomoTimeSettingComponent {
    will_be_deselected: bool,
    pub initial_value: Option<u8>,
}

impl Default for PomoTimeSettingComponent {
    fn default() -> Self {
        Self {
            will_be_deselected: false,
            initial_value: None,
        }
    }
}

impl SettingComponentTrait for PomoTimeSettingComponent {
    fn draw(&mut self, y_offset: i32, selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "POMODORO TIME",
        );
        let pomo_time_setting = unsafe { &globals::POMO_TIME_SETTING }.get_value();
        if selected {
            let time_str = str_format!(fixedstr::str12, "{:02}        ", pomo_time_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                y_offset + 8 - 1,
                FontStyle::Small,
                Rgb332::BLUE,
                time_str.as_str(),
            );
        } else {
            let time_str = str_format!(fixedstr::str12, "{:02}        ", pomo_time_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::BLACK,
                time_str.as_str(),
            );
        }
        let min_str = "   minutes";
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
        let setting = unsafe { &mut globals::POMO_TIME_SETTING };
        if self.initial_value.is_none() {
            self.initial_value = Some(setting.get_value());
        }

        if check_if_confirming() {
            self.will_be_deselected = true;
        }
        if check_if_exiting() {
            self.will_be_deselected = true;
            setting.set_value(self.initial_value.unwrap()).unwrap();
        } else {
            adjust_setting(setting);
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset(&mut self) {
        self.will_be_deselected = false;
        self.initial_value = None;
    }
}
