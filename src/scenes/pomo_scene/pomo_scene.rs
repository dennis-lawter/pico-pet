use crate::color::Rgb332;
use crate::display::render;
use crate::display::sprite::Sprite;
use crate::display::sprite::SpriteFactory;
use crate::display::text_writer;
use crate::hardware::audio::AudioFrequency as Freq;
use crate::hardware::hardware::LCD_HEIGHT;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;
use crate::scenes::SceneBehavior;
use crate::scenes::SceneType;

use super::timer::TimerEvent;

use super::sounds;
use super::timer::PomoTimer;

pub struct PomoScene<'a> {
    menu_sprite: Sprite<'a>,
    next_scene: Option<SceneType>,
    timer: PomoTimer,
    current_frequency: Freq,
    pomo_finished_sound: &'a [Freq],
    pomo_finished_index: Option<usize>,
    break_finished_sound: &'a [Freq],
    break_finished_index: Option<usize>,
}
impl Default for PomoScene<'static> {
    fn default() -> Self {
        Self {
            next_scene: None,
            menu_sprite: SpriteFactory::new_pomo_menu_sprite(0, 0),
            timer: PomoTimer::default(),
            current_frequency: Freq::None,
            pomo_finished_sound: &sounds::POMO_FINISHED,
            break_finished_sound: &sounds::BREAK_FINISHED,
            pomo_finished_index: None,
            break_finished_index: None,
        }
    }
}

pub enum PomoMenuFrame {
    Play = 0,
    Pause = 1,
    Exit = 2,
    Stop = 3,
}

impl PomoScene<'_> {
    pub fn start_pomo_sound(&mut self) {
        self.pomo_finished_index = Some(0);
    }
    pub fn start_break_sound(&mut self) {
        self.break_finished_index = Some(0);
    }
}

impl SceneBehavior for PomoScene<'_> {
    fn input(&mut self) {
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.timer.back_pressed();
            if self.timer.is_exiting {
                self.next_scene = Some(SceneType::Main);
            }
        } else if input.get_state(&KeyNames::Confirm).just_released {
            self.timer.confirm_pressed();
        }
    }

    fn tick(&mut self) {
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Clock).just_pressed {
            self.timer.timer_interrupt();
        }
        match self.timer.pop_event() {
            TimerEvent::Paused => {}
            TimerEvent::PomoFinished => {
                self.start_pomo_sound();
            }
            TimerEvent::ShortBreakFinished | TimerEvent::LongBreakFinished => {
                self.start_break_sound();
            }
            TimerEvent::None => {}
        }
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();

        match (
            &mut self.break_finished_index,
            &mut self.pomo_finished_index,
        ) {
            (None, None) => {
                hardware.end_tone();
            }
            (Some(_), Some(_)) => {
                self.pomo_finished_index = None;
                self.break_finished_index = None;
                hardware.end_tone();
            }
            (Some(index), None) => {
                if self.current_frequency != self.break_finished_sound[*index] {
                    hardware.start_tone(&self.break_finished_sound[*index]);
                }
                *index += 1;
                if *index >= self.break_finished_sound.len() {
                    self.break_finished_index = None;
                    hardware.end_tone();
                }
            }
            (None, Some(index)) => {
                if self.current_frequency != self.pomo_finished_sound[*index] {
                    hardware.start_tone(&self.pomo_finished_sound[*index]);
                }
                *index += 1;
                if *index >= self.pomo_finished_sound.len() {
                    self.pomo_finished_index = None;
                    hardware.end_tone();
                }
            }
        }
    }

    fn draw(&mut self) {
        render::flood(Rgb332::BLACK);

        let back_sprite_frame = self.timer.get_back_sprite_frame();
        self.menu_sprite.x = 0;
        self.menu_sprite.y = (LCD_HEIGHT - self.menu_sprite.h) as i32;
        self.menu_sprite.draw(back_sprite_frame as usize);

        let confirm_sprite_frame = self.timer.get_confirm_sprite_frame();
        self.menu_sprite.x = (LCD_WIDTH - self.menu_sprite.w) as i32;
        self.menu_sprite.y = (LCD_HEIGHT - self.menu_sprite.h) as i32;
        self.menu_sprite.draw(confirm_sprite_frame as usize);

        let message = self.timer.get_message();
        text_writer::bottom_dialog_box(&message);

        let (timer_text, color) = self.timer.get_timer_text_and_color();
        text_writer::bottom_big_dialog_box_custom_color(&timer_text, color);
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
