use crate::color::Rgb332;
use crate::display::render;
use crate::display::text_writer;
use crate::hardware::input::KeyNames;
use crate::scenes::SceneBehavior;
use crate::scenes::SceneType;

pub struct NyiScene {
    next_scene: Option<SceneType>,
}

impl SceneBehavior for NyiScene {
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
        render::flood(Rgb332::BLACK);
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
