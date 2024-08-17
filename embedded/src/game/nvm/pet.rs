use crate::game::hardware::rtc::real_date::RealDate;
use crate::game::hardware::rtc::real_date_time::RealDateTime;
use crate::game::hardware::rtc::real_time::RealTime;

use super::page_canon::PageCanon;
use super::NVM_BLANK;

const DEFAULT_BIRTHDAY: u8 = 0;
const DEFAULT_BIRTHMONTH: u8 = 0;
const DEFAULT_BIRTHYEAR: u8 = 0;

const DEFAULT_CURRENT_HP: u8 = 5;
const DEFAULT_SICKNESS: u8 = 0;
const DEFAULT_DAILY_HUNGER: u8 = 1;
const DEFAULT_LAST_FED_DAY: u8 = 1;
const DEFAULT_LAST_FED_MONTH: u8 = 1;
const DEFAULT_LAST_FED_YEAR: u8 = 0;

pub struct NvmPet {
    pub pet_data: [u8; 8],
    pub health_data: [u8; 8],

    // temp values
    pub is_hungry: bool,
    pub is_starved: bool,
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
            is_hungry: false,
            is_starved: false,
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

            is_hungry: false,
            is_starved: false,
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

    pub fn get_feeding_deadline(&self) -> RealDateTime {
        let nvm = crate::game::globals::get_nvm();

        let last_fed = self.get_last_fed_date();
        let mut feed_deadline = last_fed.clone();
        feed_deadline.inc_by_1_day();

        let (feeding_deadline_hr, feeding_deadline_min) = nvm.settings.get_feeding_deadline();
        RealDateTime::new(
            RealTime::new(feeding_deadline_hr, feeding_deadline_min, 0),
            feed_deadline,
        )
    }
    #[allow(dead_code)]
    pub fn get_last_fed_date(&self) -> RealDate {
        RealDate::new(
            self.health_data[3],
            self.health_data[4],
            self.health_data[5],
        )
    }
    #[allow(dead_code)]
    pub fn set_last_fed_date(&mut self, date: RealDate) {
        self.health_data[3] = date.day_of_month;
        self.health_data[4] = date.month;
        self.health_data[5] = date.year_since_2k;
    }
}
