use crate::game::display::text_writer;
use crate::game::hardware::input::KeyNames;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

pub struct IntroScene {
    frame: usize,
    next_scene: Option<SceneType>,
}
impl Default for IntroScene {
    fn default() -> Self {
        let nvm = crate::game::globals::get_nvm();
        let next_scene = if nvm.fresh {
            None
        } else {
            // DEBUG
            // None
            Some(SceneType::Main)
        };
        Self {
            next_scene,
            frame: 0,
        }
    }
}
impl SceneBehavior for IntroScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Confirm).just_released {
            let nvm = crate::game::globals::get_nvm();
            nvm.fresh = false;
            nvm.write_all();
            self.next_scene = Some(SceneType::Main);
        }
    }

    fn tick(&mut self) {
        self.frame += 1;
    }

    fn sound(&mut self) {}

    fn draw(&mut self) {
        if self.frame > 22 {
            text_writer::full_dialog_box("Setup", "DEBUGGING\npress ok to skip");
        }
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
