#[derive(Clone, Copy, PartialEq)]
pub enum SettingSelected {
    Brightness = 0,
    Volume,
    Time,
    PomoTime,
    PomoCycle,
    Reset,
    None = 255,
}

impl SettingSelected {
    const MAX_VALUE: u8 = SettingSelected::Reset as u8;
    pub fn prev(&self) -> Self {
        match self {
            Self::Brightness => Self::Reset, // loop to bottom
            // Self::Volume => Self::Brightness,
            // Self::Time => Self::Volume,
            // Self::Pomo => Self::Time,
            // Self::Reset => Self::Pomo,
            Self::None => Self::Reset, // first press to last option
            _ => {
                let val = *self as u8;
                let next_val = val.wrapping_sub(1);
                Self::from_u8(next_val as u8).unwrap_or(Self::None)
            }
        }
    }
    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::Brightness,

            // Self::Brightness => Self::Volume,
            // Self::Volume => Self::Time,
            // Self::Time => Self::Reset,
            Self::Reset => Self::Brightness,
            _ => {
                let val = *self as u8;
                let next_val = val.wrapping_add(1);
                Self::from_u8(next_val as u8).unwrap_or(Self::None)
            }
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        if value <= Self::MAX_VALUE || value == SettingSelected::None as u8 {
            Some(unsafe { core::mem::transmute(value) })
        } else {
            None
        }
    }
}

impl From<SettingSelected> for u8 {
    fn from(item: SettingSelected) -> u8 {
        item as u8
    }
}
