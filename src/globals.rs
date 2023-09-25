use crate::{
    hardware::{hardware::HardwareComponents, input::InputHandler},
    nvm::{inventory::Inventory, Nvm},
    rand::Lcg,
    setting_value::Setting,
    states::farm_state::farm_garden::FarmGarden,
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

pub static mut GARDEN: Option<FarmGarden> = None;
pub fn init_garden() {
    unsafe { self::GARDEN = Some(FarmGarden::default()) }
}
pub fn get_garden() -> &'static mut FarmGarden<'static> {
    unsafe { self::GARDEN.as_mut().unwrap() }
}

pub static mut RNG: Option<Lcg> = None;
pub fn init_rng() {
    unsafe { self::RNG = Some(Lcg::default()) }
}
pub fn get_rng() -> &'static mut Lcg {
    unsafe { self::RNG.as_mut().unwrap() }
}

pub static mut INVENTORY: Option<Inventory> = None;
pub fn init_inv() {
    unsafe { self::INVENTORY = Some(Inventory::default()) }
}
pub fn get_inv() -> &'static mut Inventory {
    unsafe { self::INVENTORY.as_mut().unwrap() }
}
