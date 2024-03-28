use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::hardware::hardware::LCD_WIDTH;
use crate::nvm::settings::SettingType;
use crate::setting_value::Setting;

use super::adjust_setting;
use super::check_if_confirming;
use super::check_if_exiting;
use super::SettingComponentTrait;

pub struct PomoCycleSettingComponent {
    pub will_be_deselected: bool,
    pub setting: Setting,
    pub initial_value: u8,
}

impl Default for PomoCycleSettingComponent {
    fn default() -> Self {
        let nvm = crate::globals::get_nvm();
        let setting = nvm.settings.get_setting(SettingType::PomodoroCycles);
        let initial_value = setting.get_value();
        Self {
            will_be_deselected: false,
            setting,
            initial_value,
        }
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
        if selected {
            let time_str = str_format!(fixedstr::str12, "{}       ", self.setting.get_value(),);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                y_offset + 8 - 1,
                FontStyle::Small,
                Rgb332::BLUE,
                time_str.as_str(),
            );
        } else {
            let time_str = str_format!(fixedstr::str12, "{}       ", self.setting.get_value(),);
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
        let nvm = crate::globals::get_nvm();
        if check_if_confirming() {
            self.will_be_deselected = true;
        } else if check_if_exiting() {
            nvm.settings
                .set_value(SettingType::PomodoroCycles, self.initial_value);
            self.will_be_deselected = true;
        } else {
            adjust_setting(&mut self.setting);
            nvm.settings
                .set_value(SettingType::PomodoroCycles, self.setting.get_value());
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset_internal_state(&mut self) {
        *self = Self::default();
    }
}
