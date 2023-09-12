use fixedstr::str_format;

use crate::{
    display::{
        render,
        text_writer::{self, FontStyle},
    },
    globals,
    setting_value::Setting,
    system::{Frequency, RealTime, SystemComponents, LCD_HEIGHT, LCD_WIDTH},
};

use super::{AppState, State};

const KEY_REPEAT_FRAMES: u8 = 5;

#[derive(Clone, PartialEq)]
enum SettingSelected {
    Brightness,
    Volume,
    None,
}
impl SettingSelected {
    fn prev(&self) -> SettingSelected {
        match self {
            SettingSelected::Brightness => SettingSelected::Volume,
            SettingSelected::Volume => SettingSelected::Brightness,
            SettingSelected::None => SettingSelected::Volume,
        }
    }
    fn next(&self) -> SettingSelected {
        match self {
            SettingSelected::Brightness => SettingSelected::Volume,
            SettingSelected::Volume => SettingSelected::Brightness,
            SettingSelected::None => SettingSelected::Brightness,
        }
    }
}

pub struct SettingsState {
    frame_count: u32,
    key_repeat_slowdown_timer: u8,
    next_state: Option<AppState>,
    song: [Frequency; 396],
    current_frequency: Frequency,
    key0_down: bool,
    key1_down: bool,
    key2_down: bool,
    key3_down: bool,
    setting_selected: SettingSelected,
    setting_highlighted: SettingSelected,
    input_enabled: bool,
    time: Option<RealTime>,
}
impl State for SettingsState {
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
            key0_down: false,
            key1_down: false,
            key2_down: false,
            key3_down: false,
            setting_selected: SettingSelected::None,
            setting_highlighted: SettingSelected::None,
            input_enabled: false,
            time: None,
        }
    }

    fn tick(&mut self, system: &mut SystemComponents) {
        // TODO: needs to be cached!!
        self.time = Some(system.get_time());

        self.frame_count += 1;
    }

    fn sound(&mut self, system: &mut SystemComponents) {
        let song_index = (self.frame_count / 2) as usize % self.song.len();
        let indexed_frequency = &self.song[song_index];
        if indexed_frequency != &self.current_frequency {
            system.start_tone(&self.song[song_index]);
            self.current_frequency = indexed_frequency.clone();
        }
    }

    fn draw(&mut self, _system: &mut SystemComponents) {
        render::flood(0b000_000_00);

        let title = "SETTINGS";
        let menu_body = "";
        text_writer::full_dialog_box(title, menu_body);

        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18,
            FontStyle::Small,
            0b000_000_00,
            "BRIGHTNESS",
        );
        text_writer::draw_text(
            24,
            18 + 8,
            FontStyle::Icon,
            0b000_000_11,
            unsafe { &globals::BRIGHTNESS_SETTING }
                .generate_bar(self.setting_selected == SettingSelected::Brightness),
        );

        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + 8 * 2,
            FontStyle::Small,
            0b000_000_00,
            "VOLUME",
        );
        text_writer::draw_text(
            24,
            18 + 8 * 3,
            FontStyle::Icon,
            0b000_000_11,
            unsafe { &globals::VOLUME_SETTING }
                .generate_bar(self.setting_selected == SettingSelected::Volume),
        );

        match self.setting_selected {
            SettingSelected::Brightness => {
                text_writer::draw_text(10, 18 + 8, FontStyle::Icon, 0b111_000_00, "4}");
            }
            SettingSelected::Volume => {
                text_writer::draw_text(10, 18 + 8 * 3, FontStyle::Icon, 0b111_000_00, "4}");
            }
            SettingSelected::None => match self.setting_highlighted {
                SettingSelected::Brightness => {
                    text_writer::draw_text(10, 18 + 8, FontStyle::Icon, 0b111_000_00, " >");
                }
                SettingSelected::Volume => {
                    text_writer::draw_text(10, 18 + 8 * 3, FontStyle::Icon, 0b111_000_00, " >");
                }
                SettingSelected::None => {}
            },
        }

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
                    LCD_HEIGHT as i32 - 16,
                    FontStyle::Small,
                    0b000_000_00,
                    time_str.as_str(),
                );
            }
            None => {}
        }
    }

    fn swap(&mut self, system: &mut SystemComponents) {
        system.set_backlight();
        render::draw(&mut system.display);
    }

    fn input(&mut self, system: &mut SystemComponents) {
        if !self.input_enabled {
            // release all buttons to enable input
            if system.key0_pressed()
                || system.key1_pressed()
                || system.key2_pressed()
                || system.key3_pressed()
            {
                return;
            } else {
                self.input_enabled = true;
            }
        }
        match self.setting_selected {
            SettingSelected::Brightness => {
                self.check_for_setting_deselected(system);
                self.adjust_setting(system, unsafe { &mut globals::BRIGHTNESS_SETTING })
            }
            SettingSelected::Volume => {
                self.check_for_setting_deselected(system);
                self.adjust_setting(system, unsafe { &mut globals::VOLUME_SETTING })
            }
            SettingSelected::None => {
                if system.key0_pressed() {
                    self.next_state = Some(AppState::GamePlay);
                    return;
                }
                self.check_for_setting_selected(system);
                self.check_for_move_highlight(system);
            }
        }

        self.key0_down = system.key0_pressed();
        self.key1_down = system.key1_pressed();
        self.key2_down = system.key2_pressed();
        self.key3_down = system.key3_pressed();
    }

    fn next_state(&self) -> &Option<super::AppState> {
        &self.next_state
    }
}

impl SettingsState {
    fn adjust_setting(&mut self, system: &mut SystemComponents, setting: &mut Setting) {
        if system.key1_pressed() && !system.key2_pressed() {
            if self.key_repeat_slowdown_timer == 0 {
                self.key_repeat_slowdown_timer = KEY_REPEAT_FRAMES;
                setting.dec();
            } else {
                self.key_repeat_slowdown_timer -= 1;
            }
        } else if system.key2_pressed() && !system.key1_pressed() {
            if self.key_repeat_slowdown_timer == 0 {
                self.key_repeat_slowdown_timer = 5;
                setting.inc();
            } else {
                self.key_repeat_slowdown_timer -= 1;
            }
        } else {
            self.key_repeat_slowdown_timer = 0;
        }
    }

    fn check_for_setting_selected(&mut self, system: &mut SystemComponents) {
        if self.setting_selected != SettingSelected::None {
            return;
        }
        if self.key3_down && !system.key3_pressed() {
            self.setting_selected = self.setting_highlighted.clone();
        }
    }

    fn check_for_setting_deselected(&mut self, system: &mut SystemComponents) {
        if self.setting_selected == SettingSelected::None {
            return;
        }
        if self.key0_down && !system.key0_pressed() {
            self.setting_selected = SettingSelected::None;
        }
    }

    fn check_for_move_highlight(&mut self, system: &mut SystemComponents) {
        if self.setting_selected != SettingSelected::None {
            return;
        }
        if system.key2_pressed() && system.key3_pressed() {
            return;
        }
        if system.key1_pressed() && system.key2_pressed() {
            return;
        }
        if self.key1_down && !system.key1_pressed() {
            self.setting_highlighted = self.setting_highlighted.next().clone();
        }
        if self.key2_down && !system.key2_pressed() {
            self.setting_highlighted = self.setting_highlighted.prev().clone();
        }
    }
}
