pub enum Meridian {
    Am,
    Pm,
}
impl Meridian {
    pub fn to_cap_str2(&self) -> fixedstr::str4 {
        match self {
            Self::Am => fixedstr::str4::from("AM"),
            Self::Pm => fixedstr::str4::from("PM"),
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
}
