use super::NVM_BLANK;

const NVM_PARITY_PAGE: u16 = 0x000;

const NVM_SENTINEL: u8 = 0x69;

pub struct NvmParity {
    pub data: [u8; 8],
}
impl Default for NvmParity {
    fn default() -> Self {
        Self {
            data: [
                NVM_SENTINEL,
                NVM_BLANK,
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
impl NvmParity {
    pub fn try_load() -> Option<Self> {
        let hardware = crate::globals::get_hardware();
        let data = hardware.get_nvm_page(NVM_PARITY_PAGE);
        if data[0] == NVM_SENTINEL {
            Some(Self { data })
        } else {
            None
        }
    }

    pub fn write(&self) {
        let hardware = crate::globals::get_hardware();
        hardware.write_nvm_page(NVM_PARITY_PAGE, &self.data);
    }
}
