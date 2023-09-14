use crate::{
    hardware::{hardware::HardwareComponents, input::InputHandler},
    nvm::Nvm,
    setting_value::Setting,
};

pub static mut BRIGHTNESS_SETTING: Setting = Setting {
    value: 15,
    max_value: 15,
};
pub static mut VOLUME_SETTING: Setting = Setting {
    value: 2,
    max_value: 4,
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
