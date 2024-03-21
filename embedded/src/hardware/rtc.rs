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
}
