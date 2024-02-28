pub mod brightness_setting_component;
pub mod reset_setting_component;
pub mod time_setting_component;
pub mod volume_setting_component;
pub use self::brightness_setting_component::BrightnessSettingComponent;
pub use self::reset_setting_component::ResetSettingComponent;
pub use self::time_setting_component::TimeSettingComponent;
pub use self::volume_setting_component::VolumeSettingComponent;

pub enum SettingComponent {
    Brightness(BrightnessSettingComponent),
    Volume(VolumeSettingComponent),
    Time(TimeSettingComponent),
    Reset(ResetSettingComponent),
}
impl SettingComponent {
    pub fn draw(&mut self, y_offset: i32, selected: bool) {
        match self {
            SettingComponent::Brightness(component) => component.draw(y_offset, selected),
            SettingComponent::Volume(component) => component.draw(y_offset, selected),
            SettingComponent::Time(component) => component.draw(y_offset, selected),
            SettingComponent::Reset(component) => component.draw(y_offset, selected),
        }
    }

    pub fn tick(&mut self) {
        match self {
            SettingComponent::Brightness(component) => component.tick(),
            SettingComponent::Volume(component) => component.tick(),
            SettingComponent::Time(component) => component.tick(),
            SettingComponent::Reset(component) => component.tick(),
        }
    }

    pub fn input(&mut self) {
        match self {
            SettingComponent::Brightness(component) => component.input(),
            SettingComponent::Volume(component) => component.input(),
            SettingComponent::Time(component) => component.input(),
            SettingComponent::Reset(component) => component.input(),
        }
    }

    pub fn is_deselected(&mut self) -> bool {
        match self {
            SettingComponent::Brightness(component) => component.is_deselected(),
            SettingComponent::Volume(component) => component.is_deselected(),
            SettingComponent::Time(component) => component.is_deselected(),
            SettingComponent::Reset(component) => component.is_deselected(),
        }
    }
}

trait SettingComponentTrait {
    fn draw(&mut self, y_offset: i32, selected: bool);
    fn tick(&mut self);
    fn input(&mut self);
    fn is_deselected(&mut self) -> bool;
}
