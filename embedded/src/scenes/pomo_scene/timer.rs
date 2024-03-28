use fixedstr::str_format;

use crate::audio::audio_player::AudioPlayer;
use crate::color::Rgb332;
use crate::nvm::settings::SettingType;

use super::pomo_scene::PomoMenuFrame;

#[derive(Clone, Copy)]
pub enum PomoPhase {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

#[derive(Clone, Copy)]
pub enum TimerStatus {
    Stopped,
    Running,
    Paused,
}

#[derive(Clone, Copy)]
pub enum TimerEvent {
    Paused,
    PomoFinished,
    ShortBreakFinished,
    LongBreakFinished,
    None,
}

const PRE_TIMER_WAIT: u16 = 5;

pub struct PomoTimer {
    pub phase: PomoPhase,
    pub timer_status: TimerStatus,
    pub is_exiting: bool,
    pre_timer: u16,
    pub timer: u16,
    phase_complete: bool,
    pub cycles_elapsed: u8,
    timer_values: TimerValues,
    event_queue: TimerEvent,
}
impl Default for PomoTimer {
    fn default() -> Self {
        Self {
            is_exiting: false,
            timer_status: TimerStatus::Stopped,
            pre_timer: 0,
            timer: 0,
            phase: PomoPhase::Pomodoro,
            phase_complete: false,
            cycles_elapsed: 0,
            timer_values: TimerValues::new(),
            event_queue: TimerEvent::None,
        }
    }
}

struct TimerValues {
    pomo_seconds: u16,
    short_break_seconds: u16,
    long_break_seconds: u16,
    cycles: u8,
}
impl TimerValues {
    fn new() -> Self {
        let nvm = crate::globals::get_nvm();
        let pomo_seconds = nvm
            .settings
            .get_setting(SettingType::PomodoroMinutes)
            .get_value() as u16
            * 60;
        let short_break_seconds = nvm
            .settings
            .get_setting(SettingType::ShortRestMinutes)
            .get_value() as u16
            * 60;
        let long_break_seconds = nvm
            .settings
            .get_setting(SettingType::LongRestMinutes)
            .get_value() as u16
            * 60;
        let cycles = nvm
            .settings
            .get_setting(SettingType::PomodoroCycles)
            .get_value();

        Self {
            pomo_seconds,
            short_break_seconds,
            long_break_seconds,
            cycles,
        }
    }
}

impl PomoTimer {
    pub fn pop_event(&mut self) -> TimerEvent {
        let event = self.event_queue;
        self.event_queue = TimerEvent::None;
        event
    }

    fn get_next_phase(&self) -> PomoPhase {
        if self.phase_complete {
            match self.phase {
                PomoPhase::Pomodoro => {
                    if self.cycles_elapsed + 1 < self.timer_values.cycles {
                        PomoPhase::ShortBreak
                    } else {
                        PomoPhase::LongBreak
                    }
                }
                PomoPhase::ShortBreak | PomoPhase::LongBreak => PomoPhase::Pomodoro,
            }
        } else {
            self.phase
        }
    }

    fn advance_phase(&mut self) {
        let next_phase = self.get_next_phase();
        self.phase = next_phase;
        match self.phase {
            PomoPhase::Pomodoro => self.timer = self.timer_values.pomo_seconds,
            PomoPhase::ShortBreak => self.timer = self.timer_values.short_break_seconds,
            PomoPhase::LongBreak => self.timer = self.timer_values.long_break_seconds,
        }
        self.phase_complete = false;
    }

    fn set_timer_status(&mut self, new_timer_status: TimerStatus) {
        match (self.timer_status, new_timer_status) {
            // no change
            (TimerStatus::Running, TimerStatus::Running) => {}
            (TimerStatus::Stopped, TimerStatus::Stopped) => {}
            (TimerStatus::Paused, TimerStatus::Paused) => {}

            (_, TimerStatus::Paused) => {
                self.event_queue = TimerEvent::Paused;
            }

            (TimerStatus::Paused, TimerStatus::Running) => {
                self.pre_timer = PRE_TIMER_WAIT;
            }
            (_, TimerStatus::Stopped) => {
                self.pre_timer = PRE_TIMER_WAIT;
                self.timer = 0;
            }
            (TimerStatus::Stopped, TimerStatus::Running) => {
                self.pre_timer = PRE_TIMER_WAIT;
                self.advance_phase();
            }
        }

        self.timer_status = new_timer_status;
    }

    pub fn back_pressed(&mut self) {
        match self.timer_status {
            TimerStatus::Running | TimerStatus::Paused => {
                self.set_timer_status(TimerStatus::Stopped);
            }
            TimerStatus::Stopped => {
                self.is_exiting = true;
            }
        }
    }

    pub fn confirm_pressed(&mut self) {
        match self.timer_status {
            TimerStatus::Running => {
                self.set_timer_status(TimerStatus::Paused);
            }
            TimerStatus::Paused => {
                self.set_timer_status(TimerStatus::Running);
            }
            TimerStatus::Stopped => {
                self.set_timer_status(TimerStatus::Running);
            }
        }
    }

