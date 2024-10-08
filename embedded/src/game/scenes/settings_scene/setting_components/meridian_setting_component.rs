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

pub struct MeridianSettingComponent {
    pub will_be_deselected: bool,
    pub setting: Setting,
    pub initial_value: u8,
}

impl Default for MeridianSettingComponent {
    fn default() -> Self {
        let nvm = crate::game::globals::get_nvm();
        let setting = nvm.settings.get_setting(SettingType::UseMeridian);
        let initial_value = setting.get_value();
        Self {
            will_be_deselected: false,
            setting,
            initial_value,
        }
    }
}

impl SettingComponentTrait for MeridianSettingComponent {
    fn draw(&mut self, y_offset: i32, _selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "CLOCK SETTING",
        );
        let value = self.setting.get_value();
        if value == 0 {
            text_writer::draw_text(
                24,
                y_offset + 8,
                FontStyle::Icon,
                Rgb332::BLUE,
                "44444444        ",
            );
            text_writer::draw_text(
                24,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::WHITE,
                "  24hr          ",
            );
            text_writer::draw_text(
                24,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::LIGHT_GREY,
                "          12hr  ",
            );
        } else {
            text_writer::draw_text(
                24,
                y_offset + 8,
                FontStyle::Icon,
                Rgb332::BLUE,
                "        44444444",
            );
            text_writer::draw_text(
                24,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::LIGHT_GREY,
                "  24hr          ",
            );
            text_writer::draw_text(
                24,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::WHITE,
                "          12hr  ",
            );
        }
    }

    fn tick(&mut self) {}

    fn input(&mut self) {
        let nvm = crate::game::globals::get_nvm();
        if check_if_confirming() {
            self.will_be_deselected = true;
        } else if check_if_exiting() {
            nvm.settings
                .set_value(SettingType::UseMeridian, self.initial_value);
            self.will_be_deselected = true;
        } else {
            adjust_setting(&mut self.setting);
            nvm.settings
                .set_value(SettingType::UseMeridian, self.setting.get_value());
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset_internal_state(&mut self) {
        *self = Self::default();
    }
}
