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

use super::timer::PomoTimer;

pub struct PomoState<'a> {
    menu_sprite: Sprite<'a>,
    next_state: Option<AppState>,
    timer: PomoTimer,
}
impl Default for PomoState<'static> {
    fn default() -> Self {
        Self {
            next_state: None,
            menu_sprite: SpriteFactory::new_pomo_menu_sprite(0, 0),
            timer: PomoTimer::default(),
        }
    }
}

pub enum PomoMenuFrame {
    Play = 0,
    Pause = 1,
    Exit = 2,
    Stop = 3,
}

impl State for PomoState<'_> {
    fn input(&mut self) {
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.timer.back_pressed();
            if self.timer.is_exiting {
                self.next_state = Some(AppState::Main);
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
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
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

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}
