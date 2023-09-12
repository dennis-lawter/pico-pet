mod setting_selected;
mod song;

use fixedstr::str_format;

use crate::{
    display::{
        render,
        text_writer::{self, FontStyle},
    },
    globals,
    setting_value::Setting,
    system::{Frequency, RealTime, SystemComponents, LCD_WIDTH},
};

use self::setting_selected::SettingSelected;

use super::{AppState, State};

const KEY_REPEAT_FRAMES: u8 = 5;

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
    new_time: Option<RealTime>,
    new_time_selection: u8,
}
impl State for SettingsState {
    fn new() -> Self {
        Self {
            frame_count: 0,
            key_repeat_slowdown_timer: 0,
            next_state: None,
            song: song::SONG,
            current_frequency: Frequency::None,
            key0_down: false,
            key1_down: false,
            key2_down: false,
            key3_down: false,
            setting_selected: SettingSelected::None,
            setting_highlighted: SettingSelected::None,
            input_enabled: false,
            time: None,
            new_time: None,
            new_time_selection: 0,
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
            SettingSelected::Time => {
                text_writer::draw_text(10, 18 + 8 * 5, FontStyle::Icon, 0b111_000_00, "4}");
            }
            SettingSelected::None => match self.setting_highlighted {
                SettingSelected::Brightness => {
                    text_writer::draw_text(10, 18 + 8, FontStyle::Icon, 0b111_000_00, " >");
                }
                SettingSelected::Volume => {
                    text_writer::draw_text(10, 18 + 8 * 3, FontStyle::Icon, 0b111_000_00, " >");
                }
                SettingSelected::Time => {
                    text_writer::draw_text(10, 18 + 8 * 5, FontStyle::Icon, 0b111_000_00, " >");
                }
                SettingSelected::None => {}
            },
        }

        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + 8 * 4,
            FontStyle::Small,
            0b000_000_00,
            "ADJUST TIME",
        );
        if self.setting_selected == SettingSelected::Time {
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
                            18 + 8 * 5,
                            FontStyle::Small,
                            0b000_000_00,
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
                            18 + 8 * 5 - 1,
                            FontStyle::Small,
                            0b000_000_11,
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
                            18 + 8 * 5,
                            FontStyle::Small,
                            0b000_000_00,
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
                            18 + 8 * 5 - 1,
                            FontStyle::Small,
                            0b000_000_11,
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
                            18 + 8 * 5,
                            FontStyle::Small,
                            0b000_000_00,
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
                            18 + 8 * 5 - 1,
                            FontStyle::Small,
                            0b000_000_11,
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
                        18 + 8 * 5,
                        FontStyle::Small,
                        0b000_000_00,
                        time_str.as_str(),
                    );
                }
                None => {}
            }
        };
        // match shown_time {
        //     Some(time) => {
        //         let time_str = str_format!(
        //             fixedstr::str16,
        //             "{:02}:{:02}:{:02}",
        //             time.hr,
        //             time.min,
        //             time.sec
        //         );
        //         text_writer::draw_text_centered(
        //             LCD_WIDTH as i32 / 2,
        //             18 + 8 * 5,
        //             FontStyle::Small,
        //             0b000_000_00,
        //             time_str.as_str(),
        //         );
        //     }
        //     None => {}
        // }
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

        self.check_for_setting_deselected(system);
        match self.setting_selected {
            SettingSelected::Brightness => {
                self.adjust_setting(system, unsafe { &mut globals::BRIGHTNESS_SETTING })
            }
            SettingSelected::Volume => {
                self.adjust_setting(system, unsafe { &mut globals::VOLUME_SETTING })
            }
            SettingSelected::Time => {
                if self.new_time.is_none() {
                    self.new_time = self.time.clone();
                    self.new_time_selection = 0;
                }
                let new_time_mut = self.new_time.as_mut().unwrap();

                if !self.key0_down && !self.key1_down && !self.key2_down && !self.key3_down {
                    if system.key1_pressed() {
                        match self.new_time_selection {
                            0 => {
                                if new_time_mut.hr == 00 {
                                    new_time_mut.hr = 23;
                                } else {
                                    new_time_mut.hr -= 1;
                                }
                            }
                            1 => {
                                if new_time_mut.min == 00 {
                                    new_time_mut.min = 59;
                                } else {
                                    new_time_mut.min -= 1;
                                }
                            }
                            2 => {
                                if new_time_mut.sec == 00 {
                                    new_time_mut.sec = 59;
                                } else {
                                    new_time_mut.sec -= 1;
                                }
                            }
                            _ => {}
                        }
                    } else if system.key2_pressed() {
                        match self.new_time_selection {
                            0 => {
                                if new_time_mut.hr == 23 {
                                    new_time_mut.hr = 0;
                                } else {
                                    new_time_mut.hr += 1;
                                }
                            }
                            1 => {
                                if new_time_mut.min == 59 {
                                    new_time_mut.min = 0;
                                } else {
                                    new_time_mut.min += 1;
                                }
                            }
                            2 => {
                                if new_time_mut.sec == 59 {
                                    new_time_mut.sec = 0;
                                } else {
                                    new_time_mut.sec += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                } else if !self.key0_down && !self.key1_down && !self.key2_down && self.key3_down {
                    if !system.key3_pressed() {
                        self.new_time_selection += 1;
                        if self.new_time_selection == 3 {
                            self.setting_selected = SettingSelected::None;
                            self.new_time_selection = 0;
                            system.set_time(self.new_time.as_mut().unwrap());
                            self.new_time = None;
                        }
                    }
                }
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
            self.new_time = None;
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
        if self.key2_down && !system.key2_pressed() {
            self.setting_highlighted = self.setting_highlighted.next().clone();
        }
        if self.key1_down && !system.key1_pressed() {
            self.setting_highlighted = self.setting_highlighted.prev().clone();
        }
    }
}
