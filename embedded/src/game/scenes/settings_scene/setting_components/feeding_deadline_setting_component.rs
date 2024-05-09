use fixedstr::str_format;

use crate::game::color::Rgb332;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::nvm::settings::SettingType;
use crate::game::setting_value::Setting;

use super::SettingComponentTrait;

#[allow(dead_code)]
pub struct FeedingDeadlineSettingComponent {
    pub initial_hr_setting: u8,
    pub hr_setting: Setting,
    pub initial_min_setting: u8,
    pub min_setting: Setting,
    pub new_time_selection: u8,
    pub will_be_deselected: bool,
}

impl Default for FeedingDeadlineSettingComponent {
    fn default() -> Self {
        let nvm = crate::game::globals::get_nvm();
        let hr_setting = nvm.settings.get_setting(SettingType::FeedingDeadlineHour);
        let initial_hr_setting = hr_setting.get_value();
        let min_setting = nvm.settings.get_setting(SettingType::FeedingDeadlineMinute);
        let initial_min_setting = min_setting.get_value();
        Self {
            initial_hr_setting,
            hr_setting,
            initial_min_setting,
            min_setting,
            new_time_selection: 0,
            will_be_deselected: false,
        }
    }
}

impl SettingComponentTrait for FeedingDeadlineSettingComponent {
    fn draw(&mut self, y_offset: i32, selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "FEEDING DEADLINE",
        );
        if selected {
            match self.new_time_selection {
                0 => {
                    let time_str = str_format!(
                        fixedstr::str16,
                        "  :{:02}",
                        // self.hr_setting.get_value(),
                        self.min_setting.get_value(),
                    );
                    text_writer::draw_text_centered(
                        LCD_WIDTH as i32 / 2,
                        y_offset + 8,
                        FontStyle::Small,
                        Rgb332::BLACK,
                        time_str.as_str(),
                    );
                    let active_time_str = str_format!(
                        fixedstr::str16,
                        "{:02}   ",
                        self.hr_setting.get_value(),
                        // self.min_setting.get_value(),
                    );
                    text_writer::draw_text_centered(
                        LCD_WIDTH as i32 / 2,
                        y_offset + 8 - 1,
                        FontStyle::Small,
                        Rgb332::BLUE,
                        active_time_str.as_str(),
                    );
                }
                1 => {
                    let time_str = str_format!(
                        fixedstr::str16,
                        "{:02}:  ",
                        self.hr_setting.get_value(),
                        // self.min_setting.get_value(),
                    );
                    text_writer::draw_text_centered(
                        LCD_WIDTH as i32 / 2,
                        y_offset + 8,
                        FontStyle::Small,
                        Rgb332::BLACK,
                        time_str.as_str(),
                    );
                    let active_time_str = str_format!(
                        fixedstr::str16,
                        "   {:02}",
                        // self.hr_setting.get_value(),
                        self.min_setting.get_value(),
                    );
                    text_writer::draw_text_centered(
                        LCD_WIDTH as i32 / 2,
                        y_offset + 8 - 1,
                        FontStyle::Small,
                        Rgb332::BLUE,
                        active_time_str.as_str(),
                    );
                }
                _ => {}
            }
        } else {
            let time_str = str_format!(
                fixedstr::str16,
                "{:02}:{:02}",
                self.hr_setting.get_value(),
                self.min_setting.get_value(),
            );
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::BLACK,
                time_str.as_str(),
            );
        }
    }

    fn tick(&mut self) {}

    fn input(&mut self) {
        let input = crate::game::globals::get_input();

        if input.get_state(&KeyNames::Back).just_released {
            match self.new_time_selection {
                0 => self.will_be_deselected = true,
                1 => self.new_time_selection = 0,
                _ => {}
            }
        } else if input.get_state(&KeyNames::Left).just_pressed {
            match self.new_time_selection {
                0 => {
                    self.hr_setting.dec();
                }
                1 => {
                    self.min_setting.dec();
                }
                _ => {}
            }
        } else if input.get_state(&KeyNames::Right).just_pressed {
            match self.new_time_selection {
                0 => {
                    self.hr_setting.inc();
                }
                1 => {
                    self.min_setting.inc();
                }
                _ => {}
            }
        } else if input.get_state(&KeyNames::Confirm).just_released {
            self.new_time_selection += 1;
            if self.new_time_selection == 2 {
                self.will_be_deselected = true;
                self.new_time_selection = 0;

                let nvm = crate::game::globals::get_nvm();
                nvm.settings.set_value(
                    SettingType::FeedingDeadlineHour,
                    self.hr_setting.get_value(),
                );
                nvm.settings.set_value(
                    SettingType::FeedingDeadlineMinute,
                    self.min_setting.get_value(),
                );
            }
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset_internal_state(&mut self) {
        *self = Self::default();
    }
}
