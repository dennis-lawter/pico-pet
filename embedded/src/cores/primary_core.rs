use crate::color::Rgb332;
use crate::display::render;
use crate::display_helper::top_bar::draw_top_bar;
use crate::scenes::main_scene::MainScene;
use crate::scenes::SceneType;

use super::scene_manager::SceneManager;

pub fn primary_main_loop() -> ! {
    let mut scene_manager = SceneManager::default();
    scene_manager.game_play_scene = Some(MainScene::default());
    scene_manager.active_scene = SceneType::Main;

    loop {
        let input = crate::globals::get_input();

        // TODO: Figure out why black crashes the app
        render::flood(Rgb332::DARKEST_BLUE);
        input.update();

        draw_top_bar();
        scene_manager.update_and_draw();

        swap();

        scene_manager.advance_scene();
    }
}

fn swap() {
    let hardware = crate::globals::get_hardware();
    hardware.set_backlight();
    crate::display::render::draw(&mut hardware.display);
}
