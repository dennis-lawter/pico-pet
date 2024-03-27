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

#[derive(Clone)]
pub struct RealTime {
    pub sec: u8,
    pub min: u8,
    pub hr: u8,
}
impl RealTime {
    pub fn bcd_to_dec(n: u8) -> u8 {
        (n / 16) * 10 + (n % 16)
    }
    pub fn dec_to_bcd(n: u8) -> u8 {
        (n / 10) * 16 + (n % 10)
    }
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
