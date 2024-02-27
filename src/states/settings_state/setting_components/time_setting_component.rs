use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::rtc::RealTime;

use super::SettingComponentTrait;

pub struct TimeSettingComponent {
    pub time: Option<RealTime>,
    pub new_time: Option<RealTime>,
    pub new_time_selection: u8,
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

    fn input(&mut self) {
        todo!()
    }
}
