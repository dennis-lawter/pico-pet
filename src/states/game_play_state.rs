use crate::{
    display::{
        render,
        sprite::{Sprite, SpriteFactory},
        text_writer::{self},
    },
    system::{Frequency, SystemComponents},
};

use super::{AppState, State};

pub struct GamePlayState<'a> {
    ferris: Sprite<'a>,
    corro: Sprite<'a>,
    frame_count: u32,
    next_state: Option<AppState>,
}
impl State for GamePlayState<'static> {
    fn new() -> Self {
        let mut ferris = SpriteFactory::new_ferris_sprite();
        ferris.x = 32;
        ferris.y = 32;

        let mut corro = SpriteFactory::new_corro_sprite();
        corro.x = 64;
        corro.y = 64;

        Self {
            ferris,
            corro,
            frame_count: 0,
            next_state: None,
        }
    }

    fn tick(&mut self, _system: &mut SystemComponents) {
        self.frame_count += 1;
    }

    fn sound(&mut self, system: &mut SystemComponents) {
        if (self.frame_count / 20) % 2 == 1 {
            if self.frame_count % 4 == 0 {
                system.start_tone(&Frequency::C4);
            } else if self.frame_count % 4 == 2 {
                system.start_tone(&Frequency::A4);
            } else {
                system.start_tone(&Frequency::None);
            }
        } else {
            system.end_tone();
        }
    }

    fn draw(&mut self, _system: &mut SystemComponents) {
        render::flood(0b000_000_00);

        self.corro.draw(0);

        self.ferris.draw(((self.frame_count / 20) % 2) as usize);
        let text = "DIALOG\\b700!\\b703 so \\c700smol\\c003\\\\ so cute";
        text_writer::bottom_dialog_box(text);
    }

    fn swap(&mut self, system: &mut SystemComponents) {
        system.set_backlight();
        render::draw(&mut system.display);
    }

    fn input(&mut self, system: &mut SystemComponents) {
        if system.key2_pressed() && system.key3_pressed() {
            self.next_state = Some(AppState::Menu);
            return;
        }
        if system.key0_pressed() {
            self.ferris.x -= 1;
        }
        if system.key1_pressed() {
            self.ferris.y += 1;
        }
        if system.key2_pressed() {
            self.ferris.y -= 1;
        }
        if system.key3_pressed() {
            self.ferris.x += 1;
        }
    }

    fn next_state(&mut self) -> &Option<super::AppState> {
        &self.next_state
    }
}
