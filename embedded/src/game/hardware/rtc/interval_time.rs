pub struct IntervalTime {
    pub neg: bool,
    pub hr: u8,
    pub min: u8,
    pub sec: u8,
}
impl IntervalTime {
    pub fn to_str(&self) -> fixedstr::str12 {
        let negation_char = if self.neg { '-' } else { '+' };
        fixedstr::str_format!(fixedstr::str12, "{} {}", negation_char, self.to_abs_str())
    }
    pub fn to_abs_str(&self) -> fixedstr::str12 {
        fixedstr::str_format!(
            fixedstr::str12,
            "{:02}:{:02}:{:02}",
            self.hr,
            self.min,
            self.sec,
        )
    }
}
