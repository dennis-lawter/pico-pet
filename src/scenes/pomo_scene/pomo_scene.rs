use crate::color::Rgb332;
use crate::display::sprite::Sprite;
use crate::display::sprite::SpriteFactory;
use crate::display::text_writer;
use crate::display::text_writer::FontStyle;
use crate::hardware::audio::AudioFrequency as Freq;
use crate::hardware::hardware::LCD_HEIGHT;
use crate::hardware::hardware::LCD_WIDTH;
use crate::hardware::input::KeyNames;
use crate::scenes::SceneBehavior;
use crate::scenes::SceneType;

use super::timer::TimerEvent;

use super::sounds;
use super::timer::PomoTimer;

const ANIM_ON_RATE: usize = 3;

pub struct PomoScene<'a> {
    menu_sprite: Sprite<'a>,
    lofi_sprite: Sprite<'a>,
    next_scene: Option<SceneType>,
    timer: PomoTimer,
    current_frequency: Freq,
    pomo_finished_sound: &'a [Freq],
    pomo_finished_index: Option<usize>,
    break_finished_sound: &'a [Freq],
    break_finished_index: Option<usize>,
    frame_count: usize,
}
impl Default for PomoScene<'static> {
    fn default() -> Self {
        Self {
            next_scene: None,
            menu_sprite: SpriteFactory::new_pomo_menu_sprite(0, 0),
            lofi_sprite: SpriteFactory::new_lofi_sprite(0, 8),
            timer: PomoTimer::default(),
            current_frequency: Freq::None,
            pomo_finished_sound: &sounds::POMO_FINISHED,
            break_finished_sound: &sounds::BREAK_FINISHED,
            pomo_finished_index: None,
            break_finished_index: None,
            frame_count: 0,
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
    pub fn is_playing_alert(&self) -> bool {
        match (self.break_finished_index, self.pomo_finished_index) {
            (Some(_), _) => true,
            (_, Some(_)) => true,
            _ => false,
        }
    }
}

impl SceneBehavior for PomoScene<'_> {
    fn input(&mut self) {
        if self.is_playing_alert() {
            return;
        }
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
        self.frame_count += 1;
        if self.frame_count > SpriteFactory::LOFI_DIMENSIONS.2 * ANIM_ON_RATE {
            self.frame_count = 0;
        }
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Clock).just_pressed {
            self.timer.timer_interrupt();
        }
        match self.timer.pop_event() {
            TimerEvent::Paused => {}
            TimerEvent::PomoFinished => {
                self.start_pomo_sound();
                let nvm = crate::globals::get_nvm();
                let cycles = unsafe { &crate::globals::POMO_CYCLE_SETTING }.get_value();
                let inventory = &mut nvm.inventory;
                inventory.inc_tomatoes();
                if self.timer.cycles_elapsed + 1 == cycles {
                    inventory.inc_raspberries();
                    inventory.inc_juice();
                } else if self.timer.cycles_elapsed > 0 {
                    inventory.inc_juice();
                }
                inventory.write();
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
                hardware.stop_vibrating();
            }
            (Some(index), None) => {
                hardware.start_vibrating();
                if self.current_frequency != self.break_finished_sound[*index] {
                    hardware.start_tone(&self.break_finished_sound[*index]);
                }
                *index += 1;
                if *index >= self.break_finished_sound.len() {
                    self.break_finished_index = None;
                    hardware.stop_vibrating();
                    hardware.end_tone();
                }
            }
            (None, Some(index)) => {
                hardware.start_vibrating();
                if self.current_frequency != self.pomo_finished_sound[*index] {
                    hardware.start_tone(&self.pomo_finished_sound[*index]);
                }
                *index += 1;
                if *index >= self.pomo_finished_sound.len() {
                    self.pomo_finished_index = None;
                    hardware.stop_vibrating();
                    hardware.end_tone();
                }
            }
        }
    }

    fn draw(&mut self) {
        if self.is_playing_alert() {
            text_writer::full_dialog_box("", "");
            let x = LCD_WIDTH as i32 / 2;
            let y = LCD_HEIGHT as i32 / 2 - 7;
            let style = FontStyle::BigBold;
            let color = Rgb332::RED;
            let text = "TIME'S UP!!!";
            text_writer::draw_text_centered(x, y, style, color, text);
            return;
        }

        self.lofi_sprite
            .draw((self.frame_count / ANIM_ON_RATE) % 180);

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
