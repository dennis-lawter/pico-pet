use crate::{
    display::{
        render,
        sprite::SpriteFactory,
        text_writer::{self, FontStyle},
    },
    setting_value::Setting,
    system::{Frequency, System},
};

pub fn primary_main_loop(system: &mut System) -> ! {
    let mut ferris = SpriteFactory::new_ferris_sprite();
    ferris.x = 32;
    ferris.y = 32;
    let mut corro = SpriteFactory::new_corro_sprite();
    corro.x = 64;
    corro.y = 64;
    let mut frame_count = 0;

    let mut brightness = Setting::new_max();

    // clear the LCD
    render::flood(0b000_000_00);
    render::draw(&mut system.display);
    let mut key_repeat_slowdown_timer = 0;
    // let mut playing_sound = false;
    // let mut sound_playing_timer = 0;

    let note_sequence = [
        Frequency::C4,
        // Frequency::Cs4,
        Frequency::D4,
        // Frequency::Ds4,
        Frequency::E4,
        Frequency::F4,
        // Frequency::Fs4,
        Frequency::G4,
        // Frequency::Gs4,
        Frequency::A4,
        // Frequency::As4,
        Frequency::B4,
        Frequency::C5,
        // Frequency::Cs5,
        Frequency::D5,
        // Frequency::Ds5,
        Frequency::E5,
        Frequency::F5,
        // Frequency::Fs5,
        Frequency::G5,
        // Frequency::Gs5,
        Frequency::A5,
        // Frequency::As5,
        Frequency::B5,
        Frequency::C6,
        // Frequency::Cs6,
        Frequency::D6,
        // Frequency::Ds6,
        Frequency::E6,
        Frequency::F6,
        // Frequency::Fs6,
        Frequency::G6,
        // Frequency::Gs6,
        Frequency::A6,
        // Frequency::As6,
        Frequency::B6,
        Frequency::C7,
    ];

    let mut in_menu = false;
    loop {
        if frame_count % 20 == 19 {
            if frame_count / 20 < note_sequence.len() {
                let freq_index = frame_count / 20;
                system.start_tone(&note_sequence[freq_index], 512);
            } else {
                system.end_tone();
            }
        }

        frame_count += 1;
        render::flood(0b000_000_00);

        match in_menu {
            true => {
                // if playing_sound {
                //     system.end_tone();
                //     playing_sound = false;
                // }
                let title = "BRIGHTNESS";
                let menu_body = "";
                text_writer::full_dialog_box(title, menu_body);
                text_writer::draw_text(
                    24,
                    18,
                    FontStyle::Icon,
                    0b000_000_11,
                    brightness.generate_bar(),
                );
            }
            false => {
                corro.draw(0);
                ferris.draw((frame_count / 20) % 2);

                // cute chittering thanks to typo

                // if playing_sound == false && (frame_count / 20) % 2 == 1 {
                //     // if playing_sound == false && frame_count % 20 == 1 {
                //     playing_sound = true;
                //     if frame_count % 4 == 0 {
                //         system.start_tone(&system::Frequency::C4, 512);
                //     } else {
                //         system.start_tone(&system::Frequency::A4, 512);
                //     }
                // } else if playing_sound {
                //     system.end_tone();
                //     playing_sound = false;
                // }

                // single tone example instead of the chittering I accidentally made

                // if playing_sound == false && frame_count % 20 == 0 {
                //     playing_sound = true;
                //     system.start_tone(&system::Frequency::A4, 512);
                // } else if playing_sound && frame_count % 20 == 0 {
                //     system.end_tone();
                //     playing_sound = false;
                // }
                let text = "DIALOG\\b700!\\b703 so \\c700smol\\c003\\\\ so cute";
                text_writer::bottom_dialog_box(text);
            }
        }

        system.set_backlight(&brightness);
        render::draw(&mut system.display);

        match in_menu {
            true => {
                if system.key0_pressed() {
                    in_menu = false;
                }
                if system.key1_pressed() && !system.key2_pressed() {
                    if key_repeat_slowdown_timer == 0 {
                        key_repeat_slowdown_timer = 5;
                        brightness.dec();
                    } else {
                        key_repeat_slowdown_timer -= 1;
                    }
                } else if system.key2_pressed() && !system.key1_pressed() {
                    if key_repeat_slowdown_timer == 0 {
                        key_repeat_slowdown_timer = 5;
                        brightness.inc();
                    } else {
                        key_repeat_slowdown_timer -= 1;
                    }
                } else {
                    key_repeat_slowdown_timer = 0;
                }
            }
            false => {
                if system.key2_pressed() && system.key3_pressed() {
                    in_menu = true;
                } else {
                    if system.key0_pressed() {
                        ferris.x -= 1;
                    }
                    if system.key1_pressed() {
                        ferris.y += 1;
                    }
                    if system.key2_pressed() {
                        ferris.y -= 1;
                    }
                    if system.key3_pressed() {
                        ferris.x += 1;
                    }
                }
            }
        }
    }
}
