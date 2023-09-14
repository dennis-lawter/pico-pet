#[derive(Clone, PartialEq)]
pub enum SettingSelected {
    Brightness,
    Volume,
    Time,
    Reset,
    None,
}
impl SettingSelected {
    pub fn prev(&self) -> Self {
        match self {
            Self::Brightness => Self::Reset, // loop to bottom
            Self::Volume => Self::Brightness,
            Self::Time => Self::Volume,
            Self::Reset => Self::Time,

            Self::None => Self::Reset, // first press to last option
        }
    }
    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::Brightness,

            Self::Brightness => Self::Volume,
            Self::Volume => Self::Time,
            Self::Time => Self::Reset,
            Self::Reset => Self::Brightness,
        }
    }
}
