use crate::{hardware::HardwareComponents, setting_value::Setting};

pub static mut BRIGHTNESS_SETTING: Setting = Setting {
    value: 15,
    max_value: 15,
};
pub static mut VOLUME_SETTING: Setting = Setting {
    value: 2,
    max_value: 4,
};
pub static mut HARDWARE: Option<HardwareComponents> = None;
