use core::cmp::min;

use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::render;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::hardware::hardware::LCD_WIDTH;

pub fn draw_top_bar() {
    draw_top_bar_bg();
    draw_top_bar_clock();

    if alert_is_necessary() {
        draw_top_bar_alert();
        draw_top_bar_inventory(10);
    } else {
        draw_top_bar_inventory(0);
    }
}

fn alert_is_necessary() -> bool {
    // TODO: Add conditions (sick, hungry, etc...)
    true
}

fn draw_top_bar_bg() {
    render::fill_rect(0, 0, LCD_WIDTH, 8, Rgb332::DARK_GREY);
}

fn draw_top_bar_alert() {
    text_writer::draw_text(0, 0, FontStyle::Icon, Rgb332::YELLOW, "!\"");
}

fn draw_top_bar_clock() {
    let hardware = crate::globals::get_hardware();
    let time = hardware.get_time();
    let time_str = time.hh_mm_str();
    let x = LCD_WIDTH as i32 - FontStyle::Small.get_glyph_dimensions().0 as i32 * (5 + 2);

    text_writer::draw_text(x, 0, FontStyle::Small, Rgb332::WHITE, &time_str);
}

fn draw_top_bar_inventory(offset: i32) {
    let nvm = crate::globals::get_nvm();
    let inventory = &nvm.inventory;

    let tomatoes = min(99, inventory.get_tomatoes());
    let raspberries = min(9, inventory.get_raspberries());
    let juice = min(9999, inventory.get_juice());

    // max values for testing
    // let tomatoes = 99;
    // let raspberries = 9;
    // let juice = 9999;

    let tomato_offset = if tomatoes > 9 { 5 } else { 0 };

    let tomato_icon = "tu";
    let x = offset;
    text_writer::draw_text(x, 0, FontStyle::Icon, Rgb332::RED, tomato_icon);
    let display_tomatoes = str_format!(fixedstr::str4, "{}", tomatoes);
    let x = 12 + offset;
    text_writer::draw_text(x, 0, FontStyle::Small, Rgb332::WHITE, &display_tomatoes);

    let rasp_icon = "rs";
    let x = 17 + offset + tomato_offset;
    text_writer::draw_text(x, 0, FontStyle::Icon, Rgb332::RED, rasp_icon);
    let display_raspberries = str_format!(fixedstr::str4, "{}", raspberries);
    let x = 28 + offset + tomato_offset;
    text_writer::draw_text(x, 0, FontStyle::Small, Rgb332::WHITE, &display_raspberries);

    let juice_icon = "w";
    let x = 35 + offset + tomato_offset;
    text_writer::draw_text(x, 0, FontStyle::Icon, Rgb332::RED, juice_icon);
    let display_juice = str_format!(fixedstr::str8, "{}ml", juice);
    let x = 43 + offset + tomato_offset;
    text_writer::draw_text(x, 0, FontStyle::Small, Rgb332::WHITE, &display_juice);
}
