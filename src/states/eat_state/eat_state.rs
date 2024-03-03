use crate::color::Rgb332;
use crate::display::render;
use crate::display::sprite::Sprite;
use crate::display::sprite::SpriteFactory;
use crate::hardware::input::KeyNames;
use crate::states::AppState;
use crate::states::State;

pub struct EatState<'a> {
    next_state: Option<AppState>,
    inv_sprite: Sprite<'a>,
}
impl Default for EatState<'_> {
    fn default() -> Self {
        Self {
            next_state: None,
            inv_sprite: SpriteFactory::new_inventory_sprite(0, 0),
        }
    }
}

impl State for EatState<'_> {
    fn input(&mut self) {
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.next_state = Some(AppState::Main);
        }
    }

    fn tick(&mut self) {
        ()
    }

    fn sound(&mut self) {
        let hardware = crate::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        render::flood(Rgb332::WHITE);

        self.inv_sprite.x = 0;
        self.inv_sprite.y = 0;
        self.inv_sprite.draw(0);

        self.inv_sprite.y += self.inv_sprite.h as i32;
        self.inv_sprite.draw(1);

        self.inv_sprite.y += self.inv_sprite.h as i32;
        self.inv_sprite.draw(2);

        self.inv_sprite.y += self.inv_sprite.h as i32;
        self.inv_sprite.draw(3);

        self.inv_sprite.y += self.inv_sprite.h as i32;
        self.inv_sprite.draw(4);
    }

    fn next_state(&self) -> &Option<AppState> {
        &self.next_state
    }
}
