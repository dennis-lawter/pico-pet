use crate::color::Rgb332;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::globals;
use crate::hardware::hardware::LCD_WIDTH;

use super::SettingComponentTrait;

pub struct BrightnessSettingComponent {}
impl SettingComponentTrait for BrightnessSettingComponent {
    fn draw(&mut self, y_offset: i32, selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "BRIGHTNESS",
        );
        text_writer::draw_text(
            24,
            y_offset + 8,
            FontStyle::Icon,
            Rgb332::BLUE,
            unsafe { &globals::BRIGHTNESS_SETTING }.generate_bar(selected),
        );
    }

    fn input(&mut self) {
        todo!()
    }
}
