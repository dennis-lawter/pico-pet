use super::page_canon::PageCanon;
use super::NVM_BLANK;

const DEFAULT_TOMATOES: u8 = 0;
const DEFAULT_RASPBERRIES: u8 = 0;
const DEFAULT_JUICE_UPPER: u8 = 0;
const DEFAULT_JUICE_LOWER: u8 = 0;

pub struct NvmInventory {
    pub data: [u8; 8],
}
impl Default for NvmInventory {
    fn default() -> Self {
        Self {
            data: [
                DEFAULT_TOMATOES,    //
                DEFAULT_RASPBERRIES, //
                DEFAULT_JUICE_LOWER, //
                DEFAULT_JUICE_UPPER, //
                NVM_BLANK,           //
                NVM_BLANK,           //
                NVM_BLANK,           //
                NVM_BLANK,           //
            ],
        }
    }
}
impl NvmInventory {
    pub fn load() -> Self {
        let hardware = crate::globals::get_hardware();
        let data = hardware.get_nvm_page(PageCanon::Inventory1.into());

        Self { data }
    }

    pub fn write(&mut self) {
        let hardware = crate::globals::get_hardware();

        // self.update_from_globals();

        hardware.write_nvm_page(PageCanon::Inventory1.into(), &self.data);
    }

    pub fn get_tomatoes(&self) -> u8 {
        self.data[0]
    }
    pub fn get_raspberries(&self) -> u8 {
        self.data[1]
    }
    pub fn get_juice(&self) -> u16 {
        (self.data[2] as u16) << 8 | self.data[3] as u16
    }
    pub fn set_juice(&mut self, juice: u16) {
        self.data[2] = (juice >> 8) as u8;
        self.data[3] = juice as u8;
    }
    // pub fn set_tomatoes(&mut self, tomatoes: u8) {
    //     self.data[0] = tomatoes;
    // }
    // pub fn set_raspberries(&mut self, raspberries: u8) {
    //     self.data[1] = raspberries;
    // }

    pub fn inc_tomatoes(&mut self) {
        self.data[0] += 1;
    }
    pub fn inc_raspberries(&mut self) {
        self.data[1] += 1;
    }
    pub fn inc_juice(&mut self) {
        let mut juice = self.get_juice();
        juice += 1;
        self.set_juice(juice);
    }
}