    fn advance_cycles(&mut self) {
        match self.phase {
            PomoPhase::Pomodoro => {}
            PomoPhase::ShortBreak => {
                self.cycles_elapsed += 1;
            }
            PomoPhase::LongBreak => {
                self.cycles_elapsed = 0;
            }
        }
    }

    fn decrement_timer(&mut self) {
        self.timer -= 1;
        if self.timer == 0 {
            self.phase_complete = true;
            self.set_event_based_on_phase();
            self.set_timer_status(TimerStatus::Stopped);
            self.advance_cycles();
        }
    }

    pub fn timer_interrupt(
        &mut self,
        countdown_321_track: &mut AudioPlayer,
        countdown_go_track: &mut AudioPlayer,
    ) {
        match self.timer_status {
            TimerStatus::Stopped | TimerStatus::Paused => {}
            TimerStatus::Running => {
                if self.pre_timer > 2 {
                    self.pre_timer -= 1;
                    countdown_321_track.play();
                } else if self.pre_timer == 2 {
                    self.pre_timer -= 1;
                    countdown_go_track.play();
                } else if self.pre_timer == 1 {
                    self.pre_timer -= 1;
                } else if self.timer > 0 {
                    self.decrement_timer();
                }
            }
        }
    }

    pub(crate) fn get_back_sprite_frame(&self) -> PomoMenuFrame {
        match self.timer_status {
            TimerStatus::Stopped => PomoMenuFrame::Exit,
            TimerStatus::Paused | TimerStatus::Running => PomoMenuFrame::Stop,
        }
    }

    pub(crate) fn get_confirm_sprite_frame(&self) -> PomoMenuFrame {
        match self.timer_status {
            TimerStatus::Running => PomoMenuFrame::Pause,
            TimerStatus::Stopped | TimerStatus::Paused => PomoMenuFrame::Play,
        }
    }

    pub(crate) fn get_message(&self) -> fixedstr::str24 {
        match (self.timer_status, self.phase, self.phase_complete) {
            (TimerStatus::Stopped, PomoPhase::Pomodoro, true) => {
                str_format!(fixedstr::str24, "Ready for a break?")
            }
            (TimerStatus::Stopped, PomoPhase::Pomodoro, false) => {
                if self.cycles_elapsed == 0 {
                    str_format!(fixedstr::str24, "Let's get to work!")
                } else {
                    str_format!(fixedstr::str24, "Let's get back to work!")
                }
            }
            (TimerStatus::Stopped, PomoPhase::ShortBreak, true) => {
                str_format!(fixedstr::str24, "Let's get back to work!")
            }
            (TimerStatus::Stopped, PomoPhase::ShortBreak, false) => {
                str_format!(fixedstr::str24, "Ready for a break?")
            }
            (TimerStatus::Stopped, PomoPhase::LongBreak, true) => {
                str_format!(fixedstr::str24, "Start again?")
            }
            (TimerStatus::Stopped, PomoPhase::LongBreak, false) => {
                str_format!(fixedstr::str24, "Ready for a break?")
            }
            (TimerStatus::Running, PomoPhase::Pomodoro, _) => {
                str_format!(fixedstr::str24, "Pomodoro #{}", self.cycles_elapsed + 1)
            }
            (TimerStatus::Running, PomoPhase::ShortBreak, _) => {
                str_format!(fixedstr::str24, "Short Break #{}", self.cycles_elapsed + 1)
            }
            (TimerStatus::Running, PomoPhase::LongBreak, _) => {
                str_format!(fixedstr::str24, "Long Break")
            }
            (TimerStatus::Paused, _, _) => {
                str_format!(fixedstr::str24, "Paused...")
            }
        }
    }

    pub fn get_timer_text_and_color(&self) -> (fixedstr::str16, Rgb332) {
        let mut color = Rgb332::BLUE;
        let text = match self.timer_status {
            TimerStatus::Running => {
                if self.pre_timer == PRE_TIMER_WAIT {
                    fixedstr::str16::from("") // Synchronizing with 1hz clock
                } else if self.pre_timer > 1 {
                    str_format!(fixedstr::str16, "{}...", self.pre_timer - 1)
                } else if self.pre_timer == 1 {
                    fixedstr::str16::from("GO!!")
                } else {
                    str_format!(
                        fixedstr::str16,
                        "{:02}:{:02}",
                        self.timer / 60,
                        self.timer % 60,
                    )
                }
            }
            TimerStatus::Stopped => {
                str_format!(fixedstr::str16, "")
            }
            TimerStatus::Paused => {
                color = Rgb332::GREEN;
                str_format!(
                    fixedstr::str16,
                    "{:02}:{:02}",
                    self.timer / 60,
                    self.timer % 60,
                )
            }
        };
        (text, color)
    }

    fn set_event_based_on_phase(&mut self) {
        self.event_queue = match self.phase {
            PomoPhase::Pomodoro => TimerEvent::PomoFinished,
            PomoPhase::ShortBreak => TimerEvent::ShortBreakFinished,
            PomoPhase::LongBreak => TimerEvent::LongBreakFinished,
        }
    }
}
