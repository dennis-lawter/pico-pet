use crate::game::display;
use crate::game::hardware::hardware::HardwareComponents;
use crate::game::hardware::input::InputHandler;
use crate::game::nvm::Nvm;

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

pub fn init_globals() {
    self::init_hardware();
    self::init_nvm();
    self::init_input();

    display::text_writer::init_singleton_fonts();
}
