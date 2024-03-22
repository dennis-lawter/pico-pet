use super::page_canon::PageCanon;
use super::NVM_BLANK;

const DEFAULT_BRIGHTNESS: u8 = 15;
const DEFAULT_VOLUME: u8 = 2;
const DEFAULT_POMO_TIME: u8 = 25;
const DEFAULT_SHORT_REST_TIME: u8 = 5;
const DEFAULT_LONG_REST_TIME: u8 = 15;
const DEFAULT_POMO_CYCLE: u8 = 4;
const DEFAULT_PET_FEEDING_DEADLINE_HOUR: u8 = 22;
const DEFAULT_PET_FEEDING_DEADLINE_MINUTE: u8 = 0;

pub struct NvmSettings {
    pub system_data: [u8; 8],
    pub pomo_data: [u8; 8],
}
impl Default for NvmSettings {
    fn default() -> Self {
        Self {
            system_data: [
                DEFAULT_BRIGHTNESS,
                DEFAULT_VOLUME,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
            ],
            pomo_data: [
                DEFAULT_POMO_TIME,
                DEFAULT_SHORT_REST_TIME,
                DEFAULT_LONG_REST_TIME,
                DEFAULT_POMO_CYCLE,
                DEFAULT_PET_FEEDING_DEADLINE_HOUR,
                DEFAULT_PET_FEEDING_DEADLINE_MINUTE,
                NVM_BLANK,
                NVM_BLANK,
            ],
        }
    }
}
impl NvmSettings {
    pub fn load() -> Self {
        let hardware = crate::globals::get_hardware();
        let system_data = hardware.get_nvm_page(PageCanon::Settings1 as u16);

        let pomo_data = hardware.get_nvm_page(PageCanon::Settings2 as u16);
        Self {
            system_data,
            pomo_data,
        }
    }

    pub fn write(&mut self) {
        let hardware = crate::globals::get_hardware();

        self.update_from_globals();

        hardware.write_nvm_page(PageCanon::Settings1 as u16, &self.system_data);
        hardware.write_nvm_page(PageCanon::Settings2 as u16, &self.pomo_data);
    }

    pub fn apply_to_globals(&self) {
        unsafe { &mut crate::globals::BRIGHTNESS_SETTING }.value = self.system_data[0];
        unsafe { &mut crate::globals::VOLUME_SETTING }.value = self.system_data[1];

        unsafe { &mut crate::globals::POMO_TIME_SETTING }.value = self.pomo_data[0];
        unsafe { &mut crate::globals::SHORT_REST_TIME_SETTING }.value = self.pomo_data[1];
        unsafe { &mut crate::globals::LONG_REST_TIME_SETTING }.value = self.pomo_data[2];
        unsafe { &mut crate::globals::POMO_CYCLE_SETTING }.value = self.pomo_data[3];
        unsafe { &mut crate::globals::FEEDING_DEADLINE_HOUR_SETTING }.value = self.pomo_data[4];
        unsafe { &mut crate::globals::FEEDING_DEADLINE_MINUTE_SETTING }.value = self.pomo_data[5];
    }

    pub fn update_from_globals(&mut self) {
        self.system_data[0] = unsafe { &mut crate::globals::BRIGHTNESS_SETTING }.value;
        self.system_data[1] = unsafe { &mut crate::globals::VOLUME_SETTING }.value;

        self.pomo_data[0] = unsafe { &mut crate::globals::POMO_TIME_SETTING }.value;
        self.pomo_data[1] = unsafe { &mut crate::globals::SHORT_REST_TIME_SETTING }.value;
        self.pomo_data[2] = unsafe { &mut crate::globals::LONG_REST_TIME_SETTING }.value;
        self.pomo_data[3] = unsafe { &mut crate::globals::POMO_CYCLE_SETTING }.value;
        self.pomo_data[4] = unsafe { &mut crate::globals::FEEDING_DEADLINE_HOUR_SETTING }.value;
        self.pomo_data[5] = unsafe { &mut crate::globals::FEEDING_DEADLINE_MINUTE_SETTING }.value;
    }
}
