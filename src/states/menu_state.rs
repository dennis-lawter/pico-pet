use crate::{
    display::{
        render,
        sprite::{Sprite, SpriteFactory},
        text_writer::{self, FontStyle},
    },
    globals,
    setting_value::Setting,
    system::{Frequency, SystemComponents},
};

use super::{AppState, State};

const KEY_REPEAT_FRAMES: u8 = 5;

pub struct MenuState {
    frame_count: u32,
    key_repeat_slowdown_timer: u8,
    next_state: Option<AppState>,
    song: [Frequency; 396],
    current_frequency: Frequency,
}
impl State for MenuState {
    fn new() -> Self {
        // let mut song_str = "C4q D4q | F4q F4q F4q F4e F4e | F4e F4e F4q C4q D4q | F4q F4q F4q F4e F4e | F4e F4e F4q C4q D4q";
        let song = [
            Frequency::C4, // take
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::None,
            Frequency::C5, // me
            Frequency::C5,
            Frequency::C5,
            Frequency::None,
            // ===========
            Frequency::A4, // out
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::G4, // to
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            Frequency::F4, // the
            Frequency::F4,
            Frequency::F4,
            Frequency::None,
            // ===========
            Frequency::G4, // ball
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            // ===========
            Frequency::D4, // game
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::None,
            // ===========
            Frequency::C4, // take
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::None,
            Frequency::C5, // me
            Frequency::C5,
            Frequency::C5,
            Frequency::None,
            // ===========
            Frequency::A4, // out
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::G4, // to
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            Frequency::F4, // the
            Frequency::F4,
            Frequency::F4,
            Frequency::None,
            // ===========
            Frequency::G4, // crowd
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            // ===========
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            // ===========
            Frequency::A4, // buy
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::A4, // me
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::A4, // some
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            // ===========
            Frequency::E4, // hot
            Frequency::E4,
            Frequency::E4,
            Frequency::None,
            Frequency::F4, // dogs
            Frequency::F4,
            Frequency::F4,
            Frequency::None,
            Frequency::G4, // and
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            // ===========
            Frequency::A4, // crack-
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::F4, // er
            Frequency::F4,
            Frequency::F4,
            Frequency::None,
            // ===========
            Frequency::D4, // jacks
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::None,
            // ===========
            Frequency::A4, // I
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::A4, // don't
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            // ===========
            Frequency::A4, // care
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::B4, // if
            Frequency::B4,
            Frequency::B4,
            Frequency::None,
            Frequency::C5, // I
            Frequency::C5,
            Frequency::C5,
            Frequency::None,
            // ===========
            Frequency::D5, // ne-
            Frequency::D5,
            Frequency::D5,
            Frequency::None,
            Frequency::B4, // ver
            Frequency::B4,
            Frequency::B4,
            Frequency::None,
            Frequency::A4, // come
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            // ===========
            Frequency::G4, // back
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            Frequency::E4, // 'cause
            Frequency::E4,
            Frequency::E4,
            Frequency::None,
            Frequency::D4, // it's
            Frequency::D4,
            Frequency::D4,
            Frequency::None,
            // ===========
            Frequency::C4, // root
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::None,
            Frequency::C5, // root
            Frequency::C5,
            Frequency::C5,
            Frequency::None,
            // ===========
            Frequency::A4, // root
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::G4, // for
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            Frequency::F4, // the
            Frequency::F4,
            Frequency::F4,
            Frequency::None,
            // ===========
            Frequency::G4, // home
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            // ===========
            Frequency::D4, // team
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::D4,
            Frequency::None,
            Frequency::D4, // if
            Frequency::D4,
            Frequency::D4,
            Frequency::None,
            // ===========
            Frequency::C4, // they
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::C4,
            Frequency::None,
            Frequency::D4, // don't
            Frequency::D4,
            Frequency::D4,
            Frequency::None,
            // ===========
            Frequency::E4, // win
            Frequency::E4,
            Frequency::E4,
            Frequency::None,
            Frequency::F4, // it's
            Frequency::F4,
            Frequency::F4,
            Frequency::None,
            Frequency::G4, // a
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            // ===========
            Frequency::A4, // shame
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            // ===========
            Frequency::None, // {rest}
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::A4, // 'cause
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            Frequency::B4, // it's
            Frequency::B4,
            Frequency::B4,
            Frequency::None,
            // ===========
            Frequency::C5, // one
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::None,
            // ===========
            Frequency::C5, // two
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::None,
            // ===========
            Frequency::C5, // three
            Frequency::C5,
            Frequency::C5,
            Frequency::None,
            Frequency::B4, // strikes
            Frequency::B4,
            Frequency::B4,
            Frequency::None,
            Frequency::A4, // you're
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            // ===========
            Frequency::G4, // out
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            Frequency::Fs4, // at
            Frequency::Fs4,
            Frequency::Fs4,
            Frequency::None,
            Frequency::G4, // the
            Frequency::G4,
            Frequency::G4,
            Frequency::None,
            // ===========
            Frequency::A4, // old
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::A4,
            Frequency::None,
            // ===========
            Frequency::B4, // ball
            Frequency::B4,
            Frequency::B4,
            Frequency::B4,
            Frequency::B4,
            Frequency::B4,
            Frequency::B4,
            Frequency::B4,
            Frequency::B4,
            Frequency::B4,
            Frequency::B4,
            Frequency::None,
            // ===========
            Frequency::C5, // ga-
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            // ===========
            Frequency::C5, // -me
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::C5,
            Frequency::None,
            // ===========
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
            Frequency::None,
        ];
        Self {
            frame_count: 0,
            key_repeat_slowdown_timer: 0,
            next_state: None,
            song,
            current_frequency: Frequency::None,
        }
    }

