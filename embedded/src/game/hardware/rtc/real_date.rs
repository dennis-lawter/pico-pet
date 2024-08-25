use core::cmp;

use super::find_day_of_week;
use super::month::Month;

#[derive(Copy, Clone)]
pub struct RealDate {
    pub day_of_week: u8,
    pub day_of_month: u8,
    pub month: u8,
    pub year_since_2k: u8,
}
impl RealDate {
    pub const ZERO_YEAR: u16 = 2024;

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
        self.day_of_week %= 7;

        self.day_of_month += 1;
        let month_enum = Month::from(self.month);
        if self.day_of_month > month_enum.days_in_month(self.year_since_2k) {
            self.day_of_month = 1;
            self.inc_by_1_month();
        }
    }
    pub fn inc_by_1_month(&mut self) {
        self.month += 1;
        if self.month > 12 {
            self.month = 1;
            self.inc_by_1_year();
        }
        let month_enum = Month::from(self.month);
        let max_days_in_month = month_enum.days_in_month(self.year_since_2k);
        if self.day_of_month > max_days_in_month {
            self.day_of_month = max_days_in_month;
        }
    }
    pub fn inc_by_1_year(&mut self) {
        self.year_since_2k += 1;
    }
    pub fn dec_by_1_day(&mut self) {
        self.day_of_week += 6;
        self.day_of_week %= 7;

        if self.day_of_month > 1 {
            self.day_of_month -= 1;
        } else {
            self.dec_by_1_month();
            self.day_of_month = Month::from(self.month).days_in_month(self.year_since_2k);
        }
    }
    pub fn dec_by_1_month(&mut self) {
        let month_enum: Month;
        if self.month > 1 {
            self.month -= 1;
            month_enum = Month::from(self.month);
        } else {
            month_enum = Month::December;

            self.dec_by_1_year();
        }
        let max_days_in_month = month_enum.days_in_month(self.year_since_2k);
        self.month = month_enum as u8;
        if self.day_of_month > max_days_in_month {
            self.day_of_month = max_days_in_month;
        }
    }
    pub fn dec_by_1_year(&mut self) {
        if self.year_since_2k > 0 {
            self.year_since_2k -= 1;
        } else {
            // Underflow protection, reset to y2k day 0 (jan 0, 2024)
            self.year_since_2k = 0;
            self.month = Month::January as u8;
            self.day_of_month = 0;
        }
    }
    pub fn yyyy_mm_dd_str(&self) -> fixedstr::str12 {
        fixedstr::str_format!(
            fixedstr::str12,
            "{:04}-{:02}-{:02}",
            self.year_since_2k as u16 + Self::ZERO_YEAR,
            self.month,
            self.day_of_month,
        )
    }
    pub fn yyyy_mmm_dd_str(&self) -> fixedstr::str12 {
        let month = Month::from(self.month);
        fixedstr::str_format!(
            fixedstr::str12,
            "{:04} {} {:02}",
            self.year_since_2k as u16 + Self::ZERO_YEAR,
            month.to_abbrev(),
            self.day_of_month,
        )
    }
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
