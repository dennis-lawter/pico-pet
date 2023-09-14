pub mod header;
pub mod settings;

pub use self::header::NvmHeader;
pub use self::settings::NvmSettings;

pub const NVM_BLANK: u8 = 0xff;
pub const NUM_PAGES_IN_USE: u16 = 2;

pub struct Nvm {
    pub parity: NvmHeader,
    pub settings: NvmSettings,
}
impl Default for Nvm {
    fn default() -> Self {
        Self {
            parity: NvmHeader::default(),
            settings: NvmSettings::default(),
        }
    }
}
impl Nvm {
    pub fn load_or_write_default() -> Self {
        match NvmHeader::try_load() {
            Some(parity) => {
                let new_nvm = Self {
                    parity,
                    settings: NvmSettings::load(),
                };

                new_nvm.settings.apply_to_globals();

                new_nvm
            }
            None => {
                let mut new_nvm = Self::default();

                new_nvm.write_all();

                new_nvm.settings.apply_to_globals();

                new_nvm
            }
        }
    }

    pub fn write_all(&mut self) {
        self.parity.write();
        self.settings.write();
    }

    pub fn erase_all_then_reboot(&mut self) {
        let hardware = crate::globals::get_hardware();

        let blank_data_buffer = [NVM_BLANK; 8];
        for page in 0..NUM_PAGES_IN_USE {
            hardware.write_nvm_page(page, &blank_data_buffer);
        }

        crate::reboot();
    }
}
