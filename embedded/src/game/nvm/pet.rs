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

    pub fn get_health(&self) -> u8 {
        self.health_data[0]
    }
    #[allow(dead_code)]
    pub fn set_health(&mut self, health: u8) {
        self.health_data[0] = health;
    }

    #[allow(dead_code)]
    pub fn get_sickness(&self) -> u8 {
        self.health_data[1]
    }
    #[allow(dead_code)]
    pub fn set_sickness(&mut self, sickness: u8) {
        self.health_data[1] = sickness;
    }

    #[allow(dead_code)]
    pub fn get_max_health(&self) -> u8 {
        self.health_data[2] + 4
    }
    #[allow(dead_code)]
    pub fn get_daily_hunger(&self) -> u8 {
        self.health_data[2]
    }
    #[allow(dead_code)]
    pub fn set_daily_hunger(&mut self, hunger: u8) {
        self.health_data[2] = hunger;
    }

    #[allow(dead_code)]
    pub fn get_last_fed_time(&self) -> (u8, u8, u8) {
        (
            self.health_data[3],
            self.health_data[4],
            self.health_data[5],
        )
    }
    #[allow(dead_code)]
    pub fn set_last_fed_time(&mut self, time: (u8, u8, u8)) {
        self.health_data[3] = time.0;
        self.health_data[4] = time.1;
        self.health_data[5] = time.2;
    }
}
