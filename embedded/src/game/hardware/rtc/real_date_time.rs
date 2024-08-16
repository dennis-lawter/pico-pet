use core::cmp;

use super::month::Month;
use super::real_date::RealDate;
use super::real_time::RealTime;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct RealDateTime {
    pub time: RealTime,
    pub date: RealDate,
}
impl RealDateTime {
    #[allow(dead_code)]
    pub fn new(time: RealTime, date: RealDate) -> Self {
        Self { time, date }
    }

    pub fn to_y2k_epoch(&self) -> u32 {
        let mut month_seconds = 0;
        for month in 1..self.date.month {
            let days_in_month = Month::from(month).days_in_month(self.date.year_since_2k);
            month_seconds += days_in_month as u32 * (60 * 60 * 24);
        }
        self.time.sec as u32
            + self.time.min as u32 * 60
            + self.time.hr as u32 * (60 * 60)
            + self.date.day_of_month as u32 * (60 * 60 * 24)
            + month_seconds
    }
}
impl PartialEq for RealDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.date == other.date
    }
}
impl PartialOrd for RealDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let my_epoch = self.to_y2k_epoch();
        let other_epoch = other.to_y2k_epoch();
        Some(my_epoch.cmp(&other_epoch))
        // if self.date == other.date {
        //     if self.time == other.time {
        //         Some(cmp::Ordering::Equal)
        //     } else if self.time > other.time {
        //         Some(cmp::Ordering::Greater)
        //     } else {
        //         Some(cmp::Ordering::Less)
        //     }
        // } else if self.date > other.date {
        //     Some(cmp::Ordering::Greater)
        // } else {
        //     Some(cmp::Ordering::Less)
        // }
    }
}
