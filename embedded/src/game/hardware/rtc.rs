use core::cmp;

use fixedstr::str_format;

pub enum Meridian {
    Am,
    Pm,
}
impl Meridian {
    pub fn to_cap_str2(&self) -> &str {
        match self {
            Self::Am => "AM",
            Self::Pm => "PM",
        }
    }
}

pub fn bcd_to_dec(n: u8) -> u8 {
    (n / 16) * 10 + (n % 16)
}
pub fn dec_to_bcd(n: u8) -> u8 {
    (n / 10) * 16 + (n % 10)
}

#[derive(Copy, Clone)]
pub struct RealTime {
    pub sec: u8,
    pub min: u8,
    pub hr: u8,
}
impl RealTime {
    // pub fn new(hr: u8, min: u8, sec: u8) -> Self {
    //     Self { sec, min, hr }
    // }
    pub fn get_meridian_hour(&self) -> u8 {
        if self.hr == 00 {
            12
        } else if self.hr > 12 {
            self.hr - 12
        } else {
            self.hr
        }
    }
    pub fn get_meridian(&self) -> Meridian {
        if self.hr <= 11 {
            Meridian::Am
        } else {
            Meridian::Pm
        }
    }
    pub fn hh_mm_str(&self) -> fixedstr::str8 {
        let hr = self.get_meridian_hour();
        let meridian = self.get_meridian();
        let meridian_str = meridian.to_cap_str2();
        str_format!(fixedstr::str8, "{:>2}:{:02}{}", hr, self.min, meridian_str)
    }
}
impl PartialEq for RealTime {
    fn eq(&self, other: &Self) -> bool {
        self.sec == other.sec && self.min == other.min && self.hr == other.hr
    }
}
impl PartialOrd for RealTime {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.hr == other.hr {
            if self.min == other.min {
                if self.sec == other.sec {
                    Some(cmp::Ordering::Equal)
                } else if self.sec > other.sec {
                    Some(cmp::Ordering::Greater)
                } else {
                    Some(cmp::Ordering::Less)
                }
            } else if self.min > other.min {
                Some(cmp::Ordering::Greater)
            } else {
                Some(cmp::Ordering::Less)
            }
        } else if self.hr > other.hr {
            Some(cmp::Ordering::Greater)
        } else {
            Some(cmp::Ordering::Less)
        }
    }
}

#[allow(dead_code)]
#[repr(u8)]
enum DayOfWeek {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}

#[allow(dead_code)]
#[repr(u8)]
enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}
impl Month {
    pub fn is_leap_year(&self, years_since_2k: u8) -> bool {
        // This is not accurate, but it does match the RTC's behavior
        years_since_2k % 4 == 0
    }
    pub fn days_in_month(&self, years_since_2k: u8) -> u8 {
        match self {
            Self::January
            | Self::March
            | Self::May
            | Self::July
            | Self::August
            | Self::October
            | Self::December => 31,
            Self::February => {
                if self.is_leap_year(years_since_2k) {
                    29
                } else {
                    28
                }
            }
            Self::April | Self::June | Self::September | Self::November => 30,
        }
    }

    fn from_u8(month: u8) -> Self {
        match month {
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            12 => Self::December,
            _ => panic!("Invalid month"), // TODO: graceful error
        }
    }
}

#[derive(Copy, Clone)]
pub struct RealDate {
    pub day_of_week: u8,
    pub day_of_month: u8,
    pub month: u8,
    pub year_since_2k: u8,
}
impl RealDate {
    pub fn add_day(&mut self) {
        self.day_of_week += 1;
        if self.day_of_week > DayOfWeek::Saturday as u8 {
            self.day_of_week = DayOfWeek::Sunday as u8;
        }
        self.day_of_month += 1;
        let month_enum = Month::from_u8(self.month);
        if self.day_of_month > month_enum.days_in_month(self.year_since_2k) {
            self.day_of_month = 1;
            self.month += 1;
            if self.month > 12 {
                self.month = 1;
                self.year_since_2k += 1;
            }
        }
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
            let days_in_month = Month::from_u8(month).days_in_month(self.date.year_since_2k);
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
