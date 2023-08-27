use crate::setting_value::{self, Setting};

pub static mut BRIGHTNESS_SETTING: Setting = Setting {
    value: setting_value::SETTING_MAX,
};
