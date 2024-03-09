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

        render::flood(Rgb332::DARKEST_BLUE);
        input.update();
        draw_top_bar();
        scene_manager.update_and_draw();

        swap();

        scene_manager.advance_scene();
    }
}

fn draw_top_bar() {
    render::fill_rect(0, 0, 128, 8, Rgb332::DARK_GREY);
    let hardware = crate::globals::get_hardware();

    let time = hardware.get_time();
    let time_str = str_format!(fixedstr::str8, "{:02}:{:02}", time.hr, time.min);
    let x = LCD_WIDTH as i32 - FontStyle::Small.get_glyph_dimensions().0 as i32 * 5;
    text_writer::draw_text(x, 0, FontStyle::Small, Rgb332::WHITE, time_str.as_str());

    let food_icon_str = "tu vvv rs vvv";
    text_writer::draw_text(0, 0, FontStyle::Icon, Rgb332::WHITE, food_icon_str);
    let inventory = crate::globals::get_inventory();
    let tomatoes = inventory.tomatoes;
    let raspberries = inventory.raspberries;
    let inventory_str = str_format!(fixedstr::str16, "   {:<3}    {:<3}", tomatoes, raspberries);
    text_writer::draw_text(
        0,
        0,
        FontStyle::Small,
        Rgb332::WHITE,
        inventory_str.as_str(),
    );
}

fn swap() {
    let hardware = crate::globals::get_hardware();
    hardware.set_backlight();
    crate::display::render::draw(&mut hardware.display);
}
