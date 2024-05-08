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

#[derive(Clone)]
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

#[allow(dead_code)]
enum DayOfWeek {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}

#[derive(Clone)]
pub struct RealDate {
    pub day_of_week: u8,
    pub day_of_month: u8,
    pub month: u8,
    pub year_since_2k: u8,
}
impl RealDate {}

#[allow(dead_code)]
#[derive(Clone)]
pub struct RealDateTime {
    pub real_time: RealTime,
    pub real_date: RealDate,
}
impl RealDateTime {
    #[allow(dead_code)]
    pub fn new(real_time: RealTime, real_date: RealDate) -> Self {
        Self {
            real_time,
            real_date,
        }
    }
}
