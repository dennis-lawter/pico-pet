use super::page_canon::PageCanon;
use super::NVM_BLANK;

const DEFAULT_BIRTHDAY: u8 = 0;
const DEFAULT_BIRTHMONTH: u8 = 0;
const DEFAULT_BIRTHYEAR: u8 = 0;

const DEFAULT_CURRENT_HP: u8 = 5;
const DEFAULT_SICKNESS: u8 = 0;
const DEFAULT_DAILY_HUNGER: u8 = 1;
const DEFAULT_LAST_FED_DAY: u8 = 0;
const DEFAULT_LAST_FED_MONTH: u8 = 0;
const DEFAULT_LAST_FED_YEAR: u8 = 0;

pub struct NvmPet {
    pub pet_data: [u8; 8],
    pub health_data: [u8; 8],
}
impl Default for NvmPet {
    fn default() -> Self {
        Self {
            pet_data: [
                DEFAULT_BIRTHDAY,
                DEFAULT_BIRTHMONTH,
                DEFAULT_BIRTHYEAR,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
            ],
            health_data: [
                DEFAULT_CURRENT_HP,
                DEFAULT_SICKNESS,
                DEFAULT_DAILY_HUNGER,
                DEFAULT_LAST_FED_DAY,
                DEFAULT_LAST_FED_MONTH,
                DEFAULT_LAST_FED_YEAR,
                NVM_BLANK,
                NVM_BLANK,
            ],
        }
    }
}
impl NvmPet {
    pub fn load() -> Self {
        let hardware = crate::game::globals::get_hardware();

        let pet_data = hardware.get_nvm_page(PageCanon::Pet1 as u16);
        let health_data = hardware.get_nvm_page(PageCanon::Pet2 as u16);

        Self {
            pet_data,
            health_data,
        }
    }

    pub fn write(&mut self) {
        let hardware = crate::game::globals::get_hardware();

        hardware.write_nvm_page(PageCanon::Pet1 as u16, &self.pet_data);
        hardware.write_nvm_page(PageCanon::Pet2 as u16, &self.health_data);
    }
}