    fn tick(&mut self, system: &mut SystemComponents) {
        self.frame_count += 1;
    }

    fn sound(&mut self, system: &mut SystemComponents) {
        let song_index = (self.frame_count / 2) as usize % self.song.len();
        let indexed_frequency = &self.song[song_index];
        if indexed_frequency != &self.current_frequency {
            system.start_tone(&self.song[song_index], 512);
            self.current_frequency = indexed_frequency.clone();
        }
    }

    fn draw(&mut self, system: &mut SystemComponents) {
        render::flood(0b000_000_00);

        let title = "BRIGHTNESS";
        let menu_body = "";
        text_writer::full_dialog_box(title, menu_body);
        text_writer::draw_text(
            24,
            18,
            FontStyle::Icon,
            0b000_000_11,
            unsafe { &globals::BRIGHTNESS_SETTING }.generate_bar(),
        );
    }

    fn swap(&mut self, system: &mut SystemComponents) {
        system.set_backlight(unsafe { &globals::BRIGHTNESS_SETTING });
        render::draw(&mut system.display);
    }

    fn input(&mut self, system: &mut SystemComponents) {
        if system.key0_pressed() {
            self.next_state = Some(AppState::GamePlay);
            return;
        }

        if system.key1_pressed() && !system.key2_pressed() {
            if self.key_repeat_slowdown_timer == 0 {
                self.key_repeat_slowdown_timer = KEY_REPEAT_FRAMES;
                unsafe { &mut globals::BRIGHTNESS_SETTING }.dec();
            } else {
                self.key_repeat_slowdown_timer -= 1;
            }
        } else if system.key2_pressed() && !system.key1_pressed() {
            if self.key_repeat_slowdown_timer == 0 {
                self.key_repeat_slowdown_timer = 5;
                unsafe { &mut globals::BRIGHTNESS_SETTING }.inc();
            } else {
                self.key_repeat_slowdown_timer -= 1;
            }
        } else {
            self.key_repeat_slowdown_timer = 0;
        }
    }

    fn next_state(&mut self) -> &Option<super::AppState> {
        &self.next_state
    }
}
