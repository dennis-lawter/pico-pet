pub struct IntervalDate {
    pub neg: bool,
    pub year: u8,
    pub month: u8,
    pub day: u8,
}
impl IntervalDate {
    pub fn to_str(&self) -> fixedstr::str16 {
        let negation_char = if self.neg { '-' } else { '+' };
        fixedstr::str_format!(fixedstr::str16, "{} {}", negation_char, self.to_abs_str())
    }
    pub fn to_abs_str(&self) -> fixedstr::str16 {
        fixedstr::str_format!(
            fixedstr::str16,
            "{:04}-{:02}-{:02}",
            self.year as u16 + 2000,
            self.month,
            self.day,
        )
    }
}
