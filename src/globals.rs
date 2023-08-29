use crate::setting_value::{Setting, SETTING_MAX};

pub static mut BRIGHTNESS_SETTING: Setting = Setting {
    value: SETTING_MAX,
    step_size: 1,
};
pub static mut VOLUME_SETTING: Setting = Setting {
    value: SETTING_MAX - 9,
    step_size: 3,
};
