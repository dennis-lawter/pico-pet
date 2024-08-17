pub mod eat_scene;
pub mod intro_scene;
pub mod main_scene;
pub mod nyi_scene;
pub mod pomo_scene;
pub mod settings_scene;
pub mod stat_scene;

#[derive(Clone)]
pub enum SceneType {
    Intro,

    Main,

    Pomo,
    Eat,
    Stat,
    Cosmetics,
    Settings,
}
impl Default for SceneType {
    fn default() -> Self {
        Self::Intro
    }
}

pub trait SceneBehavior {
    fn input(&mut self);
    fn tick(&mut self);
    fn sound(&mut self);
    fn draw(&mut self);

    fn next_scene(&self) -> &Option<SceneType>;
}
