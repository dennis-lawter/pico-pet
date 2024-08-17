pub struct IntervalDateTime {
    pub seconds: i32,
}
impl IntervalDateTime {
    pub fn from_epoch(epoch: i32) -> Self {
        let seconds = epoch;
        Self { seconds }
    }
    pub fn to_str(&self) -> fixedstr::str24 {
        let negation_char = if self.seconds < 0 { '-' } else { '+' };
        let abs_secs = self.seconds.abs() as u32;
        fixedstr::str_format!(
            fixedstr::str24,
            "{} {}d {:02}h {:02}m {:02}s",
            negation_char,
            abs_secs / (60 * 60 * 24),
            (abs_secs % (60 * 60 * 24)) / (60 * 60),
            (abs_secs % (60 * 60)) / 60,
            abs_secs % 60,
        )
    }
}
