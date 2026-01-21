pub mod dow;
pub mod interval_date_time;
pub mod meridian;
pub mod month;
pub mod real_date;
pub mod real_date_time;
pub mod real_time;

pub use self::real_date::RealDate;
pub use self::real_date_time::RealDateTime;
// pub use self::real_time::RealTime;

use self::dow::DayOfWeek;
use self::month::Month;

pub fn bcd_to_dec(n: u8) -> u8 {
    (n / 16) * 10 + (n % 16)
}
pub fn dec_to_bcd(n: u8) -> u8 {
    (n / 10) * 16 + (n % 10)
}

fn is_leap_year(years_since_2k: u8) -> bool {
    // This is not accurate, but it does match the RTC's behavior
    years_since_2k % 4 == 0
}

fn find_day_of_week(day: u8, month: Month, year: u8) -> DayOfWeek {
    let y2k_anchor_day = DayOfWeek::Tuesday as i16;
    let anchor_day =
        y2k_anchor_day as i16 + (year / 12) as i16 + (year % 12) as i16 + ((year % 12) / 4) as i16;
    let anchor_day = anchor_day % 7;

    let is_leap_year = is_leap_year(year);

    let first_doomsday: i16 = match month {
        Month::January => {
            if is_leap_year {
                3
            } else {
                4
            }
        }
        Month::February => {
            if is_leap_year {
                7
            } else {
                1
            }
        }
        Month::March => 7,
        Month::April => 4,
        Month::May => 2,
        Month::June => 6,
        Month::July => 4,
        Month::August => 1,
        Month::September => 5,
        Month::October => 3,
        Month::November => 7,
        Month::December => 5,
    };

    let difference_from_doomsday = day as i16 - first_doomsday;
    let dow = ((anchor_day + difference_from_doomsday) % 7) as u8;
    DayOfWeek::from(dow)
}
