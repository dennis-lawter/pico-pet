use crate::hardware::hardware::HardwareComponents;
use crate::hardware::input::InputHandler;
use crate::nvm::Nvm;
// use crate::rand::Lcg;
use crate::setting_value::Setting;

pub static mut BRIGHTNESS_SETTING: Setting = Setting {
    value: 15,
    min_value: 0,
    max_value: 15,
};
pub static mut VOLUME_SETTING: Setting = Setting {
    value: 2,
    min_value: 0,
    max_value: 4,
};
pub static mut POMO_TIME_SETTING: Setting = Setting {
    value: 25,
    min_value: 1,
    max_value: 90,
};
pub static mut SHORT_REST_TIME_SETTING: Setting = Setting {
    value: 5,
    min_value: 1,
    max_value: 90,
};
pub static mut LONG_REST_TIME_SETTING: Setting = Setting {
    value: 15,
    min_value: 1,
    max_value: 90,
};
pub static mut POMO_CYCLE_SETTING: Setting = Setting {
    value: 4,
    min_value: 1,
    max_value: 9,
};

pub static mut HARDWARE: Option<HardwareComponents> = None;
pub fn init_hardware() {
    unsafe { self::HARDWARE = Some(HardwareComponents::new()) }
}
pub fn get_hardware() -> &'static mut HardwareComponents {
    unsafe { self::HARDWARE.as_mut().unwrap() }
}

pub static mut INPUT: Option<InputHandler> = None;
pub fn init_input() {
    unsafe { self::INPUT = Some(InputHandler::default()) }
}
pub fn get_input() -> &'static mut InputHandler {
    unsafe { self::INPUT.as_mut().unwrap() }
}

pub static mut NVM: Option<Nvm> = None;
pub fn init_nvm() {
    unsafe { self::NVM = Some(Nvm::load_or_write_default()) }
}
pub fn get_nvm() -> &'static mut Nvm {
    unsafe { self::NVM.as_mut().unwrap() }
}

// pub static mut RNG: Option<Lcg> = None;
// pub fn init_rng() {
//     unsafe { self::RNG = Some(Lcg::default()) }
// }
// pub fn get_rng() -> &'static mut Lcg {
//     unsafe { self::RNG.as_mut().unwrap() }
// }
