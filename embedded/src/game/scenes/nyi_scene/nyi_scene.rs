use crate::game::display::text_writer;
use crate::game::hardware::input::KeyNames;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

pub struct NyiScene {
    frame_count: u32,
    frames_since_last_second: u32,
    last_second_fps: u32,
    next_scene: Option<SceneType>,
}

impl SceneBehavior for NyiScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
        }
        if input.get_state(&KeyNames::Clock).just_pressed {
            self.last_second_fps = self.frames_since_last_second;
            self.frames_since_last_second = 0;
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        self.frames_since_last_second += 1;
    }

    fn sound(&mut self) {
        let hardware = crate::game::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        let body = fixedstr::str_format!(fixedstr::str16, "{} FPS", self.last_second_fps);
        text_writer::full_dialog_box("NOT IMPL", &body);
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
impl Default for NyiScene {
    fn default() -> Self {
        Self {
            frame_count: 0,
            frames_since_last_second: 0,
            last_second_fps: 0,
            next_scene: None,
        }
    }
}
