use core::cmp;
// use core::ops::Sub;

use super::dow::DayOfWeek;
use super::find_day_of_week;
// use super::interval_date::IntervalDate;
use super::month::Month;

#[derive(Copy, Clone)]
pub struct RealDate {
    pub day_of_week: u8,
    pub day_of_month: u8,
    pub month: u8,
    pub year_since_2k: u8,
}
impl RealDate {
    pub fn new(day_of_month: u8, month: u8, year_since_2k: u8) -> Self {
        Self {
            day_of_week: find_day_of_week(day_of_month, Month::from(month), year_since_2k) as u8,
            day_of_month,
            month,
            year_since_2k,
        }
    }
    pub fn inc_by_1_day(&mut self) {
        self.day_of_week += 1;
        if self.day_of_week > DayOfWeek::Saturday as u8 {
            self.day_of_week = DayOfWeek::Sunday as u8;
        }
        self.day_of_month += 1;
        let month_enum = Month::from(self.month);
        if self.day_of_month > month_enum.days_in_month(self.year_since_2k) {
            self.day_of_month = 1;
            self.month += 1;
            if self.month > 12 {
                self.month = 1;
                self.year_since_2k += 1;
            }
        }
    }
    // pub fn yyyy_mm_dd_str(&self) -> fixedstr::str16 {
    //     fixedstr::str_format!(
    //         fixedstr::str16,
    //         "{:04}-{:02}-{:02}",
    //         self.year_since_2k as u16 + 2000,
    //         self.month,
    //         self.day_of_month
    //     )
    // }
}
impl PartialEq for RealDate {
    fn eq(&self, other: &Self) -> bool {
        self.day_of_month == other.day_of_month
            && self.month == other.month
            && self.year_since_2k == other.year_since_2k
    }
}
impl PartialOrd for RealDate {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.year_since_2k == other.year_since_2k {
            if self.month == other.month {
                if self.day_of_month == other.day_of_month {
                    Some(cmp::Ordering::Equal)
                } else if self.day_of_month > other.day_of_month {
                    Some(cmp::Ordering::Greater)
                } else {
                    Some(cmp::Ordering::Less)
                }
            } else if self.month > other.month {
                Some(cmp::Ordering::Greater)
            } else {
                Some(cmp::Ordering::Less)
            }
        } else if self.year_since_2k > other.year_since_2k {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Less)
        }
    }
}
// impl Sub for RealDate {
//     type Output = IntervalDate;
//     fn sub(self, other: Self) -> IntervalDate {
//         // TODO
//     }
// }
