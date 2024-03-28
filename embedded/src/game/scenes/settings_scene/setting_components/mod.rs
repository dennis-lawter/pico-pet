pub mod brightness_setting_component;
pub mod feeding_deadline_setting_component;
pub mod long_rest_setting_component;
pub mod pomo_cycle_setting_component;
pub mod pomo_time_setting_component;
pub mod reset_setting_component;
pub mod short_rest_setting_component;
pub mod time_setting_component;
pub mod volume_setting_component;

pub use self::brightness_setting_component::BrightnessSettingComponent;
pub use self::feeding_deadline_setting_component::FeedingDeadlineSettingComponent;
pub use self::long_rest_setting_component::LongRestSettingComponent;
pub use self::pomo_cycle_setting_component::PomoCycleSettingComponent;
pub use self::pomo_time_setting_component::PomoTimeSettingComponent;
pub use self::reset_setting_component::ResetSettingComponent;
pub use self::short_rest_setting_component::ShortRestSettingComponent;
pub use self::time_setting_component::TimeSettingComponent;
pub use self::volume_setting_component::VolumeSettingComponent;

use crate::game::hardware::input::KeyNames;
use crate::game::setting_value::Setting;

pub enum SettingComponent {
    Brightness(BrightnessSettingComponent),
    Volume(VolumeSettingComponent),
    Time(TimeSettingComponent),
    PomoTime(PomoTimeSettingComponent),
    ShortRest(ShortRestSettingComponent),
    LongRest(LongRestSettingComponent),
    PomoCycle(PomoCycleSettingComponent),
    FeedingDeadline(FeedingDeadlineSettingComponent),
    Reset(ResetSettingComponent),
}
impl SettingComponent {
    pub fn draw(&mut self, y_offset: i32, selected: bool) {
        match self {
            SettingComponent::Brightness(component) => component.draw(y_offset, selected),
            SettingComponent::Volume(component) => component.draw(y_offset, selected),
            SettingComponent::Time(component) => component.draw(y_offset, selected),
            SettingComponent::PomoTime(component) => component.draw(y_offset, selected),
            SettingComponent::ShortRest(component) => component.draw(y_offset, selected),
            SettingComponent::LongRest(component) => component.draw(y_offset, selected),
            SettingComponent::PomoCycle(component) => component.draw(y_offset, selected),
            SettingComponent::FeedingDeadline(component) => component.draw(y_offset, selected),
            SettingComponent::Reset(component) => component.draw(y_offset, selected),
        }
    }

    pub fn tick(&mut self) {
        match self {
            SettingComponent::Brightness(component) => component.tick(),
            SettingComponent::Volume(component) => component.tick(),
            SettingComponent::Time(component) => component.tick(),
            SettingComponent::PomoTime(component) => component.tick(),
            SettingComponent::ShortRest(component) => component.tick(),
            SettingComponent::LongRest(component) => component.tick(),
            SettingComponent::PomoCycle(component) => component.tick(),
            SettingComponent::FeedingDeadline(component) => component.tick(),
            SettingComponent::Reset(component) => component.tick(),
        }
    }

    pub fn input(&mut self) {
        match self {
            SettingComponent::Brightness(component) => component.input(),
            SettingComponent::Volume(component) => component.input(),
            SettingComponent::Time(component) => component.input(),
            SettingComponent::PomoTime(component) => component.input(),
            SettingComponent::ShortRest(component) => component.input(),
            SettingComponent::LongRest(component) => component.input(),
            SettingComponent::PomoCycle(component) => component.input(),
            SettingComponent::FeedingDeadline(component) => component.input(),
            SettingComponent::Reset(component) => component.input(),
        }
    }

    pub fn is_deselected(&mut self) -> bool {
        match self {
            SettingComponent::Brightness(component) => component.is_deselected(),
            SettingComponent::Volume(component) => component.is_deselected(),
            SettingComponent::Time(component) => component.is_deselected(),
            SettingComponent::PomoTime(component) => component.is_deselected(),
            SettingComponent::ShortRest(component) => component.is_deselected(),
            SettingComponent::LongRest(component) => component.is_deselected(),
            SettingComponent::PomoCycle(component) => component.is_deselected(),
            SettingComponent::FeedingDeadline(component) => component.is_deselected(),
            SettingComponent::Reset(component) => component.is_deselected(),
        }
    }

    pub fn reset(&mut self) {
        match self {
            SettingComponent::Brightness(component) => component.reset_internal_state(),
            SettingComponent::Volume(component) => component.reset_internal_state(),
            SettingComponent::Time(component) => component.reset_internal_state(),
            SettingComponent::PomoTime(component) => component.reset_internal_state(),
            SettingComponent::ShortRest(component) => component.reset_internal_state(),
            SettingComponent::LongRest(component) => component.reset_internal_state(),
            SettingComponent::PomoCycle(component) => component.reset_internal_state(),
            SettingComponent::FeedingDeadline(component) => component.reset_internal_state(),
            SettingComponent::Reset(component) => component.reset_internal_state(),
        }
    }
}

trait SettingComponentTrait {
    fn draw(&mut self, y_offset: i32, selected: bool);
    fn tick(&mut self);
    fn input(&mut self);
    fn is_deselected(&mut self) -> bool;
    fn reset_internal_state(&mut self);
}

fn adjust_setting(setting: &mut Setting) {
    let input = crate::game::globals::get_input();
    if input.get_state(&KeyNames::Left).key_repeat_triggered
        && !input.get_state(&KeyNames::Right).is_down
    {
        setting.dec();
    } else if input.get_state(&KeyNames::Right).key_repeat_triggered
        && !input.get_state(&KeyNames::Left).is_down
    {
        setting.inc();
    }
}
fn check_if_confirming() -> bool {
    let input = crate::game::globals::get_input();
    input.get_state(&KeyNames::Confirm).just_released
}
fn check_if_exiting() -> bool {
    let input = crate::game::globals::get_input();
    input.get_state(&KeyNames::Back).just_released
}
