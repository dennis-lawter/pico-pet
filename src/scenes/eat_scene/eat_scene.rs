use crate::color::Rgb332;
use crate::display::render;
use crate::display::sprite::Sprite;
use crate::display::sprite::SpriteFactory;
use crate::hardware::input::KeyNames;
use crate::scenes::SceneBehavior;
use crate::scenes::SceneType;

pub struct EatScene<'a> {
    next_scene: Option<SceneType>,
    inv_sprite: Sprite<'a>,
}
impl Default for EatScene<'_> {
    fn default() -> Self {
        Self {
            next_scene: None,
            inv_sprite: SpriteFactory::new_inventory_sprite(0, 0),
        }
    }
}

impl SceneBehavior for EatScene<'_> {
    fn input(&mut self) {
        let input = crate::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
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

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
