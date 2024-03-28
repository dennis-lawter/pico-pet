use crate::game::display::text_writer;
use crate::game::hardware::input::KeyNames;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

pub struct NyiScene {
    next_scene: Option<SceneType>,
}

impl SceneBehavior for NyiScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
        }
    }

    fn tick(&mut self) {
        ()
    }

    fn sound(&mut self) {
        let hardware = crate::game::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        text_writer::full_dialog_box("NOT IMPL", "todo!()");
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
impl Default for NyiScene {
    fn default() -> Self {
        Self { next_scene: None }
    }
}
