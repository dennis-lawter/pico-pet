use fixedstr::str_format;

use crate::color::Rgb332;

use super::pomo_state::PomoMenuFrame;

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

const PRE_TIMER_WAIT: u16 = 3;

pub struct PomoTimer {
    pub phase: PomoPhase,
    pub timer_status: TimerStatus,
    pub is_exiting: bool,
    pre_timer: u16,
    timer: u16,
    phase_complete: bool,
    cycles_elapsed: u8,
    timer_values: TimerValues,
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
        Self {
            pomo_seconds: unsafe { &crate::globals::POMO_TIME_SETTING }.get_value() as u16 * 60,
            short_break_seconds: unsafe { &crate::globals::SHORT_REST_TIME_SETTING }.get_value()
                as u16
                * 60,
            long_break_seconds: unsafe { &crate::globals::LONG_REST_TIME_SETTING }.get_value()
                as u16
                * 60,
            cycles: unsafe { &crate::globals::POMO_CYCLE_SETTING }.get_value(),
        }
    }
}

impl PomoTimer {
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

    fn set_timer_status(&mut self, new_pomo_state: TimerStatus) {
        match (self.timer_status, new_pomo_state) {
            // no change
            (TimerStatus::Running, TimerStatus::Running) => {}
            (TimerStatus::Paused, TimerStatus::Paused) => {}
            (TimerStatus::Stopped, TimerStatus::Stopped) => {}
            (TimerStatus::Stopped, TimerStatus::Paused) => {}
            (TimerStatus::Running, TimerStatus::Paused) => {}

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

        self.timer_status = new_pomo_state;
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
            self.set_timer_status(TimerStatus::Stopped);
            self.advance_cycles();
        }
    }

    pub fn timer_interrupt(&mut self) {
        match self.timer_status {
            TimerStatus::Stopped | TimerStatus::Paused => {}
            TimerStatus::Running => {
                if self.pre_timer > 0 {
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

    pub(crate) fn get_timer_text_and_color(&self) -> (fixedstr::str16, Rgb332) {
        let mut color = Rgb332::BLUE;
        let text = match self.timer_status {
            TimerStatus::Running => {
                if self.pre_timer > 0 {
                    str_format!(fixedstr::str16, "{}...", self.pre_timer)
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

    /*
    match self.timer.timer_status {
                TimerStatus::Running => {
                    match self.timer.phase {
                        PomoPhase::Pomodoro => text_writer::bottom_dialog_box(&str_format!(
                            fixedstr::str24,
                            "Pomodoro #{}",
                            self.cycles_elapsed + 1
                        )),
                        PomoPhase::ShortBreak => text_writer::bottom_dialog_box(&str_format!(
                            fixedstr::str24,
                            "Short Break #{}",
                            self.cycles_elapsed + 1
                        )),
                        PomoPhase::LongBreak => text_writer::bottom_dialog_box("Long Break"),
                    }
                    if self.pre_timer > 0 {
                        text_writer::bottom_big_dialog_box(&str_format!(
                            fixedstr::str16,
                            "{}...",
                            self.pre_timer
                        ))
                    } else if self.timer > 0 {
                        let time_left = self.timer;
                        text_writer::bottom_big_dialog_box(&str_format!(
                            fixedstr::str16,
                            "{:02}:{:02}",
                            time_left / 60,
                            time_left % 60,
                        ));
                    } else {
                        text_writer::bottom_big_dialog_box("FINISHED");
                    }
                    frame = 1;
                }
                TimerStatus::Paused => {
                    text_writer::bottom_dialog_box("...Paused...");
                    frame = 0
                }
                TimerStatus::Stopped => {
                    if self.phase_complete {
                        match self.phase {
                            PomoPhase::Pomodoro => text_writer::bottom_dialog_box("Ready for a break?"),
                            PomoPhase::ShortBreak => {
                                text_writer::bottom_dialog_box("Let's get back to work!")
                            }
                            PomoPhase::LongBreak => {
                                text_writer::bottom_dialog_box("Refreshing! Again?")
                            }
                        }
                    } else {
                        match self.phase {
                            PomoPhase::Pomodoro => {
                                if self.cycles_elapsed == 0 {
                                    text_writer::bottom_dialog_box("Let's get to work!")
                                } else {
                                    text_writer::bottom_dialog_box("Let's get back to work!")
                                }
                            }
                            PomoPhase::ShortBreak | PomoPhase::LongBreak => {
                                text_writer::bottom_dialog_box("Ready for a break?")
                            }
                        }
                    }

                    0
                }
            };
         */
}
