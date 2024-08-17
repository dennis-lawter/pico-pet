use fixedstr::str_format;

use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::display_helper::top_bar::draw_top_bar;
use crate::game::hardware::hardware::BRIGHTNESS_LUT;
use crate::game::hardware::hardware::LCD_HEIGHT;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::hardware::rtc::meridian::Meridian;
use crate::game::scenes::main_scene::MainScene;
use crate::game::scenes::SceneType;

use super::scene_manager::SceneManager;

const APPROX_FRAMES_PER_SECOND: usize = 21;
const SECONDS_UNTIL_IDLE: usize = 60 * APPROX_FRAMES_PER_SECOND;

pub fn primary_main_loop() -> ! {
    let mut scene_manager = SceneManager::default();
    scene_manager.game_play_scene = Some(MainScene::default());
    scene_manager.active_scene = SceneType::Main;

    let mut idle_frame_counter: usize = 0;

    // Draw the first frame to initialize the screen
    // Black fails to initialize the display
    render::flood(Rgb332::DARKEST_BLUE);
    swap();

    loop {
        check_feeding_deadline_is_passed();
        // Disable the timer during the pomo scene
        match scene_manager.active_scene {
            SceneType::Pomo => {
                idle_frame_counter = 0;
            }
            _ => {}
        }

        let input = crate::game::globals::get_input();
        let hardware = crate::game::globals::get_hardware();

        let any_key_pressed = hardware.key0_pressed()
            || hardware.key1_pressed()
            || hardware.key2_pressed()
            || hardware.key3_pressed();
        if idle_frame_counter >= SECONDS_UNTIL_IDLE {
            input.force_reset();
            if any_key_pressed {
                // next frame will return from idle state
                idle_frame_counter = 0;
                // wait for the user to release all keys
                while hardware.key0_pressed()
                    || hardware.key1_pressed()
                    || hardware.key2_pressed()
                    || hardware.key3_pressed()
                {}
            }
            let lowest_brightness = BRIGHTNESS_LUT[0];
            // Can't draw a fully black frame
            render::flood(Rgb332::DARKEST_BLUE);

            let x = LCD_WIDTH as i32 / 2;
            let y = LCD_HEIGHT as i32 / 2 - FontStyle::Big.get_glyph_dimensions().1 as i32 / 2;
            let time = hardware.get_time();

            // TODO: fix bug
            // I dont know why but I cant use any of the realtime helpers here...
            // Maybe they are not initialized yet?
            // But it works in the top_bar...
            // And it still works with the raw members...

            // let mut time_str = time.hh_mm_str();

            let time_str = {
                let hr = time.get_meridian_hour();
                let meridian = time.get_meridian();
                let meridian_str = match meridian {
                    Meridian::Am => "AM",
                    Meridian::Pm => "PM",
                };
                str_format!(fixedstr::str8, "{}:{:02}{}", hr, time.min, meridian_str)
            };

            text_writer::draw_text_centered(
                x,
                y,
                text_writer::FontStyle::Big,
                Rgb332::WHITE,
                time_str.as_str(),
            );
            hardware.set_backlight_raw(lowest_brightness);
            hardware.end_tone();
            crate::game::display::render::draw(&mut hardware.display);

            // Would be nice to have a delay,
            // especially in a low power mode,
            // but it requires interrupts so user input immediately ends the sleep.

            // hardware.delay.delay_ms(1000);
            continue;
        } else if any_key_pressed {
            idle_frame_counter = 0;
        } else {
            idle_frame_counter += 1;
        }

        input.update();

        render::flood(Rgb332::BLACK);

        draw_top_bar();
        scene_manager.update_and_draw();

        swap();

        scene_manager.advance_scene();
    }
}

fn check_feeding_deadline_is_passed() -> () {
    let input = crate::game::globals::get_input();
    if !input.get_state(&KeyNames::Clock).just_released {
        // limit checks to 1hz to save CPU cycles
        return;
    }
    let nvm = crate::game::globals::get_nvm();
    // DEBUG: force hungry
    // nvm.pet.is_hungry = true;
    let now = crate::game::globals::get_hardware().get_date_time();
    let feeding_deadline = nvm.pet.get_feeding_deadline();

    if now > feeding_deadline {
        nvm.pet.is_hungry = true;
    }
}

fn swap() {
    let hardware = crate::game::globals::get_hardware();
    hardware.set_backlight_from_lut();
    crate::game::display::render::draw(&mut hardware.display);
}
