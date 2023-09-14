use fixedstr::str_format;

use crate::hardware::audio::AudioFrequency;
use crate::hardware::input::KeyNames;
use crate::hardware::rtc::RealTime;
use crate::states::{AppState, State};
use crate::{
    display::{
        render,
        text_writer::{self, FontStyle},
    },
    globals,
    hardware::hardware::LCD_WIDTH,
    setting_value::Setting,
};

use super::setting_selected::SettingSelected;
use super::song;

pub struct SettingsState {
    frame_count: u32,
    next_state: Option<AppState>,
    song: [AudioFrequency; 396],
    current_frequency: AudioFrequency,
    setting_selected: SettingSelected,
    setting_highlighted: SettingSelected,
    input_enabled: bool,
    time: Option<RealTime>,
    new_time: Option<RealTime>,
    new_time_selection: u8,
}
impl State for SettingsState {
    fn tick(&mut self) {
        let hardware = crate::globals::get_hardware();
        self.time = Some(hardware.get_time());

        self.frame_count += 1;
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        let song_index = (self.frame_count / 2) as usize % self.song.len();
        let indexed_frequency = &self.song[song_index];
        if indexed_frequency != &self.current_frequency {
            hardware.start_tone(&self.song[song_index]);
            self.current_frequency = indexed_frequency.clone();
        }
    }

    fn draw(&mut self) {
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
    }

    fn input(&mut self) {
        let input = crate::globals::get_input();
        let hardware = crate::globals::get_hardware();

        if !self.input_enabled {
            // release all buttons to enable input
            if input.get_state(&KeyNames::Back).is_down
                || input.get_state(&KeyNames::Left).is_down
                || input.get_state(&KeyNames::Right).is_down
                || input.get_state(&KeyNames::Confirm).is_down
            {
                return;
            } else {
                self.input_enabled = true;
            }
        }

        if self.check_for_setting_deselected() {
            return; // our action this frame will be to deselect, prevents auto-exit menu
        }

        match self.setting_selected {
            SettingSelected::Brightness => {
                self.adjust_setting(unsafe { &mut globals::BRIGHTNESS_SETTING })
            }
            SettingSelected::Volume => self.adjust_setting(unsafe { &mut globals::VOLUME_SETTING }),
            SettingSelected::Time => {
                if self.new_time.is_none() {
                    self.new_time = self.time.clone();
                    self.new_time_selection = 0;
                }
                let new_time_mut = self.new_time.as_mut().unwrap();

                if input.get_state(&KeyNames::Left).just_pressed {
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
                } else if input.get_state(&KeyNames::Right).just_pressed {
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
                } else if input.get_state(&KeyNames::Confirm).just_released {
                    self.new_time_selection += 1;
                    if self.new_time_selection == 3 {
                        self.setting_selected = SettingSelected::None;
                        self.new_time_selection = 0;
                        hardware.set_time(self.new_time.as_mut().unwrap());
                        self.new_time = None;
                    }
                }
            }
            SettingSelected::None => {
                if input.get_state(&KeyNames::Back).just_released {
                    self.next_state = Some(AppState::GamePlay);
                    return;
                }
                self.check_for_setting_selected();
                self.check_for_move_highlight();
            }
        }
    }

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            next_state: None,
            song: song::BALL_GAME,
            current_frequency: AudioFrequency::None,
            setting_selected: SettingSelected::None,
            setting_highlighted: SettingSelected::None,
            input_enabled: false,
            time: None,
            new_time: None,
            new_time_selection: 0,
        }
    }

    fn adjust_setting(&mut self, setting: &mut Setting) {
        let input = crate::globals::get_input();

        if input.get_state(&KeyNames::Left).is_down && input.get_state(&KeyNames::Right).is_down {
            return;
        }

        if input.get_state(&KeyNames::Left).key_repeat_triggered {
            setting.dec();
        } else if input.get_state(&KeyNames::Right).key_repeat_triggered {
            setting.inc();
        }
    }

    fn check_for_setting_selected(&mut self) {
        let input = crate::globals::get_input();

        if self.setting_selected != SettingSelected::None {
            return;
        }

        if input.get_state(&KeyNames::Confirm).just_released {
            self.setting_selected = self.setting_highlighted.clone();
        }
    }

    fn check_for_setting_deselected(&mut self) -> bool {
        let input = crate::globals::get_input();

        if self.setting_selected != SettingSelected::None
            && input.get_state(&KeyNames::Back).just_released
        {
            self.new_time = None;
            self.setting_selected = SettingSelected::None;

            true
        } else {
            false
        }
    }

    fn check_for_move_highlight(&mut self) {
        let input = crate::globals::get_input();

        if self.setting_selected != SettingSelected::None {
            return;
        }
        if input.get_state(&KeyNames::Left).is_down && input.get_state(&KeyNames::Right).is_down {
            return;
        }

        if input.get_state(&KeyNames::Right).just_pressed {
            self.setting_highlighted = self.setting_highlighted.next().clone();
        } else if input.get_state(&KeyNames::Left).just_pressed {
            self.setting_highlighted = self.setting_highlighted.prev().clone();
        }
    }
}