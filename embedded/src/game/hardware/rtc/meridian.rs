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
