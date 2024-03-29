use crate::game::scenes::eat_scene::EatScene;
use crate::game::scenes::main_scene::MainScene;
use crate::game::scenes::nyi_scene::NyiScene;
use crate::game::scenes::pomo_scene::PomoScene;
use crate::game::scenes::settings_scene::SettingsScene;
use crate::game::scenes::stat_scene::stat_scene::StatScene;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

#[derive(Default)]
pub struct SceneManager<'a> {
    pub game_play_scene: Option<MainScene<'a>>,
    pub pomo_scene: Option<PomoScene<'a>>,
    pub eat_scene: Option<EatScene<'a>>,
    pub stat_scene: Option<StatScene>,
    pub cosmetic_scene: Option<NyiScene>,
    pub settings_scene: Option<SettingsScene>,

    pub active_scene: SceneType,
}
impl SceneManager<'static> {
    fn get_scene(&mut self) -> &mut dyn SceneBehavior {
        match self.active_scene {
            SceneType::Main => self.game_play_scene.as_mut().unwrap(),
            SceneType::Settings => self.settings_scene.as_mut().unwrap(),
            SceneType::Pomo => self.pomo_scene.as_mut().unwrap(),
            SceneType::Eat => self.eat_scene.as_mut().unwrap(),
            SceneType::Stat => self.stat_scene.as_mut().unwrap(),
            SceneType::Cosmetics => self.cosmetic_scene.as_mut().unwrap(),
        }
    }

    pub fn update_and_draw(&mut self) {
        let curr_scene = self.get_scene();
        curr_scene.input();
        curr_scene.tick();
        curr_scene.sound();
        curr_scene.draw();
    }

    pub fn advance_scene(&mut self) {
        let curr_scene = self.get_scene();
        match curr_scene.next_scene().clone() {
            Some(next_scene) => {
                {
                    let hardware = crate::game::globals::get_hardware();
                    hardware.start_tone(&crate::game::hardware::audio::AudioFrequency::None);
                }
                self.active_scene = next_scene.clone();
                self.game_play_scene = None;
                self.pomo_scene = None;
                self.eat_scene = None;
                self.stat_scene = None;
                self.cosmetic_scene = None;
                self.settings_scene = None;

                match next_scene {
                    SceneType::Main => self.game_play_scene = Some(MainScene::default()),
                    SceneType::Pomo => self.pomo_scene = Some(PomoScene::default()),
                    SceneType::Eat => self.eat_scene = Some(EatScene::default()),
                    SceneType::Stat => self.stat_scene = Some(StatScene::default()),
                    SceneType::Cosmetics => self.cosmetic_scene = Some(NyiScene::default()),
                    SceneType::Settings => self.settings_scene = Some(SettingsScene::default()),
                }
            }
            None => {}
        }
    }
}
