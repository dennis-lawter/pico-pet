use fixedstr::str_format;

use crate::game::color::Rgb332;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::hardware::rtc::month::Month;
use crate::game::hardware::rtc::RealDate;

use super::SettingComponentTrait;

pub struct DateSettingComponent {
    pub date: Option<RealDate>,
    pub new_date: Option<RealDate>,
    pub new_date_selection: u8,
    pub will_be_deselected: bool,
}

impl Default for DateSettingComponent {
    fn default() -> Self {
        Self {
            date: None,
            new_date: None,
            new_date_selection: 0,
            will_be_deselected: false,
        }
    }
}

impl SettingComponentTrait for DateSettingComponent {
    fn draw(&mut self, y_offset: i32, selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "ADJUST DATE",
        );
        if selected {
            match &self.new_date {
                Some(date) => match self.new_date_selection {
                    0 => {
                        let time_str = str_format!(
                            fixedstr::str16,
                            "     {} {:02}",
                            // time.year_since_2k + RealDate::ZERO_YEAR,
                            Month::from(date.month).to_abbrev(),
                            date.day_of_month,
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
                            "{}       ",
                            date.year_since_2k as u16 + RealDate::ZERO_YEAR,
                            // Month::from(date.month).to_abbrev(),
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
                            "{}     {:02}",
                            date.year_since_2k as u16 + RealDate::ZERO_YEAR,
                            // Month::from(date.month).to_abbrev(),
                            date.day_of_month,
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
                            "     {}   ",
                            // time.year_since_2k + RealDate::ZERO_YEAR,
                            Month::from(date.month).to_abbrev(),
                            // date.day_of_month,
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
                            "{} {}   ",
                            date.year_since_2k as u16 + RealDate::ZERO_YEAR,
                            Month::from(date.month).to_abbrev(),
                            // date.day_of_month,
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
                            "         {:02}",
                            // time.year_since_2k + RealDate::ZERO_YEAR,
                            // time.min,
                            date.day_of_month,
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
            match &self.date {
                Some(date) => {
                    let date_str = date.yyyy_mmm_dd_str();
                    text_writer::draw_text_centered(
                        LCD_WIDTH as i32 / 2,
                        y_offset + 8,
                        FontStyle::Small,
                        Rgb332::BLACK,
                        date_str.as_str(),
                    );
                }
                None => {}
            }
        };
    }

    fn tick(&mut self) {
        let hardware = crate::game::globals::get_hardware();
        // let input = crate::game::globals::get_input();
        // if input.get_state(&KeyNames::Clock).just_pressed {
        self.date = Some(hardware.get_date());
        // }
    }

    fn input(&mut self) {
        let input = crate::game::globals::get_input();
        let hardware = crate::game::globals::get_hardware();

        if self.new_date.is_none() {
            self.new_date = self.date.clone();
            self.new_date_selection = 0;
        }
        let new_date_mut = self.new_date.as_mut().unwrap();

        if input.get_state(&KeyNames::Back).just_released {
            match self.new_date_selection {
                0 => self.will_be_deselected = true,
                1 => self.new_date_selection = 0,
                2 => self.new_date_selection = 1,
                _ => {}
            }
        } else if input.get_state(&KeyNames::Left).just_pressed {
            // Avoid underflow
            if new_date_mut.year_since_2k == 0
                && new_date_mut.month == 1
                && new_date_mut.day_of_month == 1
            {
                // TODO: beep or something to indicate that we are already at the lowest possible date
            } else {
                match self.new_date_selection {
                    0 => new_date_mut.dec_by_1_year(),
                    1 => new_date_mut.dec_by_1_month(),
                    2 => new_date_mut.dec_by_1_day(),
                    _ => {}
                }
            }
        } else if input.get_state(&KeyNames::Right).just_pressed {
            match self.new_date_selection {
                0 => new_date_mut.inc_by_1_year(),
                1 => new_date_mut.inc_by_1_month(),
                2 => new_date_mut.inc_by_1_day(),
                _ => {}
            }
        } else if input.get_state(&KeyNames::Confirm).just_released {
            self.new_date_selection += 1;
            if self.new_date_selection == 3 {
                // Finalize date
                // TODO: Adjust feeding deadlines
                // TODO: Show a confirmation dialog that feeding deadlines will be affected
                self.will_be_deselected = true;
                self.new_date_selection = 0;
                hardware.set_date(self.new_date.as_mut().unwrap());
                self.new_date = None;
            }
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset_internal_state(&mut self) {
        self.will_be_deselected = false;
        self.date = None;
        self.new_date = None;
        self.new_date_selection = 0;
    }
}
