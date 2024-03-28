use crate::setting_value::Setting;

use super::page_canon::PageCanon;
use super::NVM_BLANK;

struct SettingValues {
    min: u8,
    max: u8,
    default: u8,
}

pub enum SettingType {
    Brightness,
    Volume,
    Vibration,

    PomodoroMinutes,
    ShortRestMinutes,
    LongRestMinutes,
    PomodoroCycles,
    FeedingDeadlineHour,
    FeedingDeadlineMinute,
}
impl SettingType {
    fn to_setting_value(&self) -> SettingValues {
        match self {
            SettingType::Brightness => SettingValues {
                min: 0,
                max: 15,
                default: 15,
            },
            SettingType::Volume => SettingValues {
                min: 0,
                max: 4,
                default: 2,
            },
            SettingType::Vibration => SettingValues {
                min: 0,
                max: 1,
                default: 1,
            },

            SettingType::PomodoroMinutes => SettingValues {
                min: 1,
                max: 90,
                default: 25,
            },
            SettingType::ShortRestMinutes => SettingValues {
                min: 1,
                max: 90,
                default: 5,
            },
            SettingType::LongRestMinutes => SettingValues {
                min: 1,
                max: 90,
                default: 15,
            },
            SettingType::PomodoroCycles => SettingValues {
                min: 1,
                max: 9,
                default: 4,
            },
            SettingType::FeedingDeadlineHour => SettingValues {
                min: 0,
                max: 23,
                default: 22,
            },
            SettingType::FeedingDeadlineMinute => SettingValues {
                min: 0,
                max: 59,
                default: 0,
            },
        }
    }
}

pub struct NvmSettings {
    pub system_data: [u8; 8],
    pub pomo_data: [u8; 8],
}
impl Default for NvmSettings {
    fn default() -> Self {
        Self {
            system_data: [
                SettingType::Brightness.to_setting_value().default,
                SettingType::Volume.to_setting_value().default,
                SettingType::Vibration.to_setting_value().default,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
            ],
            pomo_data: [
                SettingType::PomodoroMinutes.to_setting_value().default,
                SettingType::ShortRestMinutes.to_setting_value().default,
                SettingType::LongRestMinutes.to_setting_value().default,
                SettingType::PomodoroCycles.to_setting_value().default,
                SettingType::FeedingDeadlineHour.to_setting_value().default,
                SettingType::FeedingDeadlineMinute
                    .to_setting_value()
                    .default,
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

        hardware.write_nvm_page(PageCanon::Settings1 as u16, &self.system_data);
        hardware.write_nvm_page(PageCanon::Settings2 as u16, &self.pomo_data);
    }

    pub fn get_setting(&self, setting_type: SettingType) -> Setting {
        let min_value = setting_type.to_setting_value().min;
        let max_value = setting_type.to_setting_value().max;
        let value = match setting_type {
            SettingType::Brightness => self.system_data[0],
            SettingType::Volume => self.system_data[1],
            SettingType::Vibration => self.system_data[2],

            SettingType::PomodoroMinutes => self.pomo_data[0],
            SettingType::ShortRestMinutes => self.pomo_data[1],
            SettingType::LongRestMinutes => self.pomo_data[2],
            SettingType::PomodoroCycles => self.pomo_data[3],
            SettingType::FeedingDeadlineHour => self.pomo_data[4],
            SettingType::FeedingDeadlineMinute => self.pomo_data[5],
        };

        Setting {
            value,
            min_value,
            max_value,
        }
    }

    pub fn set_value(&mut self, setting_type: SettingType, value: u8) {
        match setting_type {
            SettingType::Brightness => self.system_data[0] = value,
            SettingType::Volume => self.system_data[1] = value,
            SettingType::Vibration => self.system_data[2] = value,

            SettingType::PomodoroMinutes => self.pomo_data[0] = value,
            SettingType::ShortRestMinutes => self.pomo_data[1] = value,
            SettingType::LongRestMinutes => self.pomo_data[2] = value,
            SettingType::PomodoroCycles => self.pomo_data[3] = value,
            SettingType::FeedingDeadlineHour => self.pomo_data[4] = value,
            SettingType::FeedingDeadlineMinute => self.pomo_data[5] = value,
        }
    }
}
