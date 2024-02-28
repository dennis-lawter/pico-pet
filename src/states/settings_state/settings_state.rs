use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::render;
use crate::display::text_writer::FontStyle;
use crate::display::text_writer::{self};
use crate::globals;
use crate::hardware::audio::AudioFrequency;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;
use crate::setting_value::Setting;
use crate::states::AppState;
use crate::states::State;

use super::setting_components::SettingComponent;
use super::setting_selected::SettingSelected;
use super::song;

const FRAMES_TO_RESET: u8 = 5 * 16 - 2;
const SETTING_HEIGHT_OFFSET: i32 = 10;
const SCROLL_AMOUNT: i32 = 16;
const SCROLL_LIMIT: i32 = SCROLL_AMOUNT * 8;

pub struct SettingsState {
    frame_count: u32,
    next_state: Option<AppState>,
    song: [AudioFrequency; 396],
    current_frequency: AudioFrequency,
    pub setting_selected: SettingSelected,
    setting_highlighted: SettingSelected,
    input_enabled: bool,

    setting_components: [SettingComponent; 4],

    scroll_offset: i32,

    frames_reset_button_held: u8,
}
impl State for SettingsState {
    fn tick(&mut self) {
        for component in self.setting_components.iter_mut() {
            component.tick();
        }
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
        render::flood(Rgb332::BLACK);

        let title = "SETTINGS";
        let menu_body = "";
        text_writer::full_dialog_box(title, menu_body);

        self.display_cursor();

        let offset_cycle = [0, -1, -2, -3, -2, -1];
        let animation_frames = 5;
        let phase = self.frame_count as usize / animation_frames % offset_cycle.len();
        let cycle_position = offset_cycle[phase];
        let top_arrow_offset = SETTING_HEIGHT_OFFSET + 8 + 1 + cycle_position as i32;

        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            top_arrow_offset,
            FontStyle::Icon,
            Rgb332::BLUE,
            "kl",
        );

        self.setting_components[0].draw(
            24 + 16 * 0,
            self.setting_selected == SettingSelected::Brightness,
        );
        self.setting_components[1].draw(
            24 + 16 * 1,
            self.setting_selected == SettingSelected::Volume,
        );
        self.setting_components[2]
            .draw(24 + 16 * 2, self.setting_selected == SettingSelected::Time);
        self.setting_components[3]
            .draw(24 + 16 * 3, self.setting_selected == SettingSelected::Reset);
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
                self.setting_components[0].input();
            }
            SettingSelected::Volume => {
                self.setting_components[1].input();
            }
            SettingSelected::Time => {
                self.setting_components[2].input();
            }
            SettingSelected::PomoTime => {
                self.adjust_setting(unsafe { &mut globals::POMO_TIME_SETTING });
            }
            SettingSelected::PomoCycle => {
                self.adjust_setting(unsafe { &mut globals::POMO_CYCLE_SETTING });
            }
            SettingSelected::Reset => {
                self.setting_components[3].input();
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
            // self.new_time = None;
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

    fn adjust_time(&mut self) {}

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
            SETTING_HEIGHT_OFFSET + 8 * 2 + 8 * y_cursor_offset,
            FontStyle::Icon,
            Rgb332::RED,
            icon,
        );
    }

    fn display_pomo_time_selector(&self, y_offset: i32) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            SETTING_HEIGHT_OFFSET + y_offset * 2 * 8,
            FontStyle::Small,
            Rgb332::BLACK,
            "POMODORO TIME",
        );
        let pomo_time_setting = unsafe { &globals::POMO_TIME_SETTING }.get_value();
        if self.setting_selected == SettingSelected::PomoTime {
            let time_str = str_format!(fixedstr::str12, "{:02}        ", pomo_time_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                SETTING_HEIGHT_OFFSET + (y_offset * 2 + 1) * 8 - 1,
                FontStyle::Small,
                Rgb332::BLUE,
                time_str.as_str(),
            );
        } else {
            let time_str = str_format!(fixedstr::str12, "{:02}        ", pomo_time_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                SETTING_HEIGHT_OFFSET + (y_offset * 2 + 1) * 8,
                FontStyle::Small,
                Rgb332::BLACK,
                time_str.as_str(),
            );
        }
        let min_str = "   minutes";
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            SETTING_HEIGHT_OFFSET + (y_offset * 2 + 1) * 8 - 1,
            FontStyle::Small,
            Rgb332::BLACK,
            min_str,
        );
    }

    fn display_pomo_cycle_selector(&self, y_offset: i32) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            SETTING_HEIGHT_OFFSET + y_offset * 2 * 8,
            FontStyle::Small,
            Rgb332::BLACK,
            "POMODORO CYCLES",
        );
        let pomo_cycle_setting = unsafe { &globals::POMO_CYCLE_SETTING }.get_value();
        if self.setting_selected == SettingSelected::PomoCycle {
            let time_str = str_format!(fixedstr::str12, "{:01}      ", pomo_cycle_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                SETTING_HEIGHT_OFFSET + (y_offset * 2 + 1) * 8 - 1,
                FontStyle::Small,
                Rgb332::BLUE,
                time_str.as_str(),
            );
        } else {
            let time_str = str_format!(fixedstr::str12, "{:01}      ", pomo_cycle_setting,);
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                SETTING_HEIGHT_OFFSET + (y_offset * 2 + 1) * 8,
                FontStyle::Small,
                Rgb332::BLACK,
                time_str.as_str(),
            );
        }
        let min_str = "  cycles";
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            SETTING_HEIGHT_OFFSET + (y_offset * 2 + 1) * 8 - 1,
            FontStyle::Small,
            Rgb332::BLACK,
            min_str,
        );
    }
}

impl Default for SettingsState {
    fn default() -> Self {
        let setting_components = [
            SettingComponent::Brightness(
                super::setting_components::BrightnessSettingComponent::default(),
            ),
            SettingComponent::Volume(super::setting_components::VolumeSettingComponent::default()),
            SettingComponent::Time(super::setting_components::TimeSettingComponent::default()),
            SettingComponent::Reset(super::setting_components::ResetSettingComponent::default()),
        ];
        Self {
            frame_count: 0,
            next_state: None,
            song: song::BALL_GAME,
            current_frequency: AudioFrequency::None,
            setting_selected: SettingSelected::None,
            setting_highlighted: SettingSelected::None,
            input_enabled: false,
            setting_components,

            scroll_offset: 0,

            frames_reset_button_held: 0,
        }
    }
}
