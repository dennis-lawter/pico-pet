use crate::game::hardware::input::KeyNames;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

pub struct StatScene {
    next_scene: Option<SceneType>,
}

impl Default for StatScene {
    fn default() -> Self {
        Self { next_scene: None }
    }
}

impl SceneBehavior for StatScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();

        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
        }
    }

    fn tick(&mut self) {}

    fn sound(&mut self) {}

    fn draw(&mut self) {}

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
