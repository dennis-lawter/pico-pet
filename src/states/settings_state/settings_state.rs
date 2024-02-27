use fixedstr::str_format;

use crate::color::Rgb332;
use crate::color::{self};
use crate::display::render;
use crate::display::text_writer::FontStyle;
use crate::display::text_writer::{self};
use crate::globals;
use crate::hardware::audio::AudioFrequency;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;
use crate::hardware::rtc::RealTime;
use crate::setting_value::Setting;
use crate::states::AppState;
use crate::states::State;

use super::setting_selected::SettingSelected;
use super::song;

const FRAMES_TO_RESET: u8 = 5 * 16 - 2;

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

    frames_reset_button_held: u8,
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
        render::flood(color::BLACK);

        let title = "SETTINGS";
        let menu_body = "";
        text_writer::full_dialog_box(title, menu_body);

        self.display_cursor();

        // TODO: make these an array
        // TODO: move pomo settings to a submenu

        self.display_brightness_setting(0);

        self.display_volume_setting(1);

        self.display_time_adjustment_setting(2);

        self.display_pomo_time_selector(3);

        self.display_pomo_cycle_selector(4);

        self.display_reset_setting(5);
    }

    fn input(&mut self) {
        let input = crate::globals::get_input();

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
                self.adjust_setting(unsafe { &mut globals::BRIGHTNESS_SETTING });
            }
            SettingSelected::Volume => {
                self.adjust_setting(unsafe { &mut globals::VOLUME_SETTING });
            }
            SettingSelected::Time => {
                self.adjust_time();
            }
            SettingSelected::PomoTime => {
                self.adjust_setting(unsafe { &mut globals::POMO_TIME_SETTING });
            }
            SettingSelected::PomoCycle => {
                self.adjust_setting(unsafe { &mut globals::POMO_CYCLE_SETTING });
            }
            SettingSelected::Reset => {
                self.process_reset();
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
            match self.setting_selected {
                SettingSelected::Brightness
                | SettingSelected::Volume
                | SettingSelected::PomoTime
                | SettingSelected::PomoCycle => {
                    crate::globals::get_nvm().settings.write();
                }
                _ => {}
            }
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

    fn adjust_time(&mut self) {
        let input = crate::globals::get_input();
        let hardware = crate::globals::get_hardware();

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

    fn setting_to_y_offset(setting: &SettingSelected) -> i32 {
        match setting {
            SettingSelected::None => -4, // hide cursor far above screen
            _ => (*setting as i32) * 2 + 1,
        }
    }

    fn display_cursor(&self) {
        let (icon, setting) = match self.setting_selected {
            SettingSelected::None => (" >", &self.setting_highlighted),
            _ => ("4}", &self.setting_selected),
        };

        let y_cursor_offset = Self::setting_to_y_offset(setting);
        text_writer::draw_text(
            10,
            18 + 8 * y_cursor_offset,
            FontStyle::Icon,
            color::RED,
            icon,
        );
    }

    fn display_brightness_setting(&self, y_offset: i32) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + y_offset * 2 * 8,
            FontStyle::Small,
            color::BLACK,
            "BRIGHTNESS",
        );
        text_writer::draw_text(
            24,
            18 + (y_offset * 2 + 1) * 8,
            FontStyle::Icon,
            color::BLUE,
            unsafe { &globals::BRIGHTNESS_SETTING }
                .generate_bar(self.setting_selected == SettingSelected::Brightness),
        );
    }

    fn display_volume_setting(&self, y_offset: i32) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + y_offset * 2 * 8,
            FontStyle::Small,
            color::BLACK,
            "VOLUME",
        );
        text_writer::draw_text(
            24,
            18 + (y_offset * 2 + 1) * 8,
            FontStyle::Icon,
            color::BLUE,
            unsafe { &globals::VOLUME_SETTING }
                .generate_bar(self.setting_selected == SettingSelected::Volume),
        );
    }

    fn display_time_adjustment_setting(&self, y_offset: i32) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + y_offset * 2 * 8,
            FontStyle::Small,
            color::BLACK,
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
                            18 + (y_offset * 2 + 1) * 8,
                            FontStyle::Small,
                            color::BLACK,
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
                            18 + (y_offset * 2 + 1) * 8 - 1,
                            FontStyle::Small,
                            color::BLUE,
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
                            18 + (y_offset * 2 + 1) * 8,
                            FontStyle::Small,
                            color::BLACK,
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
                            18 + (y_offset * 2 + 1) * 8 - 1,
                            FontStyle::Small,
                            color::BLUE,
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
                            18 + (y_offset * 2 + 1) * 8,
                            FontStyle::Small,
                            color::BLACK,
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
                            18 + (y_offset * 2 + 1) * 8 - 1,
                            FontStyle::Small,
                            color::BLUE,
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
                        18 + (y_offset * 2 + 1) * 8,
                        FontStyle::Small,
                        color::BLACK,
                        time_str.as_str(),
                    );
                }
                None => {}
            }
        };
    }

    fn display_pomo_time_selector(&self, y_offset: i32) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + y_offset * 2 * 8,
            FontStyle::Small,
            color::BLACK,
            "POMODORO TIME",
        );
        let pomo_time_setting = unsafe { &globals::POMO_TIME_SETTING }.get_value();
        if self.setting_selected == SettingSelected::PomoTime {
            let time_str = str_format!(fixedstr::str12, "{:02}        ", pomo_time_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                18 + (y_offset * 2 + 1) * 8 - 1,
                FontStyle::Small,
                color::BLUE,
                time_str.as_str(),
            );
        } else {
            let time_str = str_format!(fixedstr::str12, "{:02}        ", pomo_time_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                18 + (y_offset * 2 + 1) * 8,
                FontStyle::Small,
                color::BLACK,
                time_str.as_str(),
            );
        }
        let min_str = "   minutes";
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + (y_offset * 2 + 1) * 8 - 1,
            FontStyle::Small,
            color::BLACK,
            min_str,
        );
    }

    fn display_pomo_cycle_selector(&self, y_offset: i32) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + y_offset * 2 * 8,
            FontStyle::Small,
            color::BLACK,
            "POMODORO CYCLES",
        );
        let pomo_cycle_setting = unsafe { &globals::POMO_CYCLE_SETTING }.get_value();
        if self.setting_selected == SettingSelected::PomoCycle {
            let time_str = str_format!(fixedstr::str12, "{:01}      ", pomo_cycle_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                18 + (y_offset * 2 + 1) * 8 - 1,
                FontStyle::Small,
                color::BLUE,
                time_str.as_str(),
            );
        } else {
            let time_str = str_format!(fixedstr::str12, "{:01}      ", pomo_cycle_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                18 + (y_offset * 2 + 1) * 8,
                FontStyle::Small,
                color::BLACK,
                time_str.as_str(),
            );
        }
        let min_str = "  cycles";
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + (y_offset * 2 + 1) * 8 - 1,
            FontStyle::Small,
            color::BLACK,
            min_str,
        );
    }

    fn display_reset_setting(&self, y_offset: i32) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            18 + y_offset * 2 * 8,
            FontStyle::Small,
            Rgb332::from_u8(0b110_000_00),
            "RESET (HOLD CONFIRM)",
        );

        crate::display::render::solid_line_rect(
            LCD_WIDTH as i32 / 2 - (5 * 8),
            18 + (y_offset * 2 + 1) * 8,
            5 * 16,
            8,
            Rgb332::from_u8(0b110_000_00),
        );

        crate::display::render::fill_rect(
            LCD_WIDTH as i32 / 2 - (5 * 8) + 1,
            18 + (y_offset * 2 + 1) * 8 + 1,
            self.frames_reset_button_held as usize,
            6,
            Rgb332::from_u8(0b001_000_00),
        );
    }

    fn process_reset(&mut self) {
        let input = crate::globals::get_input();
        let nvm = crate::globals::get_nvm();

        if input.get_state(&KeyNames::Confirm).is_down {
            if self.frames_reset_button_held < FRAMES_TO_RESET {
                self.frames_reset_button_held += 1;
            } else {
                nvm.erase_all_then_reboot();
                nvm.settings.apply_to_globals();

                self.frames_reset_button_held = 0;
                self.setting_selected = SettingSelected::None;
            }
        } else {
            self.frames_reset_button_held = 0;
        }
    }
}

impl Default for SettingsState {
    fn default() -> Self {
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

            frames_reset_button_held: 0,
        }
    }
}
