use fixedstr::str_format;

use crate::game::color::Rgb332;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::nvm::settings::SettingType;
use crate::game::setting_value::Setting;

use super::adjust_setting;
use super::check_if_confirming;
use super::check_if_exiting;
use super::SettingComponentTrait;

pub struct LongRestSettingComponent {
    will_be_deselected: bool,
    initial_value: u8,
    setting: Setting,
}

impl Default for LongRestSettingComponent {
    fn default() -> Self {
        let nvm = crate::game::globals::get_nvm();
        let setting = nvm.settings.get_setting(SettingType::LongRestMinutes);
        let initial_value = setting.get_value();
        Self {
            will_be_deselected: false,
            initial_value,
            setting,
        }
    }
}

impl SettingComponentTrait for LongRestSettingComponent {
    fn draw(&mut self, y_offset: i32, selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "LONG REST TIME",
        );
        if selected {
            let time_str = str_format!(fixedstr::str12, "{:02}        ", self.setting.get_value(),);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                y_offset + 8 - 1,
                FontStyle::Small,
                Rgb332::BLUE,
                time_str.as_str(),
            );
        } else {
            let time_str = str_format!(fixedstr::str12, "{:02}        ", self.setting.get_value(),);
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
        let nvm = crate::game::globals::get_nvm();
        if check_if_confirming() {
            self.will_be_deselected = true;
        } else if check_if_exiting() {
            nvm.settings
                .set_value(SettingType::LongRestMinutes, self.initial_value);
            self.will_be_deselected = true;
        } else {
            adjust_setting(&mut self.setting);
            nvm.settings
                .set_value(SettingType::LongRestMinutes, self.setting.get_value());
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset_internal_state(&mut self) {
        *self = Self::default();
    }
}
