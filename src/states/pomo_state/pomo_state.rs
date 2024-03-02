use fixedstr::str_format;

use crate::color::Rgb332;
use crate::display::render;
use crate::display::sprite::Sprite;
use crate::display::sprite::SpriteFactory;
use crate::display::text_writer;
use crate::hardware::hardware::LCD_HEIGHT;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;
use crate::states::AppState;
use crate::states::State;

#[derive(Clone, Copy)]
enum PomoPhase {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

enum TimerStatus {
    Stopped,
    Running,
    Paused,
}

const PRE_TIMER_WAIT: u16 = 3;
pub struct PomoState<'a> {
    menu_sprite: Sprite<'a>,
    next_state: Option<AppState>,
    timer_status: TimerStatus,
    pre_timer: u16,
    timer: u16,
    phase: PomoPhase,
    phase_complete: bool,
    cycles_elapsed: u8,
}
impl Default for PomoState<'static> {
    fn default() -> Self {
        Self {
            next_state: None,
            menu_sprite: SpriteFactory::new_pomo_menu_sprite(0, 0),
            timer_status: TimerStatus::Stopped,
            pre_timer: 0,
            timer: 0,
            phase: PomoPhase::Pomodoro,
            phase_complete: false,
            cycles_elapsed: 0,
        }
    }
}

impl PomoState<'_> {
    fn set_timer_status(&mut self, new_pomo_state: TimerStatus) {
        let pomo_seconds = unsafe { &crate::globals::POMO_TIME_SETTING }.get_value() as u16 * 60;
        let short_break_seconds =
            unsafe { &crate::globals::SHORT_REST_TIME_SETTING }.get_value() as u16 * 60;
        let long_break_seconds =
            unsafe { &crate::globals::LONG_REST_TIME_SETTING }.get_value() as u16 * 60;
        let cycles = unsafe { &crate::globals::POMO_CYCLE_SETTING }.get_value();
        match new_pomo_state {
            TimerStatus::Stopped => {
                self.pre_timer = PRE_TIMER_WAIT;
                self.timer = 0;
            }
            TimerStatus::Running => {
                match self.timer_status {
                    TimerStatus::Running => {} // ???
                    TimerStatus::Paused => {
                        self.pre_timer = PRE_TIMER_WAIT;
                    }
                    TimerStatus::Stopped => {
                        let next_phase = if self.phase_complete {
                            match self.phase {
                                PomoPhase::Pomodoro => {
                                    if self.cycles_elapsed + 1 < cycles {
                                        PomoPhase::ShortBreak
                                    } else {
                                        PomoPhase::LongBreak
                                    }
                                }
                                PomoPhase::ShortBreak | PomoPhase::LongBreak => PomoPhase::Pomodoro,
                            }
                        } else {
                            self.phase
                        };
                        self.phase = next_phase;
                        match self.phase {
                            // TODO: fix this nesting...
                            PomoPhase::Pomodoro => self.timer = pomo_seconds,
                            PomoPhase::ShortBreak => self.timer = short_break_seconds,
                            PomoPhase::LongBreak => self.timer = long_break_seconds,
                        }
                    }
                }
                self.pre_timer = PRE_TIMER_WAIT;
            }
            TimerStatus::Paused => {
                self.pre_timer = PRE_TIMER_WAIT;
            }
        }

        self.timer_status = new_pomo_state;
    }
}

impl State for PomoState<'_> {
    fn input(&mut self) {
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            match self.timer_status {
                TimerStatus::Running | TimerStatus::Paused => {
                    self.set_timer_status(TimerStatus::Stopped);
                }
                TimerStatus::Stopped => {
                    self.next_state = Some(AppState::Main);
                }
            }
        } else if input.get_state(&KeyNames::Confirm).just_released {
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
    }

    fn tick(&mut self) {
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Clock).just_pressed {
            if self.pre_timer > 0 {
                self.pre_timer -= 1;
            } else if self.timer > 0 {
                self.timer -= 1;
                if self.timer == 0 {
                    self.phase_complete = true;
                    self.set_timer_status(TimerStatus::Stopped);
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
            }
        }
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        render::flood(Rgb332::BLACK);

        self.menu_sprite.x = 0;
        self.menu_sprite.y = (LCD_HEIGHT - self.menu_sprite.h) as i32;
        self.menu_sprite.draw(2);

        let frame;
        match self.timer_status {
            TimerStatus::Running => {
                match self.phase {
                    PomoPhase::Pomodoro => text_writer::bottom_dialog_box(&str_format!(
                        fixedstr::str16,
                        "Pomodoro #{}",
                        self.cycles_elapsed + 1
                    )),
                    PomoPhase::ShortBreak => text_writer::bottom_dialog_box(&str_format!(
                        fixedstr::str16,
                        "Short Break #{}",
                        self.cycles_elapsed + 1
                    )),
                    PomoPhase::LongBreak => text_writer::bottom_dialog_box("Long Break"),
                }
                if self.pre_timer > 0 {
                    text_writer::bottom_big_dialog_box(&str_format!(
                        fixedstr::str8,
                        "{}...",
                        self.pre_timer
                    ))
                } else if self.timer > 0 {
                    let time_left = self.timer;
                    text_writer::bottom_big_dialog_box(&str_format!(
                        fixedstr::str8,
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
                text_writer::bottom_dialog_box("Ready?");
                frame = 0;
            }
        }
        self.menu_sprite.x = (LCD_WIDTH - self.menu_sprite.w) as i32;
        self.menu_sprite.y = (LCD_HEIGHT - self.menu_sprite.h) as i32;
        self.menu_sprite.draw(frame);
    }

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}
