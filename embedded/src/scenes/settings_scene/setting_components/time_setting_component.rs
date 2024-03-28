use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;
use crate::hardware::rtc::RealTime;

use super::SettingComponentTrait;

pub struct TimeSettingComponent {
    pub time: Option<RealTime>,
    pub new_time: Option<RealTime>,
    pub new_time_selection: u8,
    pub will_be_deselected: bool,
}

impl Default for TimeSettingComponent {
    fn default() -> Self {
        Self {
            time: None,
            new_time: None,
            new_time_selection: 0,
            will_be_deselected: false,
        }
    }
}

impl SettingComponentTrait for TimeSettingComponent {
    fn draw(&mut self, y_offset: i32, selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "ADJUST TIME",
        );
        if selected {
            match &self.new_time {
                Some(time) => match self.new_time_selection {
                    0 => {
                        let time_str = str_format!(
                            fixedstr::str16,
                            "  :{:02}:{:02}",
                            // time.hr,
                            time.min,
                            time.sec
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
                            "{:02}      ",
                            time.hr,
                            // time.min,
                            // time.sec
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
                            "{:02}:  :{:02}",
                            time.hr,
                            // time.min,
                            time.sec
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
                            "   {:02}   ",
                            // time.hr,
                            time.min,
                            // time.sec
                        );
                        text_writer::draw_text_centered(
                            LCD_WIDTH as i32 / 2,
                            y_offset + 8 - 1,
                            FontStyle::Small,
                            Rgb332::BLUE,
                            active_time_str.as_str(),
                        );
                    }
                    2 => {
                        let time_str = str_format!(
                            fixedstr::str16,
                            "{:02}:{:02}:  ",
                            time.hr,
                            time.min,
                            // time.sec
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
                            "      {:02}",
                            // time.hr,
                            // time.min,
                            time.sec
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
                },
                None => {}
            }
        } else {
            match &self.time {
                Some(time) => {
                    let time_str = str_format!(
                        fixedstr::str16,
                        "{:02}:{:02}:{:02}",
                        time.hr,
                        time.min,
                        time.sec
                    );
                    text_writer::draw_text_centered(
                        LCD_WIDTH as i32 / 2,
                        y_offset + 8,
                        FontStyle::Small,
                        Rgb332::BLACK,
                        time_str.as_str(),
                    );
                }
                None => {}
            }
        };
    }

    fn tick(&mut self) {
        let hardware = crate::globals::get_hardware();
        self.time = Some(hardware.get_time());
    }

    fn input(&mut self) {
        let input = crate::globals::get_input();
        let hardware = crate::globals::get_hardware();

        if self.new_time.is_none() {
            self.new_time = self.time.clone();
            self.new_time_selection = 0;
        }
        let new_time_mut = self.new_time.as_mut().unwrap();

        if input.get_state(&KeyNames::Back).just_released {
            match self.new_time_selection {
                0 => self.will_be_deselected = true,
                1 => self.new_time_selection = 0,
                2 => self.new_time_selection = 1,
                _ => {}
            }
        } else if input.get_state(&KeyNames::Left).just_pressed {
            match self.new_time_selection {
                0 => {
                    if new_time_mut.hr == 00 {
                        new_time_mut.hr = 23;
                    } else {
                        new_time_mut.hr -= 1;
                    }
                }
                1 => {
                    if new_time_mut.min == 00 {
                        new_time_mut.min = 59;
                    } else {
                        new_time_mut.min -= 1;
                    }
                }
                2 => {
                    if new_time_mut.sec == 00 {
                        new_time_mut.sec = 59;
                    } else {
                        new_time_mut.sec -= 1;
                    }
                }
                _ => {}
            }
        } else if input.get_state(&KeyNames::Right).just_pressed {
            match self.new_time_selection {
                0 => {
                    if new_time_mut.hr == 23 {
                        new_time_mut.hr = 0;
                    } else {
                        new_time_mut.hr += 1;
                    }
                }
                1 => {
                    if new_time_mut.min == 59 {
                        new_time_mut.min = 0;
                    } else {
                        new_time_mut.min += 1;
                    }
                }
                2 => {
                    if new_time_mut.sec == 59 {
                        new_time_mut.sec = 0;
                    } else {
                        new_time_mut.sec += 1;
                    }
                }
                _ => {}
            }
        } else if input.get_state(&KeyNames::Confirm).just_released {
            self.new_time_selection += 1;
            if self.new_time_selection == 3 {
                self.will_be_deselected = true;
                self.new_time_selection = 0;
                hardware.set_time(self.new_time.as_mut().unwrap());
                self.new_time = None;
            }
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset_internal_state(&mut self) {
        self.will_be_deselected = false;
        self.time = None;
        self.new_time = None;
        self.new_time_selection = 0;
    }
}
