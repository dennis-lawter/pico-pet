#[derive(Clone, PartialEq)]
pub enum SettingSelected {
    Brightness,
    Volume,
    Time,
    None,
}
impl SettingSelected {
    pub fn prev(&self) -> Self {
        match self {
            Self::Brightness => Self::Time,
            Self::Volume => Self::Brightness,
            Self::Time => Self::Volume,
            Self::None => Self::Time,
        }
    }
    pub fn next(&self) -> Self {
        match self {
            Self::Brightness => Self::Volume,
            Self::Volume => Self::Time,
            Self::Time => Self::Brightness,
            Self::None => Self::Brightness,
        }
    }
}
