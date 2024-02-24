use super::{page_canon::PageCanon, NVM_BLANK};

const DEFAULT_BRIGHTNESS: u8 = 15;
const DEFAULT_VOLUME: u8 = 2;
const DEFAULT_POMO_TIME: u8 = 25;
const DEFAULT_SHORT_BREAK_TIME: u8 = 5;
const DEFAULT_LONG_BREAK_TIME: u8 = 15;
const DEFAULT_POMO_CYCLE: u8 = 4;

pub struct NvmSettings {
    pub data: [u8; 8],
}
impl Default for NvmSettings {
    fn default() -> Self {
        Self {
            data: [
                DEFAULT_BRIGHTNESS,
                DEFAULT_VOLUME,
                DEFAULT_POMO_TIME,
                DEFAULT_SHORT_BREAK_TIME,
                DEFAULT_LONG_BREAK_TIME,
                DEFAULT_POMO_CYCLE,
                NVM_BLANK,
                NVM_BLANK,
            ],
        }
    }
}
impl NvmSettings {
    pub fn load() -> Self {
        let hardware = crate::globals::get_hardware();
        let data = hardware.get_nvm_page(PageCanon::Settings.into());
        Self { data }
    }

    pub fn write(&mut self) {
        let hardware = crate::globals::get_hardware();

        self.update_from_globals();

        hardware.write_nvm_page(PageCanon::Settings.into(), &self.data);
    }

    pub fn apply_to_globals(&self) {
        unsafe { &mut crate::globals::BRIGHTNESS_SETTING }.value = self.data[0];
        unsafe { &mut crate::globals::VOLUME_SETTING }.value = self.data[1];
        unsafe { &mut crate::globals::POMO_TIME_SETTING }.value = self.data[2];
        unsafe { &mut crate::globals::SHORT_BREAK_TIME_SETTING }.value = self.data[3];
        unsafe { &mut crate::globals::LONG_BREAK_TIME_SETTING }.value = self.data[4];
        unsafe { &mut crate::globals::POMO_CYCLE_SETTING }.value = self.data[5];
    }

    pub fn update_from_globals(&mut self) {
        self.data[0] = unsafe { &mut crate::globals::BRIGHTNESS_SETTING }.value;
        self.data[1] = unsafe { &mut crate::globals::VOLUME_SETTING }.value;
        self.data[2] = unsafe { &mut crate::globals::POMO_TIME_SETTING }.value;
        self.data[3] = unsafe { &mut crate::globals::SHORT_BREAK_TIME_SETTING }.value;
        self.data[4] = unsafe { &mut crate::globals::LONG_BREAK_TIME_SETTING }.value;
        self.data[5] = unsafe { &mut crate::globals::POMO_CYCLE_SETTING }.value;
    }
}
