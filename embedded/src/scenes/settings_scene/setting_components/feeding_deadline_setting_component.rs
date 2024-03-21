use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::globals;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;
use crate::hardware::rtc::RealTime;

use super::SettingComponentTrait;

pub struct FeedingDeadlineSettingComponent {
    pub time: RealTime,
    pub new_time_selection: u8,
    pub will_be_deselected: bool,
}

impl Default for FeedingDeadlineSettingComponent {
    fn default() -> Self {
        let hr = unsafe { &globals::FEEDING_DEADLINE_HOUR_SETTING }.get_value();
        let min = unsafe { &globals::FEEDING_DEADLINE_MINUTE_SETTING }.get_value();
        Self {
            time: RealTime::new(hr, min, 0),
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
                        // self.time.hr,
                        self.time.min,
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
                        self.time.hr,
                        // self.time.min,
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
                        self.time.hr,
                        // self.time.min,
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
                        // self.time.hr,
                        self.time.min,
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
            let time_str =
                str_format!(fixedstr::str16, "{:02}:{:02}", self.time.hr, self.time.min,);
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
        let input = crate::globals::get_input();

        if input.get_state(&KeyNames::Back).just_released {
            match self.new_time_selection {
                0 => self.will_be_deselected = true,
                1 => self.new_time_selection = 0,
                _ => {}
            }
        } else if input.get_state(&KeyNames::Left).just_pressed {
            match self.new_time_selection {
                0 => {
                    if self.time.hr == 00 {
                        self.time.hr = 23;
                    } else {
                        self.time.hr -= 1;
                    }
                }
                1 => {
                    if self.time.min == 00 {
                        self.time.min = 59;
                    } else {
                        self.time.min -= 1;
                    }
                }
                _ => {}
            }
        } else if input.get_state(&KeyNames::Right).just_pressed {
            match self.new_time_selection {
                0 => {
                    if self.time.hr == 23 {
                        self.time.hr = 0;
                    } else {
                        self.time.hr += 1;
                    }
                }
                1 => {
                    if self.time.min == 59 {
                        self.time.min = 0;
                    } else {
                        self.time.min += 1;
                    }
                }
                _ => {}
            }
        } else if input.get_state(&KeyNames::Confirm).just_released {
            self.new_time_selection += 1;
            if self.new_time_selection == 2 {
                self.will_be_deselected = true;
                self.new_time_selection = 0;

                let setting_hr = unsafe { &mut globals::FEEDING_DEADLINE_HOUR_SETTING };
                let setting_min = unsafe { &mut globals::FEEDING_DEADLINE_MINUTE_SETTING };

                setting_hr.set_value(self.time.hr).unwrap();
                setting_min.set_value(self.time.min).unwrap();
            }
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset(&mut self) {
        let setting_hr = unsafe { &mut globals::FEEDING_DEADLINE_HOUR_SETTING }.get_value();
        let setting_min = unsafe { &mut globals::FEEDING_DEADLINE_MINUTE_SETTING }.get_value();
        self.will_be_deselected = false;
        self.time.hr = setting_hr;
        self.time.min = setting_min;
        self.new_time_selection = 0;
    }
}
