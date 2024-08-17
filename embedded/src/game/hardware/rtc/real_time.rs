use core::cmp;

use fixedstr::str_format;

use super::meridian::Meridian;

#[derive(Copy, Clone)]
pub struct RealTime {
    pub sec: u8,
    pub min: u8,
    pub hr: u8,
}
impl RealTime {
    pub fn new(hr: u8, min: u8, sec: u8) -> Self {
        Self { sec, min, hr }
    }
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
