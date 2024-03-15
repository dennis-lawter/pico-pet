use core::cmp::min;

use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::render;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::hardware::hardware::LCD_WIDTH;
use crate::scenes::main_scene::MainScene;
use crate::scenes::SceneType;

use super::scene_manager::SceneManager;

pub fn primary_main_loop() -> ! {
    let mut scene_manager = SceneManager::default();
    scene_manager.game_play_scene = Some(MainScene::default());
    scene_manager.active_scene = SceneType::Main;

    loop {
        let input = crate::globals::get_input();

        render::flood(Rgb332::DARKEST_BLUE); // Can't be black or the app crashes...
        input.update();
        draw_top_bar();
        scene_manager.update_and_draw();

        swap();

        scene_manager.advance_scene();
    }
}

fn draw_top_bar() {
    render::fill_rect(0, 0, LCD_WIDTH, 8, Rgb332::DARK_GREY);

    draw_top_bar_clock();

    draw_top_bar_alert_if_necessary();

    draw_top_bar_inventory(10);
}

fn draw_top_bar_alert_if_necessary() {
    // TODO: add alert conditions (feeding time, sick, etc...)
    text_writer::draw_text(0, 0, FontStyle::Icon, Rgb332::YELLOW, "!\"");
}

fn draw_top_bar_clock() {
    let hardware = crate::globals::get_hardware();
    let time = hardware.get_time();
    let meridian = if time.hr > 11 || time.hr == 0 {
        "PM"
    } else {
        "AM"
    };
    let time_str = str_format!(fixedstr::str8, "{:02}:{:02}{}", time.hr, time.min, meridian);
    let x = LCD_WIDTH as i32 - FontStyle::Small.get_glyph_dimensions().0 as i32 * (5 + 2);
    text_writer::draw_text(x, 0, FontStyle::Small, Rgb332::WHITE, time_str.as_str());
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

fn swap() {
    let hardware = crate::globals::get_hardware();
    hardware.set_backlight();
    crate::display::render::draw(&mut hardware.display);
}
