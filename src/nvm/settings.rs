use super::NVM_BLANK;

const NVM_SETTINGS_PAGE: u16 = 0x001;

const DEFAULT_BRIGHTNESS: u8 = 15;
const DEFAULT_VOLUME: u8 = 2;

pub struct NvmSettings {
    pub data: [u8; 8],
}
impl Default for NvmSettings {
    fn default() -> Self {
        Self {
            data: [
                DEFAULT_BRIGHTNESS,
                DEFAULT_VOLUME,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
            ],
        }
    }
}
impl NvmSettings {
    pub fn load() -> Self {
        let hardware = crate::globals::get_hardware();
        let data = hardware.get_nvm_page(NVM_SETTINGS_PAGE);
        Self { data }
    }

    pub fn write(&mut self) {
        let hardware = crate::globals::get_hardware();

        self.update_from_globals();

        hardware.write_nvm_page(NVM_SETTINGS_PAGE, &self.data);
    }

    pub fn apply_to_globals(&self) {
        unsafe { &mut crate::globals::BRIGHTNESS_SETTING }.value = self.data[0];
        unsafe { &mut crate::globals::VOLUME_SETTING }.value = self.data[1];
    }

    pub fn update_from_globals(&mut self) {
        self.data[0] = unsafe { &mut crate::globals::BRIGHTNESS_SETTING }.value;
        self.data[1] = unsafe { &mut crate::globals::VOLUME_SETTING }.value;
    }
}
