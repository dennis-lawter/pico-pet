pub mod brightness_setting_component;
pub mod time_setting_component;
pub mod volume_setting_component;
pub use self::brightness_setting_component::BrightnessSettingComponent;
pub use self::time_setting_component::TimeSettingComponent;
pub use self::volume_setting_component::VolumeSettingComponent;

pub enum SettingComponent {
    Brightness(BrightnessSettingComponent),
    Volume(VolumeSettingComponent),
    Time(TimeSettingComponent),
}
impl SettingComponent {
    pub fn draw(&mut self, y_offset: i32, selected: bool) {
        match self {
            SettingComponent::Brightness(component) => component.draw(y_offset, selected),
            SettingComponent::Volume(component) => component.draw(y_offset, selected),
            SettingComponent::Time(component) => component.draw(y_offset, selected),
        }
    }

    pub fn input(&mut self) {
        match self {
            SettingComponent::Brightness(component) => component.input(),
            SettingComponent::Volume(component) => component.input(),
            SettingComponent::Time(component) => component.input(),
        }
    }
}

trait SettingComponentTrait {
    fn draw(&mut self, y_offset: i32, selected: bool);

    fn input(&mut self);
}
